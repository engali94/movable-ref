#![allow(clippy::uninlined_format_args)]

use movable_ref::{selfref_accessors, SelfRefCell};

struct Node {
    value: SelfRefCell<String, i16>,
}

impl Node {
    fn new(value: String) -> Self {
        let value = SelfRefCell::new(value).unwrap();
        Self { value }
    }
}

selfref_accessors!(impl Node { get_value, get_value_mut: value -> String });

impl Node {
    fn len(&self) -> usize {
        self.get_value().len()
    }
}

fn main() {
    let node = Node::new("Hello, World!".to_string());
    println!("Original: {}", node.get_value());

    let boxed_node = Box::new(node);
    println!("In Box: {}", boxed_node.get_value());

    let nodes = [*boxed_node, Node::new("Another node".to_string())];

    nodes.iter().enumerate().for_each(|(i, node)| {
        println!("Node {}: '{}' (len: {})", i, node.get_value(), node.len());
    });

    println!("\nNote: These structures remain valid after all moves!");
}
