use zbus_xml_gen::generate_client_proxies_from_xml;
mod common;
use common::{assert_contains, assert_not_contains};

const ANNO_XML: &str = r#"
<node>
  <interface name="org.example.WithAnnotations">
    <annotation name="org.example.interface_ann" value="interface_value"/>
    <method name="setflag">
      <annotation name="org.example.method_ann" value="method_value"/>
      <arg name="flag" direction="in" type="b">
        <annotation name="org.example.arg_ann" value="arg_value"/>
      </arg>
    </method>
    <property name="status" type="s" access="read">
      <annotation name="org.example.prop_ann" value="prop_value"/>
    </property>
    <method name="multiple_annotations">
      <annotation name="org.example.one" value="1"/>
      <annotation name="org.example.two" value="2"/>
    </method>
  </interface>
</node>
"#;

#[test]
fn trait_with_interface_annotation_generated() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let expected = "pub trait WithAnnotations {";
    assert_contains(&actual, expected);
}

#[test]
fn setflag_method_generated() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let expected = "fn setflag(&self, flag: bool) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn property_with_annotation_generated() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let expected = "fn status(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn method_with_multiple_annotations_generated() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let expected = "fn multiple_annotations(&self) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn interface_annotation_not_rendered() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let not_expected = "org.example.interface_ann";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn method_annotation_not_rendered() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let not_expected = "org.example.method_ann";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn arg_annotation_not_rendered() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let not_expected = "org.example.arg_ann";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn property_annotation_not_rendered() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let not_expected = "org.example.prop_ann";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn multiple_method_annotation_not_rendered() {
    let actual = generate_client_proxies_from_xml(ANNO_XML);
    let not_expected = "org.example.one";
    assert_not_contains(&actual, not_expected);
}
