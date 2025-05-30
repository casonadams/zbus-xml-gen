#[cfg(feature = "cli")]
fn main() {
    use clap::Parser;
    use std::fs;
    use std::io::{self, Read};
    use zbus_xml_gen::{generate_client_proxies_from_xml, generate_server_interface_from_xml};

    #[derive(Parser)]
    #[command(author, version, about)]
    struct Cli {
        /// Generate server trait (default is client proxy)
        #[arg(long)]
        server: bool,

        /// Input XML file (defaults to stdin if not provided)
        input: Option<String>,
    }

    let cli = Cli::parse();

    // Read XML from file or stdin into a String
    let xml = match &cli.input {
        Some(path) => fs::read_to_string(path).expect("Failed to read XML file"),
        None => {
            let mut xml = String::new();
            io::stdin()
                .read_to_string(&mut xml)
                .expect("Failed to read from stdin");
            xml
        }
    };

    // Generate and print code
    let code = if cli.server {
        generate_server_interface_from_xml(&xml)
    } else {
        generate_client_proxies_from_xml(&xml)
    };
    println!("{}", code);
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("The CLI is only available with the `cli` feature enabled.");
    std::process::exit(1);
}
