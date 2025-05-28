use crate::helpers::{
    dbus_type_to_rust, dedup_and_escape, dedup_trait_name, escape_rust_keyword, to_snake_case,
};
use std::collections::HashSet;
use zbus_xml::{Annotation, ArgDirection, Interface, Node, Property, PropertyAccess};

pub fn parse_node_from_file(xml_path: &str) -> Node {
    let xml = std::fs::read_to_string(xml_path).expect("XML file not found");
    let cursor = std::io::Cursor::new(xml);
    Node::from_reader(cursor).expect("Failed to parse XML")
}

// Helper: emit annotation as doc comments
fn annotation_docs(anns: &[Annotation], indent: &str) -> String {
    anns.iter()
        .map(|ann| {
            let value = ann.value().replace('\n', "\\n");
            format!("{}/// [annotation] {} = \"{}\"", indent, ann.name(), value)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// -- CLIENT CODEGEN (no annotations) --

fn codegen_method(method: &zbus_xml::Method, used_names: &mut HashSet<String>) -> String {
    // Escape and dedup the method name
    let base_name = to_snake_case(&method.name());
    let rust_method = dedup_trait_name(&base_name, used_names, false);

    // Deduplicate arg names
    let mut used_arg_names = HashSet::new();
    let args = method
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
        .join(", ");

    // Output args
    let out_args: Vec<_> = method
        .args()
        .iter()
        .filter(|arg| matches!(arg.direction(), Some(ArgDirection::Out)))
        .collect();

    let ret_ty = match out_args.len() {
        0 => "()".to_string(),
        1 => dbus_type_to_rust(&out_args[0].ty().to_string()),
        _ => {
            let types = out_args
                .iter()
                .map(|arg| dbus_type_to_rust(&arg.ty().to_string()))
                .collect::<Vec<_>>();
            format!("({})", types.join(", "))
        }
    };

    let needs_zbus_name_attr = rust_method != escape_rust_keyword(&to_snake_case(&method.name()));

    let mut s = String::new();
    s.push_str(&format!("  /// {} method\n", method.name()));
    if needs_zbus_name_attr {
        s.push_str(&format!("  #[zbus(name = \"{}\")]\n", method.name()));
    }
    if args.is_empty() {
        s.push_str(&format!(
            "  fn {}(&self) -> zbus::Result<{}>;\n\n",
            rust_method, ret_ty
        ));
    } else {
        s.push_str(&format!(
            "  fn {}(&self, {}) -> zbus::Result<{}>;\n\n",
            rust_method, args, ret_ty
        ));
    }
    s
}

fn codegen_property(prop: &Property, used_names: &mut HashSet<String>) -> String {
    let base_name = to_snake_case(&prop.name());
    let rust_name = dedup_trait_name(&base_name, used_names, true);
    let rust_type = dbus_type_to_rust(&prop.ty().to_string());
    let mut code = String::new();

    match prop.access() {
        PropertyAccess::Read => {
            code.push_str(&format!(
                "  /// {} property\n  #[zbus(property)]\n  fn {}(&self) -> zbus::Result<{}>;\n\n",
                prop.name(),
                rust_name,
                rust_type
            ));
        }
        PropertyAccess::Write => {
            code.push_str(&format!(
                "  /// Set the {} property\n  #[zbus(property)]\n  fn set_{}(&self, value: {}) -> zbus::Result<()>;\n\n",
                prop.name(), rust_name, rust_type
            ));
        }
        PropertyAccess::ReadWrite => {
            code.push_str(&format!(
                "  /// {} property\n  #[zbus(property)]\n  fn {}(&self) -> zbus::Result<{}>;\n\n",
                prop.name(),
                rust_name,
                rust_type
            ));
            code.push_str(&format!(
                "  /// Set the {} property\n  #[zbus(property)]\n  fn set_{}(&self, value: {}) -> zbus::Result<()>;\n\n",
                prop.name(), rust_name, rust_type
            ));
        }
    }
    code
}

pub fn generate_client_proxy(interface: &Interface) -> String {
    let mut code = String::new();
    code.push_str("use zbus::proxy;\n");
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

// -- SERVER CODEGEN WITH ANNOTATION DOCS --

pub fn generate_server_impl(interface: &Interface) -> String {
    let mut code = String::new();

    if !interface.annotations().is_empty() {
        code.push_str(&annotation_docs(interface.annotations(), ""));
        code.push('\n');
    }

    let binding = interface.name();
    let trait_name = binding.rsplit('.').next().unwrap_or("Iface");
    code.push_str(&format!("pub trait {}Server {{\n", trait_name));

    let mut used_names = HashSet::new();

    // Methods
    for method in interface.methods() {
        if !method.annotations().is_empty() {
            code.push_str(&annotation_docs(method.annotations(), "    "));
            code.push('\n');
        }
        for arg in method.args() {
            if !arg.annotations().is_empty() {
                for ann in arg.annotations() {
                    code.push_str(&format!(
                        "    /// [arg: {}] [annotation] {} = \"{}\"\n",
                        arg.name().unwrap_or("unnamed"),
                        ann.name(),
                        ann.value().replace('\n', "\\n")
                    ));
                }
            }
        }

        let rust_method = dedup_and_escape(
            &escape_rust_keyword(&to_snake_case(&method.name())),
            &mut used_names,
        );

        let mut used_arg_names = HashSet::new();
        let args = method
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
            .join(", ");

        let out_args: Vec<_> = method
            .args()
            .iter()
            .filter(|arg| matches!(arg.direction(), Some(ArgDirection::Out)))
            .collect();
        let ret_ty = match out_args.len() {
            0 => "()".to_string(),
            1 => dbus_type_to_rust(&out_args[0].ty().to_string()),
            _ => {
                let types = out_args
                    .iter()
                    .map(|arg| dbus_type_to_rust(&arg.ty().to_string()))
                    .collect::<Vec<_>>();
                format!("({})", types.join(", "))
            }
        };

        let fn_decl = if args.is_empty() {
            format!("fn {}(&mut self) -> zbus::Result<{}>;", rust_method, ret_ty)
        } else {
            format!(
                "fn {}(&mut self, {}) -> zbus::Result<{}>;",
                rust_method, args, ret_ty
            )
        };
        code.push_str(&format!("    {}\n\n", fn_decl));
    }

    // Properties
    for prop in interface.properties() {
        code.push_str(&format!("    /// property: {}\n", prop.name()));
        if !prop.annotations().is_empty() {
            code.push_str(&annotation_docs(prop.annotations(), "    "));
            code.push('\n');
        }

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
        // Only insert if used
        let rust_type = dbus_type_to_rust(&prop.ty().to_string());

        match prop.access() {
            PropertyAccess::Read | PropertyAccess::ReadWrite => {
                code.push_str(&format!(
                    "    fn {}(&self) -> zbus::Result<{}>;\n",
                    get_name, rust_type
                ));
            }
            _ => {}
        }
        match prop.access() {
            PropertyAccess::Write | PropertyAccess::ReadWrite => {
                used_names.insert(set_name.clone());
                code.push_str(&format!(
                    "    fn {}(&mut self, value: {}) -> zbus::Result<()>;\n",
                    set_name, rust_type
                ));
            }
            _ => {}
        }
        code.push('\n');
    }

    code.push_str("}\n");
    code
}
