use compile_time_run::run_command;

#[test]
fn echo() {
	const VALUE: &'static str = run_command!("echo");
	assert_eq!(VALUE, "");
}

#[test]
fn echo_foo() {
	const FOO: &'static str = run_command!("echo", "foo");
	assert_eq!(FOO, "foo");
}

#[test]
fn echo_foo_bar() {
	const FOO: &'static str = run_command!("echo", "foo", "bar");
	assert_eq!(FOO, "foo bar");
}
