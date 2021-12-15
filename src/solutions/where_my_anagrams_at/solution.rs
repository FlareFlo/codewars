fn anagrams(word: &str, words: &[String]) -> Vec<String> {
	let mut word = word.chars().collect::<Vec<char>>();
	word.sort();

	let mut values = Vec::new();

	for target in words {
		let mut new_target = target.chars().collect::<Vec<char>>();
		new_target.sort();
		if word == new_target {
			values.push(target.to_owned())
		}
	}
	values
}