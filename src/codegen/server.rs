use std::collections::HashSet;

use crate::helpers::dbus_type_to_rust;
use crate::helpers::dedup_and_escape;
use crate::helpers::escape_rust_keyword;
use crate::helpers::to_snake_case;

use zbus_xml::Annotation;
use zbus_xml::ArgDirection;
use zbus_xml::Interface;
use zbus_xml::Node;
use zbus_xml::Property;
use zbus_xml::PropertyAccess;

/// Generates Rust trait code for server implementations from D-Bus introspection XML.
///
/// This function takes D-Bus introspection XML (as a string slice),
/// parses it, and produces code for Rust server traits representing all interfaces.
/// Each trait includes all methods and properties, with D-Bus annotations rendered
/// as Rust doc comments.
///
/// # Arguments
/// * `xml` - D-Bus introspection XML as a string slice.
///
/// # Returns
/// * `String` - The generated Rust code for all server traits.
///
/// # Panics
/// Panics if the XML is invalid or parsing fails.
///
/// # Example
/// ```
/// use zbus_xml_gen::generate_server_traits_from_xml;
/// let xml = r#"
/// <node>
///   <interface name="org.example.MyIface">
///     <method name="Foo"><arg name="x" type="i" direction="in"/></method>
///   </interface>
/// </node>
/// "#;
/// let code = generate_server_traits_from_xml(xml);
/// assert!(code.contains("pub trait MyIfaceServer"));
/// assert!(code.contains("fn foo(&mut self, x: i32) -> zbus::Result<()>;"));
/// ```
pub fn generate_server_traits_from_xml(xml: &str) -> String {
    let cursor = std::io::Cursor::new(xml);
    let node = Node::from_reader(cursor).expect("Failed to parse D-Bus XML");
    node.interfaces()
        .iter()
        .map(|iface| generate_server_impl(iface))
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_server_impl(interface: &Interface) -> String {
    let mut code = render_trait_header(interface);
    let mut used_names = HashSet::new();
    code.push_str(&render_methods(interface, &mut used_names));
    code.push_str(&render_properties(interface, &mut used_names));
    code.push_str("}\n");
    code
}

fn render_trait_header(interface: &Interface) -> String {
    let mut out = String::new();
    if !interface.annotations().is_empty() {
        out.push_str(&annotation_docs(interface.annotations(), ""));
        out.push('\n');
    }
    out.push_str(&format!("pub trait {}Server {{\n", trait_name(interface)));
    out
}

fn trait_name(interface: &Interface) -> String {
    interface
        .name()
        .rsplit('.')
        .next()
        .unwrap_or("Iface")
        .to_string()
}

fn render_methods(interface: &Interface, used_names: &mut HashSet<String>) -> String {
    interface
        .methods()
        .iter()
        .map(|method| render_method_signature(method, used_names))
        .collect::<String>()
}

fn render_method_signature(method: &zbus_xml::Method, used_names: &mut HashSet<String>) -> String {
    let mut out = String::new();

    out.push_str(&render_method_annotations(method));
    out.push_str(&render_args_annotations(method));
    let rust_method = method_trait_name(method, used_names);
    let args = render_in_args(method);
    let ret_ty = render_out_args(method);

    out.push_str(&format_method_signature(&rust_method, &args, &ret_ty));
    out
}

fn render_method_annotations(method: &zbus_xml::Method) -> String {
    if !method.annotations().is_empty() {
        let docs = annotation_docs(method.annotations(), "    ");
        format!("{}\n", docs)
    } else {
        String::new()
    }
}

fn render_args_annotations(method: &zbus_xml::Method) -> String {
    method
        .args()
        .iter()
        .flat_map(|arg| {
            arg.annotations().iter().map(move |ann| {
                format!(
                    "    /// [arg: {}] [annotation] {} = \"{}\"\n",
                    arg.name().unwrap_or("unnamed"),
                    ann.name(),
                    ann.value().replace('\n', "\\n")
                )
            })
        })
        .collect()
}

fn method_trait_name(method: &zbus_xml::Method, used_names: &mut HashSet<String>) -> String {
    dedup_and_escape(
        &escape_rust_keyword(&to_snake_case(&method.name())),
        used_names,
    )
}

fn render_in_args(method: &zbus_xml::Method) -> String {
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
            arg_name = escape_rust_keyword(&arg_name);
            let orig_name = arg_name.clone();
            let mut count = 2;
            while !used_arg_names.insert(arg_name.clone()) {
                arg_name = format!("{}_{}", orig_name, count);
                count += 1;
            }
            format!("{}: {}", arg_name, dbus_type_to_rust(&arg.ty().to_string()))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_out_args(method: &zbus_xml::Method) -> String {
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

fn format_method_signature(rust_method: &str, args: &str, ret_ty: &str) -> String {
    if args.is_empty() {
        format!(
            "    fn {}(&mut self) -> zbus::Result<{}>;\n\n",
            rust_method, ret_ty
        )
    } else {
        format!(
            "    fn {}(&mut self, {}) -> zbus::Result<{}>;\n\n",
            rust_method, args, ret_ty
        )
    }
}

fn render_properties(interface: &Interface, used_names: &mut HashSet<String>) -> String {
    interface
        .properties()
        .iter()
        .map(|prop| render_property_signature(prop, used_names))
        .collect::<String>()
}

fn render_property_signature(prop: &Property, used_names: &mut HashSet<String>) -> String {
    let mut out = String::new();
    out.push_str(&render_property_doc(prop));
    out.push_str(&render_property_annotations(prop));
    let (get_name, set_name) = property_fn_names(prop, used_names);
    let rust_type = dbus_type_to_rust(&prop.ty().to_string());
    out.push_str(&render_property_getter(prop, &get_name, &rust_type));
    out.push_str(&render_property_setter(
        prop, &set_name, &rust_type, used_names,
    ));
    out.push('\n');
    out
}

fn render_property_doc(prop: &Property) -> String {
    format!("    /// property: {}\n", prop.name())
}

fn render_property_annotations(prop: &Property) -> String {
    if !prop.annotations().is_empty() {
        format!("{}\n", annotation_docs(prop.annotations(), "    "))
    } else {
        String::new()
    }
}

fn property_fn_names(prop: &Property, used_names: &mut HashSet<String>) -> (String, String) {
    let base = escape_rust_keyword(&to_snake_case(&prop.name()));
    let mut get_name = format!("get_{}", base);
    if used_names.contains(&get_name) {
        get_name = format!("get_{}_prop", base);
    }
    used_names.insert(get_name.clone());

    let mut set_name = format!("set_{}", base);
    if used_names.contains(&set_name) {
        set_name = format!("set_{}_prop", base);
    }
    (get_name, set_name)
}

fn render_property_getter(prop: &Property, get_name: &str, rust_type: &str) -> String {
    match prop.access() {
        PropertyAccess::Read | PropertyAccess::ReadWrite => {
            format!(
                "    fn {}(&self) -> zbus::Result<{}>;\n",
                get_name, rust_type
            )
        }
        _ => String::new(),
    }
}

fn render_property_setter(
    prop: &Property,
    set_name: &str,
    rust_type: &str,
    used_names: &mut HashSet<String>,
) -> String {
    match prop.access() {
        PropertyAccess::Write | PropertyAccess::ReadWrite => {
            used_names.insert(set_name.to_string());
            format!(
                "    fn {}(&mut self, value: {}) -> zbus::Result<()>;\n",
                set_name, rust_type
            )
        }
        _ => String::new(),
    }
}

fn annotation_docs(anns: &[Annotation], indent: &str) -> String {
    anns.iter()
        .map(|ann| {
            let value = ann.value().replace('\n', "\\n");
            format!("{}/// [annotation] {} = \"{}\"", indent, ann.name(), value)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
