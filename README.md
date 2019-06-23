# Pingcap Talant Plan for Rust

ref: https://github.com/pingcap/talent-plan

## Project 1: The Rust toolbox

- [x] Part 1: Make the tests compile
- [x] Part 2: Accept command line arguments
    - [x] cli_no_args
    - [x] cli_get
    - [x] cli_set
    - [x] cli_rm
    - [x] cli_invalid_get
    - [x] cli_invalid_set
    - [x] cli_invalid_rm
    - [x] cli_invalid_subcommand
- [x] Part 3: Cargo environment variables
    - [x] cli_version
- [x] Part 4: Store values in memory
    - [x] get_stored_value
    - [x] overwrite_value
    - [x] get_non_existent_value
    - [x] remove_key

- [x] Part 5: Documentation
- [x] Part 6: Ensure good style with clippy and rustfmt
- [x] Extension 1: structopt

## Project 2: Log-structured file I/O

- [x] Part 1: Error handling
- [ ] Part 2: How the log behaves
- [ ] Part 3: Writing to the log
- [ ] Part 4: Reading from the log
- [ ] Part 5: Storing log pointers in the index
- [ ] Part 6: Stateless vs. stateful `KvStore`
- [ ] Part 7: Compacting the log