// Given a binary tree, implement a function that returns the maximum depth of the tree.
use std::cmp;

pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node{
            val: value,
            left: None,
            right: None
        }
    }
}

fn max_depth(root: Option<Box<Node>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1+cmp::max(left_depth, right_depth)
        },
        None => 0
    }
}

fn main() {
 let root = Some(Box::new(Node {
    val: 7,
    left: Some(Box::new(Node{
        val: 8,
        left: Some(Box::new(Node::new(2))),
        right: Some(Box::new(Node::new(3)))
    })),
    right:Some(Box::new(Node::new(3)))
 }));

 let max_depth = max_depth(root);
 println!("The max depth of given binary tree is {}", max_depth);
}