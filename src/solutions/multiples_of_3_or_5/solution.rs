fn solution(num: i32) -> i32 {
	let mut sum = 0;
	for i in 0..num {
		if i % 3 == 0 || i % 5 == 0  {
			sum += i;
		}
	}
	sum
}