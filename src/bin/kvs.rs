#[macro_use]
extern crate clap;
use clap::App;
use std::io::{self, Write};
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml)
        .name(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();
    match m.subcommand() {
        ("set", Some(_)) => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1);
        }
        ("get", Some(_)) => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1);
        }
        ("rm", Some(_)) => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1);
        }
        (_, _) => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1)
        }
    }
}
