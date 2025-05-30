use zbus_xml_gen::generate_server_interface_from_xml;

const XML: &str = r#"
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
    <method name="DeprecatedMethod">
      <annotation name="org.freedesktop.DBus.Deprecated" value="true"/>
      <arg name="value" type="s" direction="in"/>
    </method>
    <signal name="StateChanged">
      <arg name="state" type="i"/>
      <arg name="error" type="s"/>
    </signal>
    <signal name="ItemsUpdated">
      <arg name="items" type="a(sssa{ss}q)"/>
    </signal>
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
                let actual = generate_server_interface_from_xml(XML);
                if !actual.contains($expected) {
                    println!("\n=== GENERATED OUTPUT ===\n{}\n=========================", actual);
                    panic!("Assertion failed: expected snippet not found:\n{}", $expected);
                }
            }
        )*
    };
}

tests!([
    // Use/imports
    (server_contains_use_zbus, "use zbus::{interface, Result};"),

    // Trait declaration
    (server_trait_handler_decl, "pub trait ComplexDelegate: Send + Sync"),

    // Struct declaration and constructor
    (server_struct_decl, "pub struct Complex {"),
    (server_struct_new_fn, "pub fn new(delegate: Arc<dyn ComplexDelegate>) -> Self {"),

    // Interface annotation
    (server_interface_impl_start, "#[interface(name = \"org.example.Complex\")]"),

    // Trait methods (async + zbus::fdo::Result)
    (server_handler_method_get_items, "async fn get_items(&self) -> zbus::fdo::Result<Vec<(String, String, String, std::collections::HashMap<String, String>, u16)>>;"),
    (server_handler_method_multi_return, "async fn multi_return(&self) -> zbus::fdo::Result<(i32, i32, String)>;"),
    (server_handler_method_with_inputs, "async fn with_inputs(&self, key: String, flag: bool) -> zbus::fdo::Result<i32>;"),
    (server_handler_get_dict, "async fn get_dict(&self) -> zbus::fdo::Result<std::collections::HashMap<String, Vec<u8>>>;"),
    (server_handler_get_string_array, "async fn get_string_array(&self) -> zbus::fdo::Result<Vec<String>>;"),
    (server_handler_get_int_array, "async fn get_int_array(&self) -> zbus::fdo::Result<Vec<i32>>;"),
    (server_handler_get_nested_dict, "async fn get_nested_dict(&self) -> zbus::fdo::Result<std::collections::HashMap<String, std::collections::HashMap<String, String>>>;"),
    (server_handler_get_primitive, "async fn get_primitive(&self) -> zbus::fdo::Result<i32>;"),
    (server_handler_get_nothing, "async fn get_nothing(&self) -> zbus::fdo::Result<()>;"),
    (server_handler_deprecated_method, "async fn deprecated_method(&self, value: String) -> zbus::fdo::Result<()>;"),
    (server_handler_ping, "async fn ping(&self, input: String) -> zbus::fdo::Result<String>;"),
    (server_handler_no_args_returns_bool, "async fn no_args_returns_bool(&self) -> zbus::fdo::Result<bool>;"),
    (server_handler_method_no_args, "async fn method_no_args(&self) -> zbus::fdo::Result<()>;"),
    (server_handler_conflict, "async fn conflict(&self, value: String) -> zbus::fdo::Result<String>;"),
    (server_handler_match, "async fn match_(&self, value: String) -> zbus::fdo::Result<String>;"),

    // Signals
    (server_signal_state_changed, "async fn state_changed(emitter: SignalEmitter<'_>, state: i32, error: String) -> Result<()>;"),
    (server_signal_items_updated, "async fn items_updated(emitter: SignalEmitter<'_>, items: Vec<(String, String, String, std::collections::HashMap<String, String>, u16)>) -> Result<()>;"),
    (server_signal_simple_signal, "async fn simple_signal(emitter: SignalEmitter<'_>) -> Result<()>;"),

    // Properties
    (server_property_status_u, "async fn status(&self) -> u32;"),
    (server_property_enabled_bool, "async fn enabled(&self) -> bool;"),
    (server_property_set_enabled_bool, "async fn set_enabled(&mut self, val: bool)"),
    (server_property_status_s, "async fn status(&self) -> String;"),
    (server_property_number_i, "async fn number(&self) -> i32;"),
    (server_property_rw_s, "async fn rw(&self) -> String;"),
    (server_property_set_rw_s, "async fn set_rw(&mut self, val: String)"),
    (server_property_set_secret, "async fn set_secret(&mut self, val: String)"),
    (server_property_pi_d, "async fn pi(&self) -> f64;"),
    (server_property_dict_prop, "async fn dict_prop(&self) -> std::collections::HashMap<String, String>;"),
]);
