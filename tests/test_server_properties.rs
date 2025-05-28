use zbus_xml_gen::generate_server_traits_from_xml;
mod common;
use common::{assert_contains, assert_not_contains};

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
fn status_property_getter_generated() {
    let actual = generate_server_traits_from_xml(PROP_XML);
    let expected = "fn get_status(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn number_property_getter_generated() {
    let actual = generate_server_traits_from_xml(PROP_XML);
    let expected = "fn get_number(&self) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}

#[test]
fn rw_property_getter_generated() {
    let actual = generate_server_traits_from_xml(PROP_XML);
    let expected = "fn get_rw(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn rw_property_setter_generated() {
    let actual = generate_server_traits_from_xml(PROP_XML);
    let expected = "fn set_rw(&mut self, value: String) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn secret_property_setter_generated() {
    let actual = generate_server_traits_from_xml(PROP_XML);
    let expected = "fn set_secret(&mut self, value: String) -> zbus::Result<()>;";
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
    let actual = generate_server_traits_from_xml(MISC_PROP_XML);
    let expected = "fn get_enabled(&self) -> zbus::Result<bool>;";
    assert_contains(&actual, expected);
}

#[test]
fn array_property_getter_generated() {
    let actual = generate_server_traits_from_xml(MISC_PROP_XML);
    let expected = "fn get_ids(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn dict_property_getter_generated() {
    let actual = generate_server_traits_from_xml(MISC_PROP_XML);
    let expected = "fn get_config(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn int64_property_getter_generated() {
    let actual = generate_server_traits_from_xml(MISC_PROP_XML);
    let expected = "fn get_value(&self) -> zbus::Result<String>;";
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
    let actual = generate_server_traits_from_xml(WRITE_ONLY_PROP_XML);
    let expected = "fn set_secret(&mut self, value: String) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn write_only_property_no_getter() {
    let actual = generate_server_traits_from_xml(WRITE_ONLY_PROP_XML);
    let not_expected = "fn get_secret(&self)";
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
    let actual = generate_server_traits_from_xml(WEIRD_NAMES_XML);
    let expected = "fn get_first_name(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn property_with_camel_case_generated() {
    let actual = generate_server_traits_from_xml(WEIRD_NAMES_XML);
    let expected = "fn get_api_key(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}
