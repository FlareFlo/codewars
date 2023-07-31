use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::str::FromStr;

pub struct AssemblerInterpreter {}

impl AssemblerInterpreter {
	pub fn interpret(input: &str) -> Option<String> {
		let mut output = String::new();

		let code = input.split("\n")
			.map(|e| e.chars().take_while(|char| *char != ';').collect::<String>())
			.map(|line| line.trim().to_string())
			.filter(|line| !line.is_empty()) // Only keep non-comment code
			.collect::<Vec<_>>();

		// Key: Label name - Value: Line on which said label was declared
		let functions: HashMap<&str, usize> = HashMap::from_iter(
			code.iter()
				.enumerate()
				.filter(|line| line.1.ends_with(":"))
				.map(|line| (line.1.split(":").next().unwrap(), line.0))
		);

		// Key: Register name - Value: Register value
		type Registers = HashMap<String, isize>;
		let mut registers: Registers = HashMap::new();

		let match_register_or_value = |input: &str, regs: &mut Registers| {
			if input.chars().all(|char| char.is_ascii_digit()) {
				isize::from_str(input).unwrap()
			} else {
				regs[input]
			}
		};

		let mut cmp: Option<Ordering> = None;

		// Instruction pointer
		let mut ip: usize = 0;
		let mut callstack: Vec<usize> = Vec::new();
		let jmp = |label: &str, ip: &mut usize| {
			*ip = functions[label];
		};
		loop {
			// Break if we reach end of instructions
			if ip >= code.len() {
				break;
			}

			let current_line = &code[ip];
			let tokens = current_line.split_ascii_whitespace().map(|x| Rc::new(x.replace(",", ""))).collect::<Vec<_>>();
			let current_instruction = tokens[0].as_str();
			match current_instruction {
				"mov" => {
					let rhs = match_register_or_value(&tokens[2], &mut registers);
					registers.insert(tokens[1].to_string(), rhs);
				}
				"inc" => {
					*registers.get_mut(tokens[1].as_str()).unwrap() += 1;
				}
				"dec" => {
					*registers.get_mut(tokens[1].as_str()).unwrap() -= 1;
				}
				"add" => {
					let rhs = match_register_or_value(tokens[2].as_str(), &mut registers);
					*registers.get_mut(tokens[1].as_str()).unwrap() += rhs;
				}
				"sub" => {
					let rhs = match_register_or_value(tokens[2].as_str(), &mut registers);
					*registers.get_mut(tokens[1].as_str()).unwrap() -= rhs;
				}
				"mul" => {
					let rhs = match_register_or_value(tokens[2].as_str(), &mut registers);
					*registers.get_mut(tokens[1].as_str()).unwrap() *= rhs;
				}
				"div" => {
					let rhs = match_register_or_value(tokens[2].as_str(), &mut registers);
					*registers.get_mut(tokens[1].as_str()).unwrap() /= rhs;
				}
				"jmp" => {
					ip = functions[tokens[1].as_str()];
				}
				"cmp" => {
					let lhs = match_register_or_value(tokens[1].as_str(), &mut registers);
					let rhs = match_register_or_value(tokens[2].as_str(), &mut registers);
					cmp = Some(lhs.cmp(&rhs));
				}
				"jne" => {
					if cmp.unwrap().is_ne() {
						jmp(&tokens[1], &mut ip);
					}
				}
				"je" => {
					if cmp.unwrap().is_eq() {
						jmp(&tokens[1], &mut ip);
					}
				}
				"jge" => {
					if cmp.unwrap().is_ge() {
						jmp(&tokens[1], &mut ip);
					}
				}
				"jg" => {
					if cmp.unwrap().is_gt() {
						jmp(&tokens[1], &mut ip);
					}
				}
				"jle" => {
					if cmp.unwrap().is_le() {
						jmp(&tokens[1], &mut ip);
					}
				}
				"jl" => {
					if cmp.unwrap().is_lt() {
						jmp(&tokens[1], &mut ip);
					}
				}
				"call" => {
					callstack.push(ip);
					jmp(&tokens[1], &mut ip);
				}
				"ret" => {
					ip = callstack.pop().unwrap();
				}
				"msg" => {
					let args = &current_line[3..];
					let mut in_string = false;
					for char in args.chars() {
						if char == '\'' {
							in_string = !in_string
						} else {
							if in_string {
								output.push(char);
							} else {
								if !matches!(char, ',' | ' ') {
									output.push_str(&registers.get(char.to_string().as_str()).unwrap().to_string());
								}
							}
						}
					}
				}
				"end" => {
					return Some(output);
				}
				_ => {
					if !functions.contains_key(current_instruction.replace(":", "").as_str()) {
						panic!("Unmatched line: {}", current_line);
					}
				}
			}
			ip += 1;
		}
		None
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn simple_test() {
		let simple_programs = &[
			"\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
			"\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
			"\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
			"\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
			"\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
			"\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
			"\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n"];

		let expected = &[
			Some(String::from("(5+1)/2 = 3")),
			Some(String::from("5! = 120")),
			Some(String::from("Term 8 of Fibonacci series is: 21")),
			Some(String::from("mod(11, 3) = 2")),
			Some(String::from("gcd(81, 153) = 9")),
			None,
			Some(String::from("2^10 = 1024"))];

		for (prg, exp) in simple_programs.iter().zip(expected) {
			let actual = AssemblerInterpreter::interpret(*prg);
			assert_eq!(actual, *exp);
		}
	}
}