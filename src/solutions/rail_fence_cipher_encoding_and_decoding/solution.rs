fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
	let mut rails = vec![vec![None; text.len()]; num_rails];

	for (i, char) in text.chars().enumerate() {
		let cycle = 2 * num_rails - 2;
		let step = i % cycle;
		let height = if step < num_rails { step } else { cycle - step };
		rails[height][i] = Some(char);
	}

	rails.into_iter().flatten().filter_map(|e| e).collect()
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
	let mut rails = vec![vec![None; text.len()]; num_rails];

	let mut rail = 0;
	let mut i = 0;
	for char in text.chars() {
		let index = if rail % 2 == 0 {
			(i * (num_rails + 1)) + rail % num_rails
		} else {
			(i * (num_rails - rail)) + rail % num_rails
		};

		rails[rail][index] = Some(char);

		if index >= text.len() - 2 {
			rail += 1;
			i = 0;
		} else {
			i += 1;
		}
	}

	dbg_rails(&rails);

	let mut out = String::new();
	for i in 0..text.chars().count() {
		// Bounces between 0 and num_rails
		let cycle = 2 * num_rails - 2;
		let step = i % cycle;
		let height = if step < num_rails { step } else { cycle - step };
		eprintln!("{} {} {} {}", height, i, num_rails, text);
		out.push(rails[height][i].unwrap());
	}
	out
}

fn dbg_rails(rails: &Vec<Vec<Option<char>>>) {
	println!("{}", rails.iter().map(|rail| rail.clone().into_iter().map(|char| char.unwrap_or('-')).collect::<String>() + "\n").collect::<String>());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_tests() {
		assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
		assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
		assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
		assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
	}

	#[test]
	fn extra() {
		assert_eq!(decode_rail_fence_cipher("WAEICVRDLETNEERDSOEEFEAOC", 2), "WECRLTEERDSOEEFEAOCAIVDEN");
	}
}
