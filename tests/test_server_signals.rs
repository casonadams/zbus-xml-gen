use zbus_xml_gen::generate_server_traits_from_xml;
mod common;
use common::{assert_contains, assert_not_contains};

const MIXED_XML: &str = r#"
<node>
  <interface name="org.example.Mixed">
    <method name="DoThing">
      <arg name="val" type="i" direction="in"/>
      <arg name="result" type="i" direction="out"/>
    </method>
    <property name="SomeProp" type="s" access="read"/>
    <signal name="Notify"/>
  </interface>
</node>
"#;

#[test]
fn trait_generated_for_mixed_interface() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let expected = "pub trait MixedServer {";
    assert_contains(&actual, expected);
}

#[test]
fn method_generated_in_mixed_interface() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let expected = "fn do_thing(&mut self, val: i32) -> zbus::Result<i32>;";
    assert_contains(&actual, expected);
}

#[test]
fn property_generated_in_mixed_interface() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let expected = "fn get_some_prop(&self) -> zbus::Result<String>;";
    assert_contains(&actual, expected);
}

#[test]
fn no_fn_generated_for_signal_notify() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let not_expected = "fn notify";
    assert_not_contains(&actual, not_expected);
}

const SIGNAL_RESERVED_XML: &str = r#"
<node>
  <interface name="org.example.SignalTest">
    <signal name="ReservedNames">
      <arg name="type" type="s"/>
      <arg name="fn" type="i"/>
    </signal>
  </interface>
</node>
"#;

#[test]
fn no_fn_generated_for_signal_with_reserved_names() {
    let actual = generate_server_traits_from_xml(SIGNAL_RESERVED_XML);
    let not_expected = "fn reserved_names";
    assert_not_contains(&actual, not_expected);
}

const SIGNAL_NO_ARGS_XML: &str = r#"
<node>
  <interface name="org.example.SignalTest">
    <signal name="SignalNoArgs"/>
  </interface>
</node>
"#;

#[test]
fn no_fn_generated_for_signal_with_no_args() {
    let actual = generate_server_traits_from_xml(SIGNAL_NO_ARGS_XML);
    let not_expected = "fn signal_no_args";
    assert_not_contains(&actual, not_expected);
}
