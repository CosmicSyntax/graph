/// This is a struct used to represent each node. The data it can house
/// can be currently be anything.
use std::fmt;

pub struct Node<T> {
    pub data: T,
    weight: u8,
    next: Option<Box<Node<T>>>,
}

pub struct Graph<T>(pub Vec<Node<T>>);

impl<T> Graph<T> {
    pub fn new(data: T, weight: u8) -> Graph<T> {
        Graph(vec![Node {
            data,
            weight,
            next: None,
        }])
    }

    pub fn with_capacity(data: T, weight: u8, capacity: usize) -> Graph<T> {
        let mut node: Vec<Node<T>> = Vec::with_capacity(capacity);
        node.push(Node {
            data,
            weight,
            next: None,
        });
        Graph(node)
    }

    pub fn get_node(&self) -> &Vec<Node<T>> {
        &self.0
    }

    pub fn add_edge(&self, data: T, weight: u8) {
        todo!();
    }
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut message = String::new();
        message.push_str(&format!("Data:\t{}\n", self.data));
        message.push_str(&format!("Weight:\t{}\n", self.weight));
        if let Some(x) = &(self.next) {
            message.push_str(&format!("Next:\t{}\n", x));
        } else {
            message.push_str("Next:\tNo Data");
        }
        write!(f, "{}", message)
    }
}
