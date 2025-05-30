use zbus_xml_gen::generate_server_traits_from_xml;

const XML: &str = r#"
<node>
  <interface name="org.example.Complex">
    <!-- Methods -->
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
    <method name="DeprecatedMethod">
      <annotation name="org.freedesktop.DBus.Deprecated" value="true"/>
      <arg name="value" type="s" direction="in"/>
    </method>

    <!-- Signals -->
    <signal name="StateChanged">
      <arg name="state" type="i"/>
      <arg name="error" type="s"/>
    </signal>
    <signal name="ItemsUpdated">
      <arg name="items" type="a(sssa{ss}q)"/>
    </signal>

    <!-- Properties -->
    <property name="Status" type="u" access="read"/>
    <property name="Enabled" type="b" access="readwrite"/>
    <property name="status" type="s" access="read"/>
    <property name="number" type="i" access="read"/>
    <property name="rw" type="s" access="readwrite"/>
    <property name="secret" type="s" access="write"/>
    <property name="enabled" type="b" access="readwrite"/>
    <property name="pi" type="d" access="read"/>
  </interface>

  <interface name="org.example.Second">
    <method name="Ping">
      <arg name="input" type="s" direction="in"/>
      <arg name="output" type="s" direction="out"/>
    </method>
    <method name="NoArgsReturnsBool">
      <arg name="result" type="b" direction="out"/>
    </method>
    <method name="MethodNoArgs"/>
    <signal name="SimpleSignal"/>
    <property name="dict_prop" type="a{ss}" access="read"/>
    <method name="Conflict">
      <arg name="value" type="s" direction="in"/>
      <arg name="value" type="s" direction="out"/>
    </method>
    <method name="match">
      <arg name="value" type="s" direction="in"/>
      <arg name="result" type="s" direction="out"/>
    </method>
    <method name="ComplexTuple">
      <arg name="result" type="(ai(a{sv}))" direction="out"/>
    </method>
  </interface>
</node>
"#;

macro_rules! tests {
  ([ $( ($name:ident, $expected:expr) ),* $(,)? ]) => {
    $(
      #[test]
      fn $name() {
        let actual = generate_server_traits_from_xml(XML);
        if !actual.contains($expected) {
          println!("\n=== GENERATED OUTPUT ===\n{}\n=========================", actual);
          panic!("Assertion failed: expected snippet not found:\n{}", $expected);
        }
      }
    )*
  };
}

tests!([
  // Imports and Constants
  (server_contains_use_zbus, "use zbus::{interface, Connection, Result};"),
  (server_contains_object_path_const, r#"pub const OBJECT_PATH: &str = "/org/example/Complex";"#),
  (server_contains_well_known_name_const, r#"pub const WELL_KNOWN_NAME: &str = "org.example.Complex";"#),

  // Trait
  (server_trait_handler_decl, "pub trait ComplexHandler: Send + Sync {"),
  (server_handler_method_get_items, "fn get_items(&self) -> Vec<(String, String, String, std::collections::HashMap<String, String>, u16)>;"),
  (server_handler_method_multi_return, "fn multi_return(&self) -> (i32, i32, String);"),
  (server_handler_method_with_inputs, "fn with_inputs(&self, key: String, flag: bool) -> i32;"),

  // Struct
  (server_struct_decl, "pub struct Complex {"),
  (server_struct_new_fn, "pub fn new(inner: Arc<dyn ComplexHandler>) -> Self {"),

  // Impl (interface) block
  (server_interface_impl_start, "#[interface(name = \"org.example.Complex\")]"),
  (server_forward_get_dict, "fn get_dict(&self) -> std::collections::HashMap<String, Vec<u8>> {"),
  (server_forward_input_output_collision, "fn input_output_collision(&self, value: String) -> i32 {"),
  (server_multi_return_out_args, "#[zbus(out_args(\"x\", \"y\", \"name\"))]"),

  // builder
  (server_struct_builder, "pub struct ComplexServerBuilder {"),
  (server_struct_server, "pub struct ComplexServer {"),
  (server_builder_fn_build, "pub async fn build(self) -> Result<ComplexServer> {"),
  (server_register_object, r#"conn.object_server().at(obj_path, Complex::new(self.implementation.clone())).await?;"#),
  (server_request_name, r#"conn.request_name(WellKnownName::try_from("org.example.Complex")?).await?;"#)
]);

// More Handler Method Signatures
tests!([
  (server_handler_get_dict, "fn get_dict(&self) -> std::collections::HashMap<String, Vec<u8>>;"),
  (server_handler_get_string_array, "fn get_string_array(&self) -> Vec<String>;"),
  (server_handler_get_int_array, "fn get_int_array(&self) -> Vec<i32>;"),
  (server_handler_get_nested_dict, "fn get_nested_dict(&self) -> std::collections::HashMap<String, std::collections::HashMap<String, String>>;"),
  (server_handler_get_primitive, "fn get_primitive(&self) -> i32;"),
  (server_handler_get_nothing, "fn get_nothing(&self) -> ();"),
  (server_handler_deprecated_method, "fn deprecated_method(&self, value: String) -> ();"),
  (server_handler_ping, "fn ping(&self, input: String) -> String;"),
  (server_handler_no_args_returns_bool, "fn no_args_returns_bool(&self) -> bool;"),
  (server_handler_method_no_args, "fn method_no_args(&self) -> ();"),
  (server_handler_conflict, "fn conflict(&self, value: String) -> String;"),
  (server_handler_match, "fn match_(&self, value: String) -> String;"),
]);

// More Impl Forwarding
tests!([
  (server_forward_get_string_array, "fn get_string_array(&self) -> Vec<String> {"),
  (server_forward_get_int_array, "fn get_int_array(&self) -> Vec<i32> {"),
  (server_forward_get_nested_dict, "fn get_nested_dict(&self) -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {"),
  (server_forward_get_primitive, "fn get_primitive(&self) -> i32 {"),
  (server_forward_get_nothing, "fn get_nothing(&self) -> () {"),
  (server_forward_ping, "fn ping(&self, input: String) -> String {"),
  (server_forward_no_args_returns_bool, "fn no_args_returns_bool(&self) -> bool {"),
  (server_forward_method_no_args, "fn method_no_args(&self) -> () {"),
  (server_forward_conflict, "fn conflict(&self, value: String) -> String {"),
  (server_forward_match, "fn match_(&self, value: String) -> String {"),
  (server_forward_complex_tuple, r#"fn complex_tuple(&self) -> (Vec<i32>, (std::collections::HashMap<String, zbus::zvariant::Value<'_>>)) {"#)
]);
