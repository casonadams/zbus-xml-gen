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
