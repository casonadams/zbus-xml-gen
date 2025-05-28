use zbus_xml_gen::generate_client_proxies_from_xml;
mod common;
use common::assert_contains;

const EMPTY_XML: &str = r#"
<node>
    <interface name="org.example.empty"/>
</node>
"#;

#[test]
fn trait_generated_for_empty_interface() {
    let actual = generate_client_proxies_from_xml(EMPTY_XML);
    let expected = "pub trait empty {";
    assert_contains(&actual, expected);
}

const MULTI_EMPTY_XML: &str = r#"
<node>
    <interface name="org.example.empty1"/>
    <interface name="org.example.empty2"/>
</node>
"#;

#[test]
fn trait_generated_for_multiple_empty_interfaces_1() {
    let actual = generate_client_proxies_from_xml(MULTI_EMPTY_XML);
    let expected = "pub trait empty1 {";
    assert_contains(&actual, expected);
}

#[test]
fn trait_generated_for_multiple_empty_interfaces_2() {
    let actual = generate_client_proxies_from_xml(MULTI_EMPTY_XML);
    let expected = "pub trait empty2 {";
    assert_contains(&actual, expected);
}

const EMPTY_ANNOTATED_XML: &str = r#"
<node>
    <interface name="org.example.anno">
      <annotation name="org.example.foo" value="bar"/>
    </interface>
</node>
"#;

#[test]
fn trait_generated_for_empty_annotated_interface() {
    let actual = generate_client_proxies_from_xml(EMPTY_ANNOTATED_XML);
    let expected = "pub trait anno {";
    assert_contains(&actual, expected);
}
