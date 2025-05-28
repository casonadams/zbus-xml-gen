use zbus_xml_gen::generate_client_proxy;
mod common;
use common::{assert_contains, parse_interface};

const EMPTY_XML: &str = r#"
<node>
    <interface name="org.example.empty"/>
</node>
"#;

#[test]
fn trait_generated_for_empty_interface() {
    let iface = parse_interface(EMPTY_XML);
    let actual = generate_client_proxy(&iface);
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
    let node = zbus_xml::Node::from_reader(std::io::Cursor::new(MULTI_EMPTY_XML)).unwrap();
    let iface1 = node
        .interfaces()
        .iter()
        .find(|i| i.name() == "org.example.empty1")
        .unwrap();
    let actual1 = generate_client_proxy(iface1);
    let expected1 = "pub trait empty1 {";
    assert_contains(&actual1, expected1);
}

#[test]
fn trait_generated_for_multiple_empty_interfaces_2() {
    let node = zbus_xml::Node::from_reader(std::io::Cursor::new(MULTI_EMPTY_XML)).unwrap();
    let iface2 = node
        .interfaces()
        .iter()
        .find(|i| i.name() == "org.example.empty2")
        .unwrap();
    let actual2 = generate_client_proxy(iface2);
    let expected2 = "pub trait empty2 {";
    assert_contains(&actual2, expected2);
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
    let iface = parse_interface(EMPTY_ANNOTATED_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "pub trait anno {";
    assert_contains(&actual, expected);
}
