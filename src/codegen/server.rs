use std::fmt::Write;
use zbus_xml::{ArgDirection, Interface, Method, Node, Property, Signal};

use crate::codegen::{dbus_type_to_rust, escape_rust_keyword, to_snake_case};

pub fn generate_server_interface_from_xml(xml: &str) -> String {
    let node = Node::from_reader(std::io::Cursor::new(xml)).expect("Failed to parse D-Bus XML");

    node.interfaces()
        .iter()
        .map(generate_interface_block)
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_interface_block(interface: &Interface) -> String {
    let iface_name = interface.name();
    let struct_name = iface_name.rsplit('.').next().unwrap_or("Iface");
    let trait_name = format!("{}Delegate", struct_name);

    let mut out = String::new();

    writeln!(out, "use std::sync::Arc;").unwrap();
    writeln!(out, "use async_trait::async_trait;").unwrap();
    writeln!(out, "use zbus::{{interface, Result}};").unwrap();
    writeln!(out, "use zbus::object_server::SignalEmitter;\n").unwrap();

    // Trait
    writeln!(out, "#[async_trait]").unwrap();
    writeln!(out, "pub trait {}: Send + Sync + 'static {{", trait_name).unwrap();
    for method in interface.methods() {
        writeln!(out, "{}", generate_trait_method(method)).unwrap();
    }
    for prop in interface.properties() {
        write!(out, "{}", generate_trait_property(prop)).unwrap();
    }
    writeln!(out, "}}\n").unwrap();

    // Struct
    writeln!(out, "#[derive(Clone)]").unwrap();
    writeln!(out, "pub struct {} {{", struct_name).unwrap();
    writeln!(out, "    pub delegate: Arc<dyn {}>,", trait_name).unwrap();
    writeln!(out, "}}\n").unwrap();

    writeln!(out, "impl {} {{", struct_name).unwrap();
    writeln!(
        out,
        "    pub fn new(delegate: Arc<dyn {}>) -> Self {{",
        trait_name
    )
    .unwrap();
    writeln!(out, "        Self {{ delegate }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}\n").unwrap();

    // Interface impl
    writeln!(out, "#[interface(name = \"{}\")]", iface_name).unwrap();
    writeln!(out, "impl {} {{", struct_name).unwrap();
    for method in interface.methods() {
        writeln!(out, "{}", generate_delegate_method(method)).unwrap();
    }
    for prop in interface.properties() {
        writeln!(out, "{}", generate_delegate_property(prop)).unwrap();
    }
    for signal in interface.signals() {
        writeln!(out, "{}", generate_signal_signature(signal)).unwrap();
    }
    writeln!(out, "}}").unwrap();

    out
}

fn generate_trait_method(method: &Method) -> String {
    let name = escape_rust_keyword(&to_snake_case(&method.name()));
    let args = method_args(method);
    let ret = method_return_type(method);
    format!("    async fn {}(&self{}) -> {};", name, args, ret)
}

fn generate_trait_property(prop: &Property) -> String {
    let name = escape_rust_keyword(&to_snake_case(&prop.name()));
    let ty = dbus_type_to_rust(&prop.ty().to_string());
    let mut out = String::new();

    if prop.access().read() {
        writeln!(&mut out, "    async fn {}(&self) -> {};", name, ty).unwrap();
    }
    if prop.access().write() {
        writeln!(
            &mut out,
            "    async fn set_{}(&self, val: {}) -> Result<()>;",
            name, ty
        )
        .unwrap();
    }

    out
}

fn generate_delegate_method(method: &Method) -> String {
    let name = escape_rust_keyword(&to_snake_case(&method.name()));
    let args = method_args(method);
    let ret = method_return_type(method);
    let call_args = method_arg_names(method);

    format!(
        "    async fn {}(&self{}) -> {} {{\n        self.delegate.{}({}).await\n    }}\n",
        name, args, ret, name, call_args
    )
}

fn generate_delegate_property(prop: &Property) -> String {
    let name = escape_rust_keyword(&to_snake_case(&prop.name()));
    let ty = dbus_type_to_rust(&prop.ty().to_string());
    let mut out = String::new();

    if prop.access().read() {
        writeln!(
            &mut out,
            "    #[zbus(property)]\n    async fn {}(&self) -> {} {{\n        self.delegate.{}().await\n    }}",
            name, ty, name
        )
        .unwrap();
    }
    if prop.access().write() {
        writeln!(
            &mut out,
            "    #[zbus(property)]\n    async fn set_{}(&mut self, val: {}) {{\n        let _ = self.delegate.set_{}(val).await;\n    }}",
            name, ty, name
        )
        .unwrap();
    }

    out
}

fn generate_signal_signature(signal: &Signal) -> String {
    let name = to_snake_case(&signal.name());
    let args: Vec<_> = signal
        .args()
        .iter()
        .map(|arg| {
            let name = escape_rust_keyword(&to_snake_case(arg.name().unwrap_or("arg")));
            let ty = dbus_type_to_rust(&arg.ty().to_string());
            format!("{}: {}", name, ty)
        })
        .collect();

    let param_list = if args.is_empty() {
        "emitter: SignalEmitter<'_>".to_string()
    } else {
        format!("emitter: SignalEmitter<'_>, {}", args.join(", "))
    };

    format!(
        "    #[zbus(signal)]\n    async fn {}({}) -> Result<()>;",
        name, param_list
    )
}

fn method_args(method: &Method) -> String {
    let args: Vec<_> = method
        .args()
        .iter()
        .filter(|a| a.direction() == Some(ArgDirection::In))
        .map(|a| {
            let name = escape_rust_keyword(&to_snake_case(a.name().unwrap_or("arg")));
            let ty = dbus_type_to_rust(&a.ty().to_string());
            format!("{}: {}", name, ty)
        })
        .collect();

    if args.is_empty() {
        "".into()
    } else {
        format!(", {}", args.join(", "))
    }
}

fn method_arg_names(method: &Method) -> String {
    method
        .args()
        .iter()
        .filter(|a| a.direction() == Some(ArgDirection::In))
        .map(|a| escape_rust_keyword(&to_snake_case(a.name().unwrap_or("arg"))))
        .collect::<Vec<_>>()
        .join(", ")
}

fn method_return_type(method: &Method) -> String {
    let out_args: Vec<_> = method
        .args()
        .iter()
        .filter(|a| a.direction() == Some(ArgDirection::Out))
        .collect();

    match out_args.len() {
        0 => "zbus::fdo::Result<()>".into(),
        1 => format!(
            "zbus::fdo::Result<{}>",
            dbus_type_to_rust(&out_args[0].ty().to_string())
        ),
        _ => format!(
            "zbus::fdo::Result<({})>",
            out_args
                .iter()
                .map(|a| dbus_type_to_rust(&a.ty().to_string()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}
