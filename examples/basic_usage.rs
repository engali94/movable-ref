use tether::SelfRef;

struct Node {
    value: String,
    self_ref: SelfRef<String, i16>,
}

impl Node {
    fn new(value: String) -> Self {
        let mut node = Self {
            value,
            self_ref: SelfRef::null(),
        };
        
        // Set up the self-reference
        node.self_ref.set(&mut node.value).unwrap();
        node
    }
    
    fn get_value(&self) -> &str {
        unsafe { self.self_ref.as_ref_unchecked() }
    }
    
    fn len(&self) -> usize {
        self.get_value().len()
    }
}

fn main() {
    let node = Node::new("Hello, World!".to_string());
    println!("Original: {}", node.get_value());
    
    let boxed_node = Box::new(node);
    println!("In Box: {}", boxed_node.get_value());
    
    let mut nodes = Vec::new();
    nodes.push(*boxed_node);
    nodes.push(Node::new("Another node".to_string()));
    
    for (i, node) in nodes.iter().enumerate() {
        println!("Node {}: '{}' (len: {})", i, node.get_value(), node.len());
    }
    
    println!("\nNote: These structures remain valid after all moves!");
} 
