struct Sieve {
	primes: Vec<u64>,
	last: usize,
	before_last: usize,
}

impl Sieve {
	#[inline(always)]
	fn unset_bit(&mut self, n: usize) {
		let index = n / 64;
		let bit = n % 64;
		self.primes[index] &= !(1 << bit);
	}
}

impl Iterator for Sieve {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.last == self.before_last {
			return None;
		}

		self.before_last = self.last;

		let start = (self.last + 1) / 64;
		let mut found = false;

		for i in start..self.primes.len() {
			if self.primes[i] != 0 {
				let base = i * 64;
				for offset in 0..64 {
					let n = base + offset;
					if n > self.before_last && (self.primes[i] & (1 << offset)) != 0 {
						self.last = n;
						found = true;
						break;
					}
				}
				if found {
					break;
				}
			}
		}

		for i in (self.last..self.primes.len() * 64).step_by(self.last) {
			self.unset_bit(i);
		}

		Some(self.last as u32)
	}
}


fn stream() -> impl Iterator<Item = u32> {
	Sieve {
		primes: vec![u64::MAX; 1_000_000],
		last: 1,
		before_last: 0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_segment(start: u32, numbers: [u32; 10]){
		let mut prime_iterator = stream();
		for _ in 0..start{
			prime_iterator.next();
		}
		for i in numbers{
			assert_eq!(Some(i), prime_iterator.next(),
					   "\nYour result (left) did not match the expected output (right)");
		}
	}

	#[test]
	fn tests() {
		println!("testing segment from 0");
		test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

		println!("testing segment from 10");
		test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);

		println!("testing segment from 100");
		test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);

		println!("testing segment from 1,000");
		test_segment(1_000, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017]);
	}
}