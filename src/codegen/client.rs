use std::collections::HashSet;

use crate::codegen::dbus_type_to_rust;
use crate::codegen::dedup_trait_name;
use crate::codegen::escape_rust_keyword;
use crate::codegen::to_snake_case;

use zbus_xml::{ArgDirection, Interface, Method, Node, Property, PropertyAccess};

pub fn generate_client_proxies_from_xml(xml: &str) -> String {
    let cursor = std::io::Cursor::new(xml);
    let node = Node::from_reader(cursor).expect("Failed to parse D-Bus XML");
    let mut code = String::new();

    code.push_str(
        r#"use zbus::proxy;
use zbus::Result;

"#,
    );

    for iface in node.interfaces() {
        code.push_str(&generate_client_proxy(iface));
    }

    code
}

fn generate_client_proxy(interface: &Interface) -> String {
    let mut code = String::new();

    let iface_name = interface.name();
    code.push_str(&format!(
        r#"#[proxy(interface = "{}" , assume_defaults = true)]
"#,
        iface_name
    ));

    let trait_name = iface_name.rsplit('.').next().unwrap_or("Iface");
    code.push_str(&format!("pub trait {} {{\n", trait_name));

    let mut used_names = HashSet::new();
    for method in interface.methods() {
        code.push_str(&codegen_method(method, &mut used_names));
    }
    for prop in interface.properties() {
        code.push_str(&codegen_property(prop, &mut used_names));
    }
    for signal in interface.signals() {
        code.push_str(&codegen_signal(signal, &mut used_names));
    }

    code.push_str("}\n");
    code
}

fn codegen_signal(signal: &zbus_xml::Signal, used_names: &mut HashSet<String>) -> String {
    let rust_name = dedup_trait_name(&to_snake_case(&signal.name()), used_names, false);
    let types: Vec<_> = signal
        .args()
        .iter()
        .map(|arg| dbus_type_to_rust(&arg.ty().to_string()))
        .collect();

    let stream_type = match types.len() {
        0 => "()".to_string(),
        1 => types[0].clone(),
        _ => format!("({})", types.join(", ")),
    };

    format!(
        r#"  #[zbus(signal)]
  fn {rust_name}(&self) -> zbus::Result<zbus::SignalStream<{stream_type}>>;

"#,
    )
}

fn codegen_method(method: &Method, used_names: &mut HashSet<String>) -> String {
    let rust_method = render_rust_method_name(method, used_names);
    let args = render_method_args(method);
    let ret_ty = render_method_return_type(method);
    let needs_zbus_name_attr = method_needs_zbus_name_attr(method, &rust_method);

    let mut s = String::new();
    if needs_zbus_name_attr {
        s.push_str(&render_zbus_name_attr(method));
    }
    s.push_str(&render_method_signature(&rust_method, &args, &ret_ty));
    s
}

fn render_rust_method_name(method: &Method, used_names: &mut HashSet<String>) -> String {
    let base_name = to_snake_case(&method.name());
    dedup_trait_name(&base_name, used_names, false)
}

fn render_method_args(method: &Method) -> String {
    let mut used_arg_names = HashSet::new();
    method
        .args()
        .iter()
        .filter(|arg| matches!(arg.direction(), Some(ArgDirection::In)))
        .map(|arg| render_arg(arg, &mut used_arg_names))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_arg(arg: &zbus_xml::Arg, used_arg_names: &mut HashSet<String>) -> String {
    let mut arg_name = arg
        .name()
        .map(to_snake_case)
        .unwrap_or_else(|| "arg".to_string());
    arg_name = escape_rust_keyword(&arg_name);
    let orig_name = arg_name.clone();
    let mut count = 2;
    while !used_arg_names.insert(arg_name.clone()) {
        arg_name = format!("{}_{}", orig_name, count);
        count += 1;
    }
    format!("{}: {}", arg_name, dbus_type_to_rust(&arg.ty().to_string()))
}

fn render_method_return_type(method: &Method) -> String {
    let out_args: Vec<_> = method
        .args()
        .iter()
        .filter(|arg| matches!(arg.direction(), Some(ArgDirection::Out)))
        .collect();
    match out_args.len() {
        0 => "()".to_string(),
        1 => dbus_type_to_rust(&out_args[0].ty().to_string()),
        _ => {
            let types = out_args
                .iter()
                .map(|arg| dbus_type_to_rust(&arg.ty().to_string()))
                .collect::<Vec<_>>();
            format!("({})", types.join(", "))
        }
    }
}

fn method_needs_zbus_name_attr(method: &Method, rust_method: &str) -> bool {
    rust_method != escape_rust_keyword(&to_snake_case(&method.name()))
}

fn render_zbus_name_attr(method: &Method) -> String {
    format!(r#"  #[zbus(name = "{}")]\n"#, method.name())
}

fn render_method_signature(rust_method: &str, args: &str, ret_ty: &str) -> String {
    if args.is_empty() {
        format!(
            r#"  fn {}(&self) -> zbus::Result<{}>;

"#,
            rust_method, ret_ty
        )
    } else {
        format!(
            r#"  fn {}(&self, {}) -> zbus::Result<{}>;

"#,
            rust_method, args, ret_ty
        )
    }
}

fn codegen_property(prop: &Property, used_names: &mut HashSet<String>) -> String {
    let rust_name = render_property_rust_name(prop, used_names);
    let rust_type = render_property_type(prop);
    match prop.access() {
        PropertyAccess::Read => render_property_getter(&rust_name, &rust_type),
        PropertyAccess::Write => render_property_setter(&rust_name, &rust_type),
        PropertyAccess::ReadWrite => {
            let get = render_property_getter(&rust_name, &rust_type);
            let set = render_property_setter(&rust_name, &rust_type);
            format!("{}{}", get, set)
        }
    }
}

fn render_property_rust_name(prop: &Property, used_names: &mut HashSet<String>) -> String {
    let base_name = to_snake_case(&prop.name());
    dedup_trait_name(&base_name, used_names, true)
}

fn render_property_type(prop: &Property) -> String {
    dbus_type_to_rust(&prop.ty().to_string())
}

fn render_property_getter(rust_name: &str, rust_type: &str) -> String {
    format!(
        r#"  #[zbus(property)]
  fn {}(&self) -> zbus::Result<{}>;

"#,
        rust_name, rust_type
    )
}

fn render_property_setter(rust_name: &str, rust_type: &str) -> String {
    format!(
        r#"  #[zbus(property)]
  fn set_{}(&self, value: {}) -> zbus::Result<()>;

"#,
        rust_name, rust_type
    )
}
