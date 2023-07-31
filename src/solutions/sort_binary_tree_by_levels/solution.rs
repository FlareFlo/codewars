#[derive(Clone, Debug)]
struct Node {
	value: u32,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>
}

impl Node {
	pub fn new(value: u32) -> Self {
		Self {
			value,
			left: None,
			right: None,
		}
	}
	pub fn left(mut self, left: Self) -> Self {
		self.left = Some(Box::new(left));
		self
	}
	pub fn right(mut self, right: Self) -> Self {
		self.right = Some(Box::new(right));
		self
	}
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
	let mut values: Vec<Vec<u32>> = vec![];
	recurse(&mut values, root, &mut 0);
	values.into_iter().flatten().collect()
}

fn recurse(values: &mut Vec<Vec<u32>>, node: &Node, level: &mut usize) {
	*level += 1;
	if values.len() <=  *level + 1 {
		values.resize(*level + 1, vec![]);
	}
	values[*level - 1].push(node.value);
	if let Some(left) = &node.left {
		recurse(values, left.as_ref(), level);
	}
	if let Some(right) = &node.right {
		recurse(values, right.as_ref(), level);
	}
	*level -= 1;

}

#[cfg(test)]
mod sample_tests {
	use super::*;

	#[test]
	fn root_only() {
		assert_eq!(tree_by_levels(&Node::new(42)),
				   [42],
				   "\nYour result (left) didn't match the expected output (right).");
	}

	#[test]
	fn complete_tree() {
		let root = Node::new(1)
			.left(Node::new(2)
				.left(Node::new(4))
				.right(Node::new(5)))
			.right(Node::new(3)
				.left(Node::new(6)));
		assert_eq!(tree_by_levels(&root),
				   [1,2,3,4,5,6],
				   "\nYour result (left) didn't match the expected output (right).");
	}
}