use crate::codegen::method::codegen_method;
use crate::codegen::property::codegen_property;
use std::collections::HashSet;
use zbus_xml::Node;

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
