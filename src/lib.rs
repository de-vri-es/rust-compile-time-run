// Copyright 2019 Maarten de Vries <maarten@de-vri.es>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! This crate contains macros to run commands on your system during compile time.
//!
//! You should probably not be doing this in most public projects,
//! but there could be legit uses in personal or private projects.
//!
//! An example:
//! ```
//! use compile_time_run::{run_command, run_command_str};
//! const VALUE_STR   : &'static str  = run_command_str!("echo", "Hello World!");
//! const VALUE_BYTES : &'static [u8] = run_command!("echo", "Hello World!");
//! ```


use proc_macro_hack::proc_macro_hack;

/// Run a command at compile time, and return the output as a byte slice.
///
/// The output is a static &[u8], and can be used for the value of consts.
/// If the command fails to run, a compile error is generated.
///
/// For example:
/// ```
/// use compile_time_run::run_command;
/// const VALUE : &'static [u8] = run_command!("echo", "Hello World!");
/// ```
#[proc_macro_hack]
pub use compile_time_run_macro::run_command;


/// Run a command at compile time, and return the output as a str.
///
/// The output is a static &str, and can be used for the value of consts.
/// If the command fails to run, a compile error is generated.
///
/// For example:
/// ```
/// use compile_time_run::run_command_str;
/// const VALUE : &'static str = run_command_str!("echo", "Hello World!");
/// ```
#[proc_macro_hack]
pub use compile_time_run_macro::run_command_str;
