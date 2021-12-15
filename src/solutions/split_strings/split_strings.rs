fn solution(s: &str) -> Vec<String> {
	let mut splits = vec![];
	let mut string = s;

	while string.len() > 1 {
		splits.push(string[..2].to_owned());
		string = &string[2..];
	}
	if string.len() == 1 {
		splits.push(format!("{}_", string));
	}
	splits
}