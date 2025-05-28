use zbus_xml_gen::generate_client_proxy;
mod common;
use common::{assert_contains, parse_interface};

const METHOD_XML: &str = r#"
<node>
  <interface name="org.example.MethodCases">
    <method name="NoArgsNoReturn"/>
    <method name="NoArgsWithReturn">
      <arg name="result" type="s" direction="out"/>
    </method>
    <method name="SingleInputNoReturn">
      <arg name="input" type="i" direction="in"/>
    </method>
    <method name="MultiInputSingleOutput">
      <arg name="x" type="i" direction="in"/>
      <arg name="y" type="i" direction="in"/>
      <arg name="sum" type="i" direction="out"/>
    </method>
    <method name="MultiInputMultiOutput">
      <arg name="a" type="i" direction="in"/>
      <arg name="b" type="i" direction="in"/>
      <arg name="sum" type="i" direction="out"/>
      <arg name="diff" type="i" direction="out"/>
    </method>
    <method name="MultiOutput">
      <arg name="first" type="s" direction="out"/>
      <arg name="second" type="s" direction="out"/>
    </method>
    <method name="InputOutputNameCollision">
      <arg name="value" type="s" direction="in"/>
      <arg name="value" type="s" direction="out"/>
    </method>
  </interface>
</node>
"#;

#[test]
fn trait_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "pub trait MethodCases {";
    assert_contains(&actual, expected);
}

#[test]
fn no_args_no_return_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn no_args_no_return(&self) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn no_args_with_return_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn no_args_with_return(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn single_input_no_return_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn single_input_no_return(&self, input: i32) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn multi_input_single_output_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn multi_input_single_output(&self, x: i32, y: i32) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}

#[test]
fn multi_input_multi_output_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected =
        "fn multi_input_multi_output(&self, a: i32, b: i32) -> zbus::Result<(i32, i32)>;";
    assert_contains(&actual, expected);
}

#[test]
fn multi_output_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn multi_output(&self) -> zbus::Result<(String, String)>;";
    assert_contains(&actual, expected);
}

#[test]
fn input_output_name_collision_generated() {
    let iface = parse_interface(METHOD_XML);
    let actual = generate_client_proxy(&iface);
    let expected = "fn input_output_name_collision(&self, value: String) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}
