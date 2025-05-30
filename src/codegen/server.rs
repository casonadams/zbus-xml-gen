use std::collections::HashSet;

use crate::codegen::{dbus_type_to_rust, escape_rust_keyword, to_snake_case};

use zbus_xml::{ArgDirection, Interface, Method, Node, Signal};

pub fn generate_server_traits_from_xml(xml: &str) -> String {
    let cursor = std::io::Cursor::new(xml);
    let node = Node::from_reader(cursor).expect("Failed to parse D-Bus XML");
    node.interfaces()
        .iter()
        .map(generate_server_impl)
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_server_impl(interface: &Interface) -> String {
    let iface_name = interface.name();
    let struct_name = interface_struct_name(interface);
    let trait_name = interface_trait_name(interface);
    let impl_struct = struct_name.to_string();
    let server_struct = format!("{}Server", struct_name);
    let builder_struct = format!("{}ServerBuilder", struct_name);

    let methods = interface.methods().iter().collect::<Vec<_>>();
    let method_names: Vec<_> = methods
        .iter()
        .map(|m| escape_rust_keyword(&to_snake_case(&m.name())))
        .collect();

    let signals = interface.signals().iter().collect::<Vec<_>>();
    let mut out = String::new();

    let legacy = strip_trailing_digits(&iface_name).to_lowercase();
    let default_object_path = format!("/{}", legacy.replace('.', "/"));
    let default_well_known_name = iface_name.clone();

    out.push_str(
        r#"use std::convert::TryFrom;
use std::sync::Arc;
use zbus::{interface, Connection, Result};
use zbus::object_server::SignalEmitter;
use zbus::names::WellKnownName;
use zbus::zvariant::ObjectPath;

"#,
    );

    out.push_str(&format!(r#"pub trait {}: Send + Sync {{"#, trait_name));
    for (method, name) in methods.iter().zip(method_names.iter()) {
        let args = render_in_args_with_attrs(method);
        let ret_ty = render_out_args(method);
        out.push_str(&format!(
            "\n  fn {}(&self{}) -> {};",
            name,
            if args.is_empty() {
                "".into()
            } else {
                format!(", {}", args)
            },
            ret_ty
        ));
    }
    out.push_str("\n}\n\n");

    out.push_str(&format!(
        r#"#[derive(Clone)]
pub struct {} {{
  inner: Arc<dyn {}>,
}}

impl {} {{
  pub fn new(inner: Arc<dyn {}>) -> Self {{
    Self {{ inner }}
  }}
}}

"#,
        impl_struct, trait_name, impl_struct, trait_name
    ));

    out.push_str(&format!(
        r#"#[interface(name = "{}")]
impl {} {{
"#,
        iface_name, impl_struct
    ));
    for (method, name) in methods.iter().zip(method_names.iter()) {
        out.push_str(&render_delegating_method(method, name));
    }
    for signal in &signals {
        out.push_str(&render_signal_emitter(signal));
    }
    out.push_str("}\n\n");

    out.push_str(&format!(
        r#"pub struct {} {{
  pub connection: Arc<Connection>,
}}

impl {} {{
"#,
        server_struct, server_struct
    ));
    for signal in &signals {
        let name = to_snake_case(&signal.name());
        let args = signal
            .args()
            .iter()
            .map(|arg| {
                let arg_name = escape_rust_keyword(&to_snake_case(arg.name().unwrap_or("arg")));
                let arg_ty = dbus_type_to_rust(&arg.ty().to_string());
                format!("{}: {}", arg_name, arg_ty)
            })
            .collect::<Vec<_>>()
            .join(", ");
        let arg_names = signal
            .args()
            .iter()
            .map(|arg| escape_rust_keyword(&to_snake_case(arg.name().unwrap_or("arg"))))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            r#"  pub async fn emit_{}(&self, {}) -> Result<()> {{
    let emitter = SignalEmitter::new(&self.connection, "{}")?;
    {}::{}(&emitter, {}).await
  }}

"#,
            name, args, default_object_path, impl_struct, name, arg_names
        ));
    }
    out.push_str("}\n");

    out.push_str(&format!(
        r#"
pub struct {} {{
  pub implementation: Arc<dyn {}>,
  pub object_path: String,
  pub well_known_name: String,
}}

impl {} {{
  pub fn new(implementation: Arc<dyn {}>) -> Self {{
    Self {{
      implementation,
      object_path: "{}".into(),
      well_known_name: "{}".into(),
    }}
  }}

  pub fn object_path(mut self, path: &str) -> Self {{
    self.object_path = path.to_string();
    self
  }}

  pub fn well_known_name(mut self, name: &str) -> Self {{
    self.well_known_name = name.to_string();
    self
  }}

  pub async fn build(self) -> Result<{}> {{
    let conn = Connection::session().await?;
    let obj_path = ObjectPath::try_from(self.object_path.clone())?;
    conn.object_server().at(obj_path, {}::new(self.implementation.clone())).await?;
    conn.request_name(WellKnownName::try_from(self.well_known_name.clone())?).await?;
    Ok({} {{
      connection: Arc::new(conn),
    }})
  }}
}}
"#,
        builder_struct,
        trait_name,
        builder_struct,
        trait_name,
        default_object_path,
        default_well_known_name,
        server_struct,
        impl_struct,
        server_struct
    ));

    out
}

fn interface_struct_name(interface: &Interface) -> String {
    interface
        .name()
        .rsplit('.')
        .next()
        .unwrap_or("Iface")
        .to_string()
}

fn interface_trait_name(interface: &Interface) -> String {
    format!("{}Handler", interface_struct_name(interface))
}

fn render_in_args_with_attrs(method: &Method) -> String {
    let mut used_arg_names = HashSet::new();
    method
        .args()
        .iter()
        .filter(|arg| matches!(arg.direction(), Some(ArgDirection::In)))
        .map(|arg| {
            let mut arg_name = arg
                .name()
                .map(to_snake_case)
                .unwrap_or_else(|| "arg".to_string());
            let attr = match arg_name.as_str() {
                "hdr" | "header" => "#[zbus(header)] ",
                "emitter" => "#[zbus(signal_emitter)] ",
                "server" | "object_server" => "#[zbus(object_server)] ",
                _ => "",
            };
            arg_name = escape_rust_keyword(&arg_name);
            let orig_name = arg_name.clone();
            let mut count = 2;
            while !used_arg_names.insert(arg_name.clone()) {
                arg_name = format!("{}_{}", orig_name, count);
                count += 1;
            }
            format!(
                "{}{}: {}",
                attr,
                arg_name,
                dbus_type_to_rust(&arg.ty().to_string())
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_delegating_method(method: &Method, name: &str) -> String {
    let args = render_in_args_with_attrs(method);
    let ret_ty = render_out_args(method);
    let call_args = method
        .args()
        .iter()
        .filter(|arg| matches!(arg.direction(), Some(ArgDirection::In)))
        .map(|arg| escape_rust_keyword(&to_snake_case(arg.name().unwrap_or("arg"))))
        .collect::<Vec<_>>()
        .join(", ");

    let mut out = String::new();
    if let Some(meta) = render_out_args_meta(method) {
        out.push_str(&format!("  {}\n", meta));
    }
    out.push_str(&format!(
        "  fn {}(&self{}) -> {} {{\n",
        name,
        if args.is_empty() {
            "".into()
        } else {
            format!(", {}", args)
        },
        ret_ty
    ));
    out.push_str(&format!("    self.inner.{}({})\n", name, call_args));
    out.push_str("  }\n\n");
    out
}

fn render_out_args_meta(method: &Method) -> Option<String> {
    let out_args: Vec<_> = method
        .args()
        .iter()
        .filter(|arg| matches!(arg.direction(), Some(ArgDirection::Out)))
        .collect();
    if out_args.len() > 1 {
        let names = out_args
            .iter()
            .map(|arg| format!("\"{}\"", arg.name().unwrap_or("out")))
            .collect::<Vec<_>>();
        Some(format!("#[zbus(out_args({}))]", names.join(", ")))
    } else {
        None
    }
}

fn render_out_args(method: &Method) -> String {
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

fn render_signal_emitter(signal: &Signal) -> String {
    let fn_name = to_snake_case(&signal.name());
    let args: Vec<_> = signal
        .args()
        .iter()
        .map(|arg| {
            let name = escape_rust_keyword(&to_snake_case(arg.name().unwrap_or("arg")));
            let ty = dbus_type_to_rust(&arg.ty().to_string());
            format!("{}: {}", name, ty)
        })
        .collect();

    let param_names: Vec<_> = signal
        .args()
        .iter()
        .map(|arg| escape_rust_keyword(&to_snake_case(arg.name().unwrap_or("arg"))))
        .collect();

    let tuple = match param_names.len() {
        0 => "()".to_string(),
        1 => format!("({},)", param_names[0]),
        _ => format!("({})", param_names.join(", ")),
    };

    format!(
        r#" #[zbus(signal)]
  pub async fn {}(emitter: &SignalEmitter<'_>{}) -> Result<()> {{
    emitter.emit_signal("{}", &{})
  }}
"#,
        fn_name,
        if args.is_empty() {
            "".into()
        } else {
            format!(", {}", args.join(", "))
        },
        signal.name(),
        tuple
    )
}

fn strip_trailing_digits(s: &str) -> &str {
    s.trim_end_matches(|c: char| c.is_ascii_digit())
}
