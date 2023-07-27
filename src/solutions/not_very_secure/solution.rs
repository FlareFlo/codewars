fn alphanumeric(password: &str) -> bool {
	if password.len() == 0 {
		return false;
	}
	password.chars().all(|char|char.is_alphanumeric())
}

#[cfg(test)]
mod tests {
	use super::alphanumeric;

	fn do_test(s: &str, expected: bool) {
		let actual = alphanumeric(s);
		assert_eq!(actual, expected, "\nInput: {s:?}\nYour result (left) did not match the expected output (right)")
	}

	#[test]
	fn sample_tests() {
		do_test("hello world_", false);
		do_test("PassW0rd", true);
		do_test("     ", false);
	}
}