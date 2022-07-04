fn valid_parentheses(input: &str) -> bool {
	let mut counter = 0;

	for char in input.chars() {
		match char {
			'(' => {counter += 1},
			')' => {
				counter -= 1;
				if counter < 0 {
					return false;
				}
			},
			_ => {}
		}
	}
	counter == 0
}