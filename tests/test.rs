use compile_time_run::run_command;
use compile_time_run::run_command_str;

use assert2::assert;

// Ensure run_command! can be used in static context.
const _STATIC_ECHO: &[u8] = run_command!("echo");
const _STATIC_ECHO_STR: &str = run_command_str!("echo");

#[test]
fn echo() {
	assert!(run_command!("echo") == b"");
	assert!(run_command_str!("echo") == "");
}

#[test]
fn echo_foo() {
	assert!(run_command!("echo", "foo") == b"foo");
	assert!(run_command_str!("echo", "foo") == "foo");
}

#[test]
fn echo_foo_bar() {
	assert_eq!(run_command!("echo", "foo", "bar"), b"foo bar");
	assert_eq!(run_command_str!("echo", "foo", "bar"), "foo bar");
}
