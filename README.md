# zbus-xml-gen

Generate type-safe Rust code from D-Bus XML introspection files.

This library and CLI help you create zbus client proxies and server traits automatically from your D-Bus XML interface files.

## Features

- Library: Easily generate Rust trait code for zbus proxies and servers from XML, all in memory.
- CLI: Generate code from XML files or stdin with a simple command.

## Library Usage

Add to your Cargo.toml:

```toml
[dependencies]
zbus-xml-gen = "0.1"
```

### Generate Client Proxies

```rust
use zbus_xml_gen::generate_client_proxies_from_xml;

let xml = r#"
<node>
  <interface name="org.example.Foo">
    <method name="Bar"><arg name="x" type="i" direction="in"/></method>
    <property name="status" type="s" access="read"/>
  </interface>
</node>
"#;

let code = generate_client_proxies_from_xml(xml);
println!("{}", code);
// -> Generates Rust traits usable as zbus client proxies
```

### Generate Server Traits

```rust
use zbus_xml_gen::generate_server_traits_from_xml;

let xml = r#"
<node>
  <interface name="org.example.Foo">
    <method name="Bar"><arg name="x" type="i" direction="in"/></method>
  </interface>
</node>
"#;

let code = generate_server_traits_from_xml(xml);
println!("{}", code);
// -> Generates Rust traits for implementing D-Bus servers
```

## CLI Usage

Enable the CLI with the cli feature:

```toml
[dependencies]
zbus-xml-gen = { version = "0.1", features = ["cli"] }
```

### Build and Run

```sh
cargo run --features cli -- --help
```

### Usage:

```sh
zbus-xml-gen [--server] [input.xml]
```

- `input.xml` – Path to a D-Bus introspection XML file. If not given, reads from stdin.
- `--server` – Generate server trait code (default: client proxy code).

### Examples:

```sh
# Generate client proxy traits from file
zbus-xml-gen interfaces.xml

# Generate server traits from stdin
cat interfaces.xml | zbus-xml-gen --server
```

## Why?

- Don’t hand-write D-Bus interface bindings for Rust and zbus.
- Generate type-safe, idiomatic Rust code from your XML interface definitions automatically.
