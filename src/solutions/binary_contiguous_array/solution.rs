fn binarray(a: &[u8]) -> u32 {
	let len = a.len();

	let is_contiguous = |input: &[u8]| {
		let mut counter = 0_i32;
		for i in input {
			counter += if *i != 0 {
				1
			} else {
				-1
			};
		};
		counter == 0
	};

	if is_contiguous(a) {
		return len as u32;
	}

	let mut longest = 0;
	for (start, _) in a.iter().enumerate() {
		let start = start as usize;
		for (end, _) in a.iter().enumerate() {
			let true_end = len - end;
			if true_end - start < 1 {
				break;
			}
			let slice = &a[start..true_end];
			if len < longest {
				continue;
			}
			let is = is_contiguous(&slice);
			if is && slice.len() > longest {
				longest = slice.len();
				break;
			}
		}
	}
	longest as u32
}




#[cfg(test)]
mod tests {
	use tokio::time::Instant;
	use super::binarray;

	const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

	fn dotest(a: &[u8], expected: u32) {
		assert_eq!(binarray(a), expected, "{ERR_MSG} with a= {a:?}")
	}

	#[test]
	fn fixed_tests() {
		for (input, expected) in [(vec![0,1], 2),
			(vec![0], 0),
			(vec![1,1,0,1,1,0,1,1],4),
			(vec![0,1,1,0,1,1,1,0,0,0],10),
			(vec![0,0,1,1,1,0,0,0,0,0],6),
		] {
			dotest(&input, expected);
		}
	}

	#[test]
	fn bench() {
		let start = Instant::now();
		for _ in 0..100000000 {
			dotest(&[0,1,1,0,1,1,1,0,0,0], 10);
		}
		println!("{:?}", start.elapsed());
	}
}

