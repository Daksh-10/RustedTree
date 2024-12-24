use std::io;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Node {
    id: u16, 
    children: Vec<Node>,
}

impl Node {
    fn new(id: u16) -> Self {
        Node {
            id,
            children: Vec::new(),
        }
    }

    fn traverse(&self) {
        println!("Node ID: {}", self.id);
        for child in &self.children {
            child.traverse();
        }
    }

    // Serialize the node to a file
    fn serialize_to_file(&self, file_path: &str) {
        let file = File::create(file_path).expect("Failed to create file");
        bincode::serialize_into(file, self).expect("Failed to serialize node");
    }

    // Deserialize a node from a file
    fn deserialize_from_file(file_path: &str) -> Node {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");
        bincode::deserialize(&buffer).expect("Failed to deserialize node")
    }
    
}

// Using Leibniz's Equation to calculate pi value
// 1 - 1/3 + 1/5 - 1/7 + 1/9 - .... = PI/4

// fn calculate_pi(terms: usize) -> f64 {
//     let mut pi = 0.0;
//     let mut sign = 1.0;
//     for i in 0..terms {
//         pi += sign / (2.0 * i as f64 + 1.0);
//         sign = -sign;
//     }
//     pi * 4.0
// }

fn take_input() -> usize{
    println!("Enter the depth of the tree: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let depth: usize = input
        .trim() // Remove whitespace and newline
        .parse() // parse into usize 
        .expect("Please enter a valid number");

    println!("Your depth: {}", depth);
    depth
}

fn main() {
    
    
    let depth = take_input();
    
    let pi_digits: Vec<i8> = vec![4, 1, 5, 9, 2, 6, 5, 3, 5];
    // println!("{}",calculate_pi(1000000));  use this function instead

    let mut id = 1;
    let mut root_of_tree = Node::new(id);
    id+=1;

    
    let mut current_level = &mut root_of_tree;
    for i in 0..depth {
        for _ in 1..=pi_digits[i]{
            current_level.children.push(Node::new(id));
            id+=1;
        }
        current_level = &mut current_level.children[0];
    }
    //Traversal before ser-de
    root_of_tree.traverse();

    // Serialize the tree to a file
    root_of_tree.serialize_to_file("tree.bin");

    // Deserialize the tree from the file
    let deserialized_tree = Node::deserialize_from_file("tree.bin");

    //printing the required root_of_tree initialized struct
    println!("The deserialized root struct is-> \n{:?}", deserialized_tree);

    // deserialized_tree.traverse();
}
