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

//! This is an implementation crate used by [compile-time-run](https://docs.rs/compile-time-run)
//! with [proc-macro-hack](https://docs.rs/proc-macro-hack)
//! to expose a procedural macro that can be invoked in expression context.

extern crate proc_macro;

use proc_macro_hack::proc_macro_hack;
use syn::parse_macro_input;

#[proc_macro_hack]
pub fn run_command_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	detail::run_command_str(parse_macro_input!(input))
		.unwrap_or_else(|error| error.to_compile_error())
		.into()
}

#[proc_macro_hack]
pub fn run_command(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	detail::run_command(parse_macro_input!(input))
		.unwrap_or_else(|error| error.to_compile_error())
		.into()
}

mod detail {
	use std::process::Command;

	use quote::quote;
	use syn::{Error, Result};
	use proc_macro2::Span;

	pub fn run_command_str(input: ArgList) -> Result<proc_macro2::TokenStream> {
		let args : Vec<_> = input.args.iter().map(|x| x.value()).collect();

		let output = execute_command(Command::new(&args[0]).args(&args[1..]))?;
		let output = strip_trailing_newline(output.stdout);
		let output = std::str::from_utf8(&output).expect("invalid UTF-8 in command output");

		Ok(quote!(#output))
	}

	pub fn run_command(input: ArgList) -> Result<proc_macro2::TokenStream> {
		let args : Vec<_> = input.args.iter().map(|x| x.value()).collect();

		let output = execute_command(Command::new(&args[0]).args(&args[1..]))?;
		let output = strip_trailing_newline(output.stdout);

		if output.is_empty() {
			// If the array is empty and the resulting code doesn't compile,
			// this gives a nicer error than the else branch.
			Ok(quote!(&[0u8; 0]))
		} else {
			Ok(quote!( &[ #(#output,)* ] ))
		}
	}

	/// Comma seperated argument list of string literals.
	pub struct ArgList {
		args : syn::punctuated::Punctuated<syn::LitStr, syn::token::Comma>,
	}

	impl syn::parse::Parse for ArgList {
		fn parse(input: syn::parse::ParseStream) -> Result<Self> {
			type Inner = syn::punctuated::Punctuated<syn::LitStr, syn::token::Comma>;
			let args = Inner::parse_terminated(&input)?;

			if args.is_empty() {
				Err(Error::new(input.cursor().span(), "missing required argument: command"))
			} else {
				Ok(Self{args})
			}
		}
	}

	fn execute_command(command: &mut Command) -> Result<std::process::Output> {
		let output = command.output().map_err(|error|
			Error::new(Span::call_site(), format!("failed to execute command: {}", error))
		)?;

		verbose_command_error(output).map_err(|message|
			Error::new(Span::call_site(), message)
		)
	}

	/// Check if a command ran successfully, and if not, return a verbose error.
	fn verbose_command_error(output: std::process::Output) -> std::result::Result<std::process::Output, String>
	{
		// If the command succeeded, just return the output as is.
		if output.status.success() {
			Ok(output)

		// If the command terminated with non-zero exit code, return an error.
		} else if let Some(status) = output.status.code() {
			// Include stderr in the error message if it's not empty, no too long,
			// has no newlines and is valid UTF-8.
			let message = Some(strip_trailing_newline(output.stderr));

			let message = message.filter(|m| m.len() > 0 && m.len() <= 500);
			let message = message.filter(|m| m.iter().position(|c| c == &b'\n').is_none());
			let message = message.and_then(|m| String::from_utf8(m).ok());

			if let Some(message) = message {
				Err(format!("external command exited with status {}: {}", status, message))
			} else {
				Err(format!("external command exited with status {}", status))
			}

		// The command was killed by a signal.
		} else {
			// Include the signal number on Unix.
			#[cfg(target_family = "unix")] {
				use std::os::unix::process::ExitStatusExt;
				if let Some(signal) = output.status.signal() {
					Err(format!("external command killed by signal {}", signal))
				} else {
					Err(format!("external command failed, but did not exit and was not killed by a signal, this can only be a bug in std::process"))
				}
			}
			#[cfg(not(target_family = "unix"))] {
				Err(format!("external command killed by signal"))
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
}
