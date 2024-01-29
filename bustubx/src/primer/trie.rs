#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
	is_value_node: bool,
	children: HashMap<char, TrieNode>
}

#[derive(Debug)]
struct Trie {
	root: TrieNode
}

impl Trie {
	fn new()->Trie {
		return Trie {
			root: TrieNode::default()
		}
	 }
	fn get<T>(&self, key: char)->T { todo!() }
	fn put<T>(&self, key: char, value: T)->Trie{ todo!()}
	fn remove(&self, key: char)->Trie { todo!() }
}


#[cfg(test)]
mod tests {
	use crate::primer::trie;

	#[test]
	fn test_can_construct() {
	}

	fn test_can_put() {

	}

	fn test_can_get() {

	}

	fn test_copies_on_write() {

	}
}