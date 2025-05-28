use std::collections::HashSet;

use crate::codegen::dbus_type_to_rust;
use crate::codegen::dedup_trait_name;
use crate::codegen::escape_rust_keyword;
use crate::codegen::to_snake_case;

use zbus_xml::ArgDirection;
use zbus_xml::Method;
use zbus_xml::Node;
use zbus_xml::Property;
use zbus_xml::PropertyAccess;

/// Generates Rust trait code for a client proxy to a D-Bus interface from D-Bus introspection XML.
///
/// This function takes a D-Bus introspection XML (as a string slice),
/// parses it, and produces the code for Rust traits representing all interfaces.
/// Returns a string with all the generated code (one trait per interface).
///
/// # Arguments
/// * `xml` - D-Bus introspection XML as a string slice.
///
/// # Returns
/// * `String` - The generated Rust code for all client proxy traits.
///
/// # Panics
/// Panics if the XML is invalid or parsing fails.
///
/// # Example
/// ```
/// use zbus_xml_gen::generate_client_proxies_from_xml;
/// let xml = r#"
/// <node>
///   <interface name="org.example.Foo">
///     <method name="Bar"><arg name="x" type="i" direction="in"/></method>
///     <property name="status" type="s" access="read"/>
///   </interface>
/// </node>
/// "#;
/// let code = generate_client_proxies_from_xml(xml);
/// assert!(code.contains("#[proxy(interface = \"org.example.Foo\""));
/// assert!(code.contains("pub trait Foo {"));
/// assert!(code.contains("fn bar(&self, x: i32) -> zbus::Result<()>;"));
/// assert!(code.contains("fn status(&self) -> zbus::Result<String>;"));
/// ```
pub fn generate_client_proxies_from_xml(xml: &str) -> String {
    let cursor = std::io::Cursor::new(xml);
    let node = Node::from_reader(cursor).expect("Failed to parse D-Bus XML");
    let mut code = String::new();
    code.push_str("use zbus::proxy;\n");
    code.push_str(
        &node
            .interfaces()
            .iter()
            .map(|iface| generate_client_proxy(iface))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    code
}

fn generate_client_proxy(interface: &zbus_xml::Interface) -> String {
    let mut code = String::new();
    code.push_str(&format!(
        "#[proxy(interface = \"{}\", assume_defaults = true)]\n",
        interface.name()
    ));

    let binding = interface.name();
    let trait_name = binding.rsplit('.').next().unwrap_or("Iface");
    code.push_str(&format!("pub trait {} {{\n", trait_name));

    let mut used_names = HashSet::new();
    for method in interface.methods() {
        code.push_str(&codegen_method(method, &mut used_names));
    }
    for prop in interface.properties() {
        code.push_str(&codegen_property(prop, &mut used_names));
    }

    code.push_str("}\n");
    code
}

fn codegen_method(method: &Method, used_names: &mut HashSet<String>) -> String {
    let rust_method = render_rust_method_name(method, used_names);
    let args = render_method_args(method);
    let ret_ty = render_method_return_type(method);
    let needs_zbus_name_attr = method_needs_zbus_name_attr(method, &rust_method);

    let mut s = String::new();
    s.push_str(&render_method_doc(method));
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

fn render_method_doc(method: &Method) -> String {
    format!("  /// {} method\n", method.name())
}

fn render_zbus_name_attr(method: &Method) -> String {
    format!("  #[zbus(name = \"{}\")]\n", method.name())
}

fn render_method_signature(rust_method: &str, args: &str, ret_ty: &str) -> String {
    if args.is_empty() {
        format!(
            "  fn {}(&self) -> zbus::Result<{}>;\n\n",
            rust_method, ret_ty
        )
    } else {
        format!(
            "  fn {}(&self, {}) -> zbus::Result<{}>;\n\n",
            rust_method, args, ret_ty
        )
    }
}

fn codegen_property(prop: &Property, used_names: &mut HashSet<String>) -> String {
    let rust_name = render_property_rust_name(prop, used_names);
    let rust_type = render_property_type(prop);
    match prop.access() {
        PropertyAccess::Read => render_property_getter(prop, &rust_name, &rust_type),
        PropertyAccess::Write => render_property_setter(prop, &rust_name, &rust_type),
        PropertyAccess::ReadWrite => {
            let get = render_property_getter(prop, &rust_name, &rust_type);
            let set = render_property_setter(prop, &rust_name, &rust_type);
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

fn render_property_getter(prop: &Property, rust_name: &str, rust_type: &str) -> String {
    format!(
        "  /// {} property\n  #[zbus(property)]\n  fn {}(&self) -> zbus::Result<{}>;\n\n",
        prop.name(),
        rust_name,
        rust_type
    )
}

fn render_property_setter(prop: &Property, rust_name: &str, rust_type: &str) -> String {
    format!(
        "  /// Set the {} property\n  #[zbus(property)]\n  fn set_{}(&self, value: {}) -> zbus::Result<()>;\n\n",
        prop.name(),
        rust_name,
        rust_type
    )
}
