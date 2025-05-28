use zbus_xml_gen::generate_server_traits_from_xml;

mod common;
use common::assert_contains;

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
fn signal_notify_emitter_fn_signature() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let expected = "fn emit_notify<'a>(";
    assert_contains(&actual, expected);
}

#[test]
fn signal_notify_emitter_uses_signal_emitter() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let expected = "SignalEmitter<'a>";
    assert_contains(&actual, expected);
}

#[test]
fn signal_notify_emitter_return_type() {
    let actual = generate_server_traits_from_xml(MIXED_XML);
    let expected = "-> impl std::future::Future<Output = zbus::Result<()>> + Send";
    assert_contains(&actual, expected);
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
fn signal_reserved_names_function() {
    let actual = generate_server_traits_from_xml(SIGNAL_RESERVED_XML);
    let expected = "fn emit_reserved_names<'a>(";
    assert_contains(&actual, expected);
}

#[test]
fn signal_reserved_names_arg_type_is_escaped() {
    let actual = generate_server_traits_from_xml(SIGNAL_RESERVED_XML);
    let expected = "type_: String";
    assert_contains(&actual, expected);
}

#[test]
fn signal_reserved_names_arg_fn_is_escaped() {
    let actual = generate_server_traits_from_xml(SIGNAL_RESERVED_XML);
    let expected = "fn_: i32";
    assert_contains(&actual, expected);
}

const SIGNAL_NO_ARGS_XML: &str = r#"
<node>
  <interface name="org.example.SignalTest">
    <signal name="SignalNoArgs"/>
  </interface>
</node>
"#;

#[test]
fn signal_no_args_function_generated() {
    let actual = generate_server_traits_from_xml(SIGNAL_NO_ARGS_XML);
    let expected = "fn emit_signal_no_args<'a>(";
    assert_contains(&actual, expected);
}

#[test]
fn signal_no_args_uses_signal_emitter() {
    let actual = generate_server_traits_from_xml(SIGNAL_NO_ARGS_XML);
    let expected = "SignalEmitter<'a>";
    assert_contains(&actual, expected);
}

#[test]
fn signal_no_args_return_type() {
    let actual = generate_server_traits_from_xml(SIGNAL_NO_ARGS_XML);
    let expected = "-> impl std::future::Future<Output = zbus::Result<()>> + Send";
    assert_contains(&actual, expected);
}

#[test]
fn signal_no_args_emit_unit_value() {
    let actual = generate_server_traits_from_xml(SIGNAL_NO_ARGS_XML);
    let expected = "&()";
    assert_contains(&actual, expected);
}
