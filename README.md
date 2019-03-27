[![Documentation](https://docs.rs/compile-time-run/badge.svg)](https://docs.rs/compile-time-run)
[![crates.io](https://img.shields.io/crates/v/compile-time-run.svg)](https://crates.io/crates/compile-time-run)
[![Build Status](https://travis-ci.org/de-vri-es/compile-time-run.svg?branch=master)](https://travis-ci.org/de-vri-es/compile-time-run)

# compile-time-run

This crate contains macros to run commands on your system during compile time.

You should probably not be doing this in most public projects,
but there could be legit uses in personal or private projects.

An example:
```rust
use compile_time_run::{run_command, run_command_str};
const VALUE_STR   : &'static str  = run_command_str!("echo", "Hello World!");
const VALUE_BYTES : &'static [u8] = run_command!("echo", "Hello World!");
```
