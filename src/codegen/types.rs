pub fn dbus_type_to_rust(ty: &str) -> String {
    let mut chars = ty.chars().peekable();
    parse_dbus_type(&mut chars)
}

fn parse_dbus_type<I>(chars: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    match chars.next() {
        Some('y') => "u8".into(),
        Some('b') => "bool".into(),
        Some('n') => "i16".into(),
        Some('q') => "u16".into(),
        Some('i') => "i32".into(),
        Some('u') => "u32".into(),
        Some('x') => "i64".into(),
        Some('t') => "u64".into(),
        Some('d') => "f64".into(),
        Some('s') => "String".into(),
        Some('o') => "zbus::zvariant::ObjectPath<'_>".into(),
        Some('g') => "zbus::zvariant::Signature<'_>".into(),
        Some('v') => "zbus::zvariant::Value<'_>".into(),
        Some('a') => {
            if let Some('{') = chars.peek() {
                chars.next(); // skip '{'
                let key = parse_dbus_type(chars);
                let val = parse_dbus_type(chars);
                assert_eq!(chars.next(), Some('}'), "invalid dict signature");
                format!("std::collections::HashMap<{}, {}>", key, val)
            } else {
                let inner = parse_dbus_type(chars);
                format!("Vec<{}>", inner)
            }
        }
        Some('(') => {
            let mut fields = Vec::new();
            while let Some(&ch) = chars.peek() {
                if ch == ')' {
                    chars.next();
                    break;
                }
                fields.push(parse_dbus_type(chars));
            }
            format!("({})", fields.join(", "))
        }
        Some(ch) => {
            eprintln!("Warning: unknown D-Bus type '{}'", ch);
            "zbus::zvariant::Value<'_>".into()
        }
        None => "zbus::zvariant::Value<'_>".into(),
    }
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
        dbus_type_to_rust,
        [
            // Basic types
            (type_s, "s", "String"),
            (type_y, "y", "u8"),
            (type_b, "b", "bool"),
            (type_n, "n", "i16"),
            (type_q, "q", "u16"),
            (type_i, "i", "i32"),
            (type_u, "u", "u32"),
            (type_x, "x", "i64"),
            (type_t, "t", "u64"),
            (type_d, "d", "f64"),
            (type_o, "o", "zbus::zvariant::ObjectPath<'_>"),
            (type_g, "g", "zbus::zvariant::Signature<'_>"),
            (type_v, "v", "zbus::zvariant::Value<'_>"),
            // Arrays
            (type_ay, "ay", "Vec<u8>"),
            (type_as, "as", "Vec<String>"),
            (type_ai, "ai", "Vec<i32>"),
            // Structs
            (type_struct_ii, "(ii)", "(i32, i32)"),
            (type_struct_sib, "(sib)", "(String, i32, bool)"),
            (type_struct_nested, "((ii)i)", "((i32, i32), i32)"),
            // Dicts
            (
                type_dict_ss,
                "a{ss}",
                "std::collections::HashMap<String, String>"
            ),
            (
                type_dict_sv,
                "a{sv}",
                "std::collections::HashMap<String, zbus::zvariant::Value<'_>>"
            ),
            (type_dict_iu, "a{iu}", "std::collections::HashMap<i32, u32>"),
            // Combinations
            (type_array_of_structs, "a(ii)", "Vec<(i32, i32)>"),
            (
                type_array_of_dicts,
                "aa{sv}",
                "Vec<std::collections::HashMap<String, zbus::zvariant::Value<'_>>>"
            ),
            // Fallback cases
            (type_unknown, "z", "zbus::zvariant::Value<'_>"),
            (type_empty, "", "zbus::zvariant::Value<'_>")
        ]
    );
}
