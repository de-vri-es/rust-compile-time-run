[![Documentation](https://docs.rs/compile-time-run/badge.svg)](https://docs.rs/compile-time-run)
[![crates.io](https://img.shields.io/crates/v/compile-time-run.svg)](https://crates.io/crates/compile-time-run)
[![Build Status](https://travis-ci.org/de-vri-es/rust-compile-time-run.svg?branch=master)](https://travis-ci.org/de-vri-es/rust-compile-time-run)

# compile-time-run

This crate contains macros to run commands on your system during compile time.
It can be used in some situations to take over functionaility that would otherwise
have to be done using a build script.


An example:
```rust
use compile_time_run::{run_command, run_command_str};
const VALUE_STR   : &'static str  = run_command_str!("echo", "Hello World!");
const VALUE_BYTES : &'static [u8] = run_command!("echo", "Hello World!");
```

Keep in mind that running abitrary commands during your build phase can easily hurt portability.
