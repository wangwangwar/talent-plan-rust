/// How the log behaves
/// 
/// The log is a record of the transactions committed to the database. By "replaying" the records in the log on startup we reconstruct the previous state of the database.
/// 
/// "set"
///     The user invokes kvs set mykey myvalue
///     kvs creates a value representing the "set" command, containing its key and value
///     It then serializes that command to a String
///     It then appends the serialized command to a file containing the log
///     If that succeeds, it exits silently with error code 0
///     If it fails, it exits by printing the error and returning a non-zero error code
/// 
/// "get"
///     The user invokes kvs get mykey
///     kvs reads the entire log, one command at a time, recording the affected key and file offset of the command to an in-memory key -> log pointer map
///     It then checks the map for the log pointer
///     If it fails, it prints "Key not found", and exits with exit code 0
///     If it succeeds
///         It deserializes the command to get the last recorded value of the key
///         It prints the value to stdout and exits with exit code 0
/// 
/// "rm"
///     The user invokes kvs rm mykey
///     Same as the "get" command, kvs reads the entire log to build the in-memory index
///     It then checks the map if the given key exists
///     If the key does not exist, it prints "Key not found", and exits with a non-zero error code
///     If it succeeds
///         It creates a value representing the "rm" command, containing its key
///         It then appends the serialized command to the log
///         If that succeeds, it exits silently with error code 0
extern crate structopt;
use structopt::StructOpt;
use std::path::Path;
use kvs::{self, KvStore, Result};

use std::io::{self, Write, prelude::*};
use std::error::Error;
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

const LOG_DATA_PATH_NAME: &str = "./";

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut kvs = KvStore::open(Path::new(LOG_DATA_PATH_NAME))?;
    
    match opt {
        Opt::Set{key, value} => {
            let result = kvs.set(key, value);
            match result {
                Ok(_) => {
                    process::exit(0);
                }
                Err(kvs::Error::Io(err)) => {
                    io::stderr().write_all(err.description().as_bytes());
                    process::exit(-1);
                },
                Err(kvs::Error::Serde(err)) => {
                    io::stderr().write_all(err.description().as_bytes());
                    process::exit(-1);
                },
                Err(kvs::Error::KeyNotFound(_)) => {
                    io::stdout().write_all(b"Key not found");
                    process::exit(-1);
                },
            }
        },
        Opt::Get{key} => {
            match kvs.get(key) {
                Ok(optional_value) => {
                    match optional_value {
                        Some(value) => {
                            io::stdout().write_all(value.as_bytes());
                            process::exit(0);
                        }
                        None => {
                            io::stdout().write_all(b"Key not found");
                            process::exit(0);
                        }
                    }
                }
                Err(kvs::Error::Io(err)) => {
                    io::stderr().write_all(err.description().as_bytes());
                    process::exit(-1);
                },
                Err(kvs::Error::Serde(err)) => {
                    io::stderr().write_all(err.description().as_bytes());
                    process::exit(-1);
                },
                Err(kvs::Error::KeyNotFound(_)) => {
                    io::stdout().write_all(b"Key not found");
                    process::exit(-1);
                },
            }
        },
        Opt::Remove{key} => {
            let result = kvs.remove(key);
            match result {
                Ok(_) => process::exit(0),
                Err(kvs::Error::Io(err)) => {
                    io::stderr().write_all(err.description().as_bytes());
                    process::exit(-1);
                },
                Err(kvs::Error::Serde(err)) => {
                    io::stderr().write_all(err.description().as_bytes());
                    process::exit(-1);
                },
                Err(kvs::Error::KeyNotFound(_)) => {
                    io::stdout().write_all(b"Key not found");
                    process::exit(-1);
                },
            }
        }
    }
}
