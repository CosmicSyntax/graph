use std::fmt;

/// This is a struct used to represent each node. The data it can house
/// can be currently be anything.
pub struct Node<T> {
    pub data: T,
    weight: u8,
    next: Option<Box<Node<T>>>,
}

/// This is a struct that represents the whole graph data structure for
/// particular instance
pub struct Graph<T>(pub Vec<Node<T>>);

/// Trait to restrict what the generic could be, and the following lines
/// of code also allows the user to the turbo fish ::<>
pub trait Restriction {
    fn alloc(init_size: usize) -> Vec<Graph<Self>>
        where
            Self: Sized;
}

// macro to expand to include 6 data types
macro_rules! make_restriction {
    ($($x:ty),*) => {
        $(
            impl Restriction for $x {
                fn alloc(init_size: usize) -> Vec<Graph<Self>> {
                    let data: Vec<Graph<$x>> = Vec::with_capacity(init_size);
                    data
                }
            }
        )*
    };
}
make_restriction![u8, u32, u64, f32, f64, String];

pub fn new<Z: Restriction>(init_size: usize) -> Vec<Graph<Z>> {
    Restriction::alloc(init_size)
}

/// Methods for Graph data
impl<T> Graph<T> {
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
