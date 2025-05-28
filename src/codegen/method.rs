use std::collections::HashSet;

use crate::helpers::dbus_type_to_rust;
use crate::helpers::dedup_trait_name;
use crate::helpers::escape_rust_keyword;
use crate::helpers::to_snake_case;

use zbus_xml::ArgDirection;
use zbus_xml::Method;

/// Generates a Rust trait method definition for a D-Bus method from XML.
///
/// This function takes a `zbus_xml::Method` and produces a Rust trait method signature as a string,
/// suitable for inclusion in a trait (client or server). It:
/// - Converts the D-Bus method name to a Rust-friendly, deduplicated identifier
/// - Generates Rust argument names and types for D-Bus "in" arguments, deduplicating names as needed
/// - Determines the Rust return type from the D-Bus "out" arguments (as a tuple if multiple)
/// - Optionally adds the `#[zbus(name = "...")]` attribute if the Rust name differs from the D-Bus name
///
/// # Arguments
/// * `method` - The D-Bus method definition parsed from XML
/// * `used_names` - A set of already-used method names, for deduplication
///
/// # Returns
/// * `String` - The Rust trait method code for this method
///
/// # Example
/// let method: zbus_xml::Method = ...;
/// let mut used_names = HashSet::new();
/// let sig = codegen_method(&method, &mut used_names);
/// assert!(sig.contains("fn my_method_name"));
pub fn codegen_method(method: &Method, used_names: &mut HashSet<String>) -> String {
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
