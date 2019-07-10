#![deny(missing_docs)]
//! Key value store
extern crate libc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};
use std::path::Path;
use std::result;

/// Key value store struct
#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
    log_file: File,
}

/// Custom error type
#[derive(Debug)]
pub enum Error {
    /// IO Error
    Io(io::Error),
    /// Serde error
    Serde(bincode::Error),
    /// Key not found error
    KeyNotFound(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Serde(err)
    }
}

/// Custom result type
pub type Result<T> = result::Result<T, Error>;

/// For serde
#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },

    Get { key: String },

    Remove { key: String },
}

/// Log data file's name
const LOG_DATA_FILE_NAME: &str = "log.data";

/// Implementation choices
/// 
/// 1. Serialization Format
///     Do you want to prioritize performance? Do you want to be able to read the content of the
///     log in plain text?
/// 
/// 2. Serialization Method
///     Write it either to a String or a stream implementing Write?
/// 
/// 3. Deserialization Method
///     3.1 Should you read all records in the log into memory at once and then replay them into 
///         your map type; or should you read them one at a time while replaying the into your map?
///     3.2 Should you read into a buffer before deserializing or deserialize from a file stream?
///
/// 4. IO Mode
///     Read and write the log data file in which IO mode? Buffered or direct? Block or non-block?
///     Sync or async?
/// 
/// A1: Bincode
/// 
/// A2: Write it to a String
/// 
/// A3.1: Now I choose to read then one at a time while replaying into the map. From the memory 
///     usage perspective, the former implementation use a buffer with *infinite size*, while 
///     the latter use a buffer with *one record size*. From the IO efficiency perspective, the
///     former implementation read IO sequently, while the latter read IO randomly. Reading IO
///     sequently is faster than randomly in most cases.
///     TODO: Better choice is to use a buffer with a *larger fixed size*, balance the memory
///     usage and IO efficiency.
/// 
/// A3.2: Now I choose to read into a buffer before deserializing. Bincode has the function
///     `bincode::deserilize_from` which can deserialize an object directly from `File`(`Read`er),
///     but how it read the file is unknown to me (source code?), and can't control the way to
///     read the file.
/// 
/// A4: Now I choose the default buffered, block and async IO mode. At the beginning I want to use
///     buffered IO like this:
///     ```
///     extern crate libc;
///     use std::{fs::OpenOptions, os::unix::fs::OpenOptionsExt};
/// 
///     fn main() {
///         let options = OpenOptions::new()
///             .read(true)
///             .write(true)
///             .create(true)
///         if cfg!(unix) {
///             options.custom_flags(libc::O_DIRECT);
///         }
///         let file = options.open("foo.txt");
///     }
///     ```
///     But return the error `Io(Os { code: 22, kind: InvalidInput, message: "Invalid argument" })`
///     TODO: Fix it
impl KvStore {
    /// Open the KV store
    ///
    /// TODO:
    /// Open/create the log data file in `path` with *direct* mode, because we
    /// manage data caching at the application level, so we do not need the
    /// file system to implement this service for them. The use of a file
    /// buffer cache results in undesirable overheads in such cases, since data
    /// is first moved from the disk to the file buffer cache and from there to
    /// the application buffer. This “doublecopying” of data results in more CPU
    /// consumption and adds overhead to the memory too.
    ///
    /// Return the new instance
    pub fn open(path: &Path) -> Result<Self> {
        let mut map = HashMap::new();
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true);
        let mut log_file = options.open(path.join(LOG_DATA_FILE_NAME))?;

        loop {
            let mut len_buf = [0; 1];
            let result = log_file
                .read_exact(&mut len_buf)
                .map_err(Error::Io)
                .and_then(|_| {
                    let len = len_buf.first().unwrap();
                    let mut command_buf = vec![0; *len as usize];
                    log_file.read_exact(&mut command_buf)?;
                    let command: Command = bincode::deserialize(&command_buf)?;
                    match command {
                        Command::Set { key, value } => {
                            map.insert(key, value);
                        }
                        Command::Remove { key } => {
                            map.remove(&key);
                        }
                        Command::Get { .. } => {}
                    }
                    Ok(())
                });
            if result.is_err() {
                break;
            }
        }

        Ok(KvStore { map, log_file })
    }

    /// Set the value of the string `key` to the `value`
    ///
    /// Steps:
    ///     It then serializes that command to a String
    ///     It then appends the serialized command to a file containing the log
    ///     If that succeeds, it exits silently with error code 0
    ///     If it fails, it exits by printing the error and returning a non-zero error code
    ///
    /// Binary format:
    ///     <length of serialized command><serialized command>
    ///
    /// Return `Ok` if success,
    /// return `Err` if failure
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let mut encoded: Vec<u8> = bincode::serialize(&command).unwrap();
        encoded.insert(0, encoded.len() as u8);
        self.log_file.write_all(&encoded)?;

        self.map.insert(key, value);
        Ok(())
    }

    /// Get the value of the string `key`
    ///
    /// "get"
    ///     kvs reads the entire log, one command at a time, recording the affected key and file offset of the command to an in-memory key -> log pointer map
    ///     It then checks the map for the log pointer
    ///     If it fails, it prints "Key not found", and exits with exit code 0
    ///     If it succeeds
    ///         It deserializes the command to get the last recorded value of the key
    ///         It prints the value to stdout and exits with exit code 0
    ///
    /// Return `Ok(Some)` when getting a existent key,
    /// return `Ok(None)` when getting a non-existent key,
    /// return `Err` when error
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    /// Remove the `key`
    ///
    /// Steps:
    ///     Same as the "get" command, kvs reads the entire log to build the in-memory index
    ///     It then checks the map if the given key exists
    ///     If the key does not exist, it prints "Key not found", and exits with a non-zero error code
    ///     If it succeeds
    ///         It creates a value representing the "rm" command, containing its key
    ///         It then appends the serialized command to the log
    ///         If that succeeds, it exits silently with error code 0
    ///
    /// Return `Ok(value)` previously stored value when removing a existent key,
    /// return `Ok(None)`
    /// return `Err(Error::KeyNotFound)` when when removing a non-existent key
    /// return `Err` when other error occurs
    pub fn remove(&mut self, key: String) -> Result<String> {
        let command = Command::Remove { key: key.clone() };
        let mut encoded: Vec<u8> = bincode::serialize(&command).unwrap();
        encoded.insert(0, encoded.len() as u8);
        self.log_file.write_all(&encoded)?;
        self.map.remove(&key).ok_or_else(|| Error::KeyNotFound(key))
    }
}
