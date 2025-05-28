use zbus_xml_gen::generate_server_traits_from_xml;
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
fn trait_with_interface_annotation_rendered_as_doc() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let expected = "/// [annotation] org.example.interface_ann = \"interface_value\"";
    assert_contains(&actual, expected);
}

#[test]
fn setflag_method_with_method_annotation_rendered_as_doc() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let expected = "/// [annotation] org.example.method_ann = \"method_value\"";
    assert_contains(&actual, expected);
}

#[test]
fn setflag_arg_with_annotation_rendered_as_doc() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let expected = "/// [arg: flag] [annotation] org.example.arg_ann = \"arg_value\"";
    assert_contains(&actual, expected);
}

#[test]
fn property_with_annotation_rendered_as_doc() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let expected = "/// [annotation] org.example.prop_ann = \"prop_value\"";
    assert_contains(&actual, expected);
}

#[test]
fn method_with_multiple_annotations_rendered_as_doc() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let expected1 = "/// [annotation] org.example.one = \"1\"";
    let expected2 = "/// [annotation] org.example.two = \"2\"";
    assert_contains(&actual, expected1);
    assert_contains(&actual, expected2);
}

#[test]
fn interface_annotation_not_rendered_raw() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let not_expected = "<annotation name=\"org.example.interface_ann\"";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn method_annotation_not_rendered_raw() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let not_expected = "<annotation name=\"org.example.method_ann\"";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn arg_annotation_not_rendered_raw() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let not_expected = "<annotation name=\"org.example.arg_ann\"";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn property_annotation_not_rendered_raw() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let not_expected = "<annotation name=\"org.example.prop_ann\"";
    assert_not_contains(&actual, not_expected);
}

#[test]
fn multiple_method_annotations_not_rendered_raw() {
    let actual = generate_server_traits_from_xml(ANNO_XML);
    let not_expected1 = "<annotation name=\"org.example.one\"";
    let not_expected2 = "<annotation name=\"org.example.two\"";
    assert_not_contains(&actual, not_expected1);
    assert_not_contains(&actual, not_expected2);
}
