use compile_time_run::run_command;
use compile_time_run::run_command_str;

#[test]
fn echo() {
	{
		const VALUE: &'static [u8] = run_command!("echo");
		assert_eq!(VALUE, b"");
	} {
		const VALUE: &'static str = run_command_str!("echo");
		assert_eq!(VALUE, "");
	}
}

#[test]
fn echo_foo() {
	{
		const VALUE: &'static [u8] = run_command!("echo", "foo");
		assert_eq!(VALUE, b"foo");
	} {
		const VALUE: &'static str = run_command_str!("echo", "foo");
		assert_eq!(VALUE, "foo");
	}
}

#[test]
fn echo_foo_bar() {
	{
		const VALUE: &'static [u8] = run_command!("echo", "foo", "bar");
		assert_eq!(VALUE, b"foo bar");
	} {
		const VALUE: &'static str = run_command_str!("echo", "foo", "bar");
		assert_eq!(VALUE, "foo bar");
	}
}
