use std::env;
use std::io::{self, Read};
use zbus_xml_gen::{generate_client_proxy, parse_node_from_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    let node = if args.len() > 1 {
        // File path provided as first arg
        parse_node_from_file(&args[1])
    } else {
        // No arg: read from stdin
        let mut xml = String::new();
        io::stdin()
            .read_to_string(&mut xml)
            .expect("Failed to read from stdin");
        let cursor = std::io::Cursor::new(xml);
        zbus_xml::Node::from_reader(cursor).expect("Failed to parse XML from stdin")
    };

    for iface in node.interfaces() {
        let code = generate_client_proxy(iface);
        println!("{}", code);
    }
}
