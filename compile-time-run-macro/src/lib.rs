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

extern crate proc_macro;

use std::process::Command;

use syn::parse_macro_input;
use quote::quote;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn run_command_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let args : Vec<_> = parse_macro_input!(input as ArgList).args.iter().map(|x| x.value()).collect();

	let output = Command::new(&args[0]).args(&args[1..]).output().expect(&format!("run_command: failed to execute command: {}", &args[0]));
	let output = verbose_command_error(&args[0], output).unwrap();
	let output = strip_trailing_newline(output.stdout);
	let output = std::str::from_utf8(&output).expect("invalid UTF-8 in command output");

	let tokens = quote!{#output};
	proc_macro::TokenStream::from(tokens)
}

#[proc_macro_hack]
pub fn run_command(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let args : Vec<_> = parse_macro_input!(input as ArgList).args.iter().map(|x| x.value()).collect();

	let output = Command::new(&args[0]).args(&args[1..]).output().expect(&format!("run_command: failed to execute command: {}", &args[0]));
	let output = verbose_command_error(&args[0], output).unwrap();
	let output = strip_trailing_newline(output.stdout);
	let output = std::str::from_utf8(&output).expect("invalid UTF-8 in command output");

	let tokens = quote!{#output};
	proc_macro::TokenStream::from(tokens)
}

struct ArgList {
	args : syn::punctuated::Punctuated<syn::LitStr, syn::token::Comma>,
}

impl syn::parse::Parse for ArgList {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		type Inner = syn::punctuated::Punctuated<syn::LitStr, syn::token::Comma>;
		let args = Inner::parse_terminated(&input)?;

		if args.is_empty() {
			Err(syn::Error::new(input.cursor().span(), "missing required argument: command"))
		} else {
			Ok(Self{args})
		}
	}
}

/// Remove a trailing newline from a byte string.
fn strip_trailing_newline(mut input: Vec<u8>) -> Vec<u8> {
	if input.len() > 0 && input[input.len() - 1] == b'\n' {
		input.pop();
	}
	input
}

/// Check if a command ran successfully, and if not, return a verbose error.
fn verbose_command_error<C>(command: C, output: std::process::Output) -> std::io::Result<std::process::Output> where
	C: std::fmt::Display,
{
	// If the command succeeded, just return the output as is.
	if output.status.success() {
		Ok(output)

	// If the command terminated with non-zero exit code, return an error.
	} else if let Some(status) = output.status.code() {
		// Include stderr in the error message, if it's valid UTF-8 and not empty.
		let message = strip_trailing_newline(output.stderr);
		if let Some(message) = String::from_utf8(message).ok().filter(|x| !x.is_empty()) {
			Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} failed with status {}: {}", command, status, message)))
		} else {
			Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} failed with status {}", command, status)))
		}

	// The command was killed by a signal.
	} else {
		// Include the signal number on Unix.
		#[cfg(target_family = "unix")] {
			use std::os::unix::process::ExitStatusExt;
			let signal = output.status.signal().unwrap();
			Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} killed by signal {}",  command, signal)))
		}
		#[cfg(not(target_family = "unix"))] {
			Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} killed by signal", command)))
		}
	}
}
