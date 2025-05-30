use zbus_xml_gen::generate_client_proxies_from_xml;

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
        let actual = generate_client_proxies_from_xml(XML);
        if !actual.contains($expected) {
          println!("\n=== GENERATED OUTPUT ===\n{}\n=========================", actual);
          panic!("Assertion failed: expected snippet not found:\n{}", $expected);
        }
      }
    )*
  };
}

// Constants and Structs
tests!([
    (
        client_well_known_name_const,
        r#"pub const WELL_KNOWN_NAME: &str = "org.example.Complex";"#
    ),
    (
        client_object_path_const,
        r#"pub const OBJECT_PATH: &str = "/org/example/complex";"#
    ),
    (client_service_config_struct, "pub struct ServiceConfig {"),
    (
        client_service_config_default,
        "impl Default for ServiceConfig {"
    ),
    (
        client_service_config_default_impl,
        r#"well_known_name: WELL_KNOWN_NAME.to_string(),"#
    ),
    (
        client_service_config_default_path,
        r#"object_path: OBJECT_PATH.to_string(),"#
    )
]);

// Trait Declarations
tests!([
    (
        client_trait_complex_decl,
        r#"#[proxy(interface = "org.example.Complex" , assume_defaults = true)]
pub trait Complex {"#
    ),
    (client_trait_second_decl, "pub trait Second {")
]);

// Method Signatures
tests!([
  (client_get_items, "fn get_items(&self) -> zbus::Result<Vec<(String, String, String, std::collections::HashMap<String, String>, u16)>>;"),
  (client_get_dict, "fn get_dict(&self) -> zbus::Result<std::collections::HashMap<String, Vec<u8>>>;"),
  (client_get_string_array, "fn get_string_array(&self) -> zbus::Result<Vec<String>>;"),
  (client_get_int_array, "fn get_int_array(&self) -> zbus::Result<Vec<i32>>;"),
  (client_get_nested_dict, "fn get_nested_dict(&self) -> zbus::Result<std::collections::HashMap<String, std::collections::HashMap<String, String>>>;"),
  (client_get_primitive, "fn get_primitive(&self) -> zbus::Result<i32>;"),
  (client_get_nothing, "fn get_nothing(&self) -> zbus::Result<()>;"),
  (client_multi_return, "fn multi_return(&self) -> zbus::Result<(i32, i32, String)>;"),
  (client_with_inputs, "fn with_inputs(&self, key: String, flag: bool) -> zbus::Result<i32>;"),
  (client_input_output_collision, "fn input_output_collision(&self, value: String) -> zbus::Result<i32>;"),
  (client_deprecated_method, "fn deprecated_method(&self, value: String) -> zbus::Result<()>;"),
  (client_ping_method, "fn ping(&self, input: String) -> zbus::Result<String>;"),
  (client_no_args_returns_bool, "fn no_args_returns_bool(&self) -> zbus::Result<bool>;"),
  (client_method_no_args, "fn method_no_args(&self) -> zbus::Result<()>;"),
  (client_conflict_method, "fn conflict(&self, value: String) -> zbus::Result<String>;"),
  (client_reserved_keyword_method, "fn match_(&self, value: String) -> zbus::Result<String>;"),
  (client_complex_tuple, "fn complex_tuple(&self) -> zbus::Result<(Vec<i32>, (std::collections::HashMap<String, zbus::zvariant::Value<'_>>))>;")
]);

// Properties
tests!([
    (
        client_property_status,
        "fn status(&self) -> zbus::Result<u32>;"
    ),
    (
        client_property_enabled_get,
        "fn enabled(&self) -> zbus::Result<bool>;"
    ),
    (
        client_property_enabled_set,
        "fn set_enabled(&self, value: bool) -> zbus::Result<()>;"
    ),
    (
        client_property_rw_get,
        "fn rw(&self) -> zbus::Result<String>;"
    ),
    (
        client_property_rw_set,
        "fn set_rw(&self, value: String) -> zbus::Result<()>;"
    ),
    (
        client_property_secret_set,
        "fn set_secret(&self, value: String) -> zbus::Result<()>;"
    ),
    (client_property_pi, "fn pi(&self) -> zbus::Result<f64>;"),
    (
        client_property_number,
        "fn number(&self) -> zbus::Result<i32>;"
    ),
    (
        client_property_dict,
        "fn dict_prop(&self) -> zbus::Result<std::collections::HashMap<String, String>>;"
    )
]);

// Signals
tests!([
  (client_signal_state_changed, "fn state_changed(&self) -> zbus::Result<zbus::SignalStream<(i32, String)>>;"),
  (client_signal_items_updated, "fn items_updated(&self) -> zbus::Result<zbus::SignalStream<Vec<(String, String, String, std::collections::HashMap<String, String>, u16)>>>;"),
  (client_simple_signal, "fn simple_signal(&self) -> zbus::Result<zbus::SignalStream<()>>;")
]);
