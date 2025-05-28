use std::collections::HashSet;

use crate::helpers::dbus_type_to_rust;
use crate::helpers::dedup_trait_name;
use crate::helpers::to_snake_case;

use zbus_xml::Property;
use zbus_xml::PropertyAccess;

/// Generates Rust trait method(s) for a D-Bus property from XML.
///
/// This function takes a parsed D-Bus property and returns the Rust trait method(s) as a string,
/// suitable for inclusion in a client proxy trait. Depending on the property's access,
/// it will generate a getter, setter, or both, with appropriate Rust types and naming.
///
/// # Arguments
/// * `prop` - The D-Bus property (`zbus_xml::Property`) to generate code for.
/// * `used_names` - A mutable set of used property names, for deduplication.
///
/// # Returns
/// * `String` - The Rust trait method(s) for this property, including doc comments and the `#[zbus(property)]` attribute.
///
/// # Example
/// let prop: zbus_xml::Property = ...;
/// let mut used_names = HashSet::new();
/// let code = codegen_property(&prop, &mut used_names);
/// assert!(code.contains("fn my_property(&self) -> `zbus::Result<String>`;"));
pub fn codegen_property(prop: &Property, used_names: &mut HashSet<String>) -> String {
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
