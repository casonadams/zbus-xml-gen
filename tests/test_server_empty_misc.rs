use zbus_xml_gen::generate_server_traits_from_xml;
mod common;
use common::assert_contains;

const EMPTY_XML: &str = r#"
<node>
    <interface name="org.example.empty"/>
</node>
"#;

#[test]
fn trait_generated_for_empty_interface() {
    let actual = generate_server_traits_from_xml(EMPTY_XML);
    let expected = "pub trait emptyServer {";
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
    let actual = generate_server_traits_from_xml(MULTI_EMPTY_XML);
    let expected1 = "pub trait empty1Server {";
    assert_contains(&actual, expected1);
}

#[test]
fn trait_generated_for_multiple_empty_interfaces_2() {
    let actual = generate_server_traits_from_xml(MULTI_EMPTY_XML);
    let expected2 = "pub trait empty2Server {";
    assert_contains(&actual, expected2);
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
    let actual = generate_server_traits_from_xml(EMPTY_ANNOTATED_XML);
    let expected = "pub trait annoServer {";
    assert_contains(&actual, expected);
}
