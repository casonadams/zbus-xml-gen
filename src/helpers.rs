use heck::ToSnakeCase;
use std::collections::HashSet;

pub fn to_snake_case(name: &str) -> String {
    name.to_snake_case()
}

pub fn dbus_type_to_rust(ty: &str) -> String {
    match ty {
        "s" => "String".to_string(),
        "u" => "u32".to_string(),
        "i" => "i32".to_string(),
        "b" => "bool".to_string(),
        "ay" => "Vec<u8>".to_string(),
        _ => "String".to_string(),
    }
}

pub fn escape_rust_keyword(ident: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "type", "match", "ref", "mut", "const", "fn", "mod", "pub", "self", "super", "as", "trait",
        "struct", "enum", "impl", "use", "where", "loop", "move", "static", "async", "await",
        "dyn", "crate", "abstract", "final", "macro", "try", "union", "box", "continue", "else",
        "extern", "false", "for", "if", "in", "let", "return", "true", "unsafe", "while",
    ];
    if KEYWORDS.contains(&ident) {
        format!("{}_", ident)
    } else {
        ident.to_string()
    }
}

pub fn dedup_and_escape(name: &str, used: &mut HashSet<String>) -> String {
    let ident = escape_rust_keyword(name);
    if !used.contains(&ident) {
        used.insert(ident.clone());
        ident
    } else {
        let mut i = 2;
        loop {
            let new_ident = format!("{}_{}", ident, i);
            if !used.contains(&new_ident) {
                used.insert(new_ident.clone());
                return new_ident;
            }
            i += 1;
        }
    }
}

pub fn dedup_trait_name(
    base: &str,
    used: &mut HashSet<String>,
    _is_property: bool,
) -> String {
    // Escape keyword
    let candidate = escape_rust_keyword(base);

    // If this is a property accessor and there's already a method with the same name, add _prop
    if used.contains(&candidate) {
        // Try with "_prop"
        let mut prop_candidate = format!("{}_prop", candidate);
        let mut idx = 2;
        while used.contains(&prop_candidate) {
            prop_candidate = format!("{}_prop{}", candidate, idx);
            idx += 1;
        }
        used.insert(prop_candidate.clone());
        return prop_candidate;
    }
    used.insert(candidate.clone());
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! table_tests {
        ($func:ident, [ $( ($name:ident, $input:expr, $expected:expr) ),* $(,)? ]) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($func($input), $expected, "input: {:?}", $input);
                }
            )*
        };
    }

    table_tests!(
        to_snake_case,
        [
            (camel_case, "CamelCase", "camel_case"),
            (already_snake_case, "snake_case", "snake_case"),
            (mixed_example_case, "mixedExampleCase", "mixed_example_case"),
            (single_letter, "A", "a"),
            (empty_string, "", ""),
            (xml_http_request, "XMLHttpRequest", "xml_http_request"),
            (url_value, "URLValue", "url_value"),
            (http_request_id, "HTTPRequestID", "http_request_id"),
            (field1_value2, "Field1Value2", "field1_value2"),
            (my_url, "my-URL", "my_url"),
            (load_html_page, "loadHTMLPage", "load_html_page"),
        ]
    );

    table_tests!(
        dbus_type_to_rust,
        [
            (type_s, "s", "String"),
            (type_u, "u", "u32"),
            (type_i, "i", "i32"),
            (type_b, "b", "bool"),
            (type_ay, "ay", "Vec<u8>"),
            (type_unknown, "unknown", "String"),
            (type_x, "x", "String"),
            (type_empty, "", "String"),
        ]
    );
}
