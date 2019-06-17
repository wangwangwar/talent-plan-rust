extern crate structopt;
use structopt::StructOpt;

use std::io::{self, Write};
use std::process;

#[derive(StructOpt, Debug)]
#[structopt()]
enum Opt {
    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    Set {
        #[structopt(name = "KEY")]
        key: String,

        #[structopt(name = "VALUE")]
        value: String,
    },

    #[structopt(name = "get", about = "Get the string value of a given string key")]
    Get {
        #[structopt(name = "KEY")]
        key: String,
    },

    #[structopt(name = "rm", about = "Remove a given key")]
    Remove {
        #[structopt(name = "KEY")]
        key: String,
    }
}

fn main() {
    let opt = Opt::from_args();
    
    match opt {
        Opt::Set{..} => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1);
        },
        Opt::Get{..} => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1);
        },
        Opt::Remove{..} => {
            let _ = io::stderr().write_all(b"unimplemented");
            process::exit(-1);
        }
    }
}
