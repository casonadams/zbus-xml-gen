use zbus_xml_gen::generate_client_proxy;
mod common;
use common::{assert_contains, assert_not_contains, parse_interface};

const PROP_XML: &str = r#"
<node>
    <interface name="org.example.HasProp">
        <property name="status" type="s" access="read"/>
        <property name="number" type="i" access="read"/>
        <property name="rw" type="s" access="readwrite"/>
        <property name="secret" type="s" access="write"/>
    </interface>
</node>
"#;

#[test]
fn status_property_generated() {
    let iface = parse_interface(PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn status(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn number_property_generated() {
    let iface = parse_interface(PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn number(&self) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}

#[test]
fn rw_property_getter_generated() {
    let iface = parse_interface(PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn rw(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn rw_property_setter_generated() {
    let iface = parse_interface(PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn set_rw(&self, value: String) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn secret_property_setter_generated() {
    let iface = parse_interface(PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn set_secret(&self, value: String) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

const MISC_PROP_XML: &str = r#"
<node>
    <interface name="org.example.MiscProp">
        <property name="enabled" type="b" access="read"/>
        <property name="ids" type="au" access="read"/>
        <property name="config" type="a{ss}" access="read"/>
        <property name="value" type="x" access="read"/>
    </interface>
</node>
"#;

#[test]
fn bool_property_getter_generated() {
    let iface = parse_interface(MISC_PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn enabled(&self) -> zbus::Result<bool>;";
    assert_contains(&actual, expected);
}

#[test]
fn array_property_getter_generated() {
    let iface = parse_interface(MISC_PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn ids(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn dict_property_getter_generated() {
    let iface = parse_interface(MISC_PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn config(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn int64_property_getter_generated() {
    let iface = parse_interface(MISC_PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn value(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

const WRITE_ONLY_PROP_XML: &str = r#"
<node>
    <interface name="org.example.WriteOnly">
        <property name="secret" type="s" access="write"/>
    </interface>
</node>
"#;

#[test]
fn write_only_property_setter_generated() {
    let iface = parse_interface(WRITE_ONLY_PROP_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn set_secret(&self, value: String) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn write_only_property_no_getter() {
    let iface = parse_interface(WRITE_ONLY_PROP_XML);
    let actual = generate_client_proxy(&iface);
    let not_expected = "fn secret(&self)";
    assert_not_contains(&actual, not_expected);
}

const WEIRD_NAMES_XML: &str = r#"
<node>
    <interface name="org.example.WeirdProp">
        <property name="First_Name" type="s" access="read"/>
        <property name="APIKey" type="s" access="read"/>
    </interface>
</node>
"#;

#[test]
fn property_with_underscore_generated() {
    let iface = parse_interface(WEIRD_NAMES_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn first_name(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn property_with_camel_case_generated() {
    let iface = parse_interface(WEIRD_NAMES_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn api_key(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}
