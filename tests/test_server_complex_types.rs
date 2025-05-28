use zbus_xml_gen::generate_server_traits_from_xml;
mod common;
use common::assert_contains;

const COMPLEX_XML: &str = r#"
<node>
  <interface name="org.example.Complex">
    <method name="GetItems">
      <arg name="items" type="a(sssa{ss}q)" direction="out"/>
    </method>
    <method name="GetDict">
      <arg name="dict" type="a{say}" direction="out"/>
    </method>
    <method name="GetStringArray">
      <arg name="values" type="as" direction="out"/>
    </method>
    <method name="GetIntArray">
      <arg name="values" type="ai" direction="out"/>
    </method>
    <method name="GetNestedDict">
      <arg name="map" type="a{sa{ss}}" direction="out"/>
    </method>
    <method name="GetPrimitive">
      <arg name="value" type="i" direction="out"/>
    </method>
    <method name="GetNothing"/>
    <method name="MultiReturn">
      <arg name="x" type="i" direction="out"/>
      <arg name="y" type="i" direction="out"/>
      <arg name="name" type="s" direction="out"/>
    </method>
    <method name="WithInputs">
      <arg name="key" type="s" direction="in"/>
      <arg name="flag" type="b" direction="in"/>
      <arg name="result" type="i" direction="out"/>
    </method>
    <method name="InputOutputCollision">
      <arg name="value" type="s" direction="in"/>
      <arg name="value" type="i" direction="out"/>
    </method>
  </interface>
</node>
"#;

#[test]
fn get_items_method_generated_with_expected_type() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_items(&mut self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn get_dict_method_generated_with_expected_type() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_dict(&mut self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn get_string_array_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_string_array(&mut self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn get_int_array_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_int_array(&mut self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn get_nested_dict_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_nested_dict(&mut self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn get_primitive_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_primitive(&mut self) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}

#[test]
fn get_nothing_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn get_nothing(&mut self) -> zbus::Result<()>;";
    assert_contains(&actual, expected);
}

#[test]
fn multi_return_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn multi_return(&mut self) -> zbus::Result<(i32, i32, String)>;";
    assert_contains(&actual, expected);
}

#[test]
fn with_inputs_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn with_inputs(&mut self, key: String, flag: bool) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}

#[test]
fn input_output_collision_method_generated() {
    let actual = generate_server_traits_from_xml(COMPLEX_XML);
    let expected = "fn input_output_collision(&mut self, value: String) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}
