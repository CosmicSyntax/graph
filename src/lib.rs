//! This is a library for implementing Graph data structures
use std::fmt;

/// This is a struct used to represent each node. The data it can house
/// can be currently be anything.
pub struct Node<T> {
    pub data: T,
    weight: u8,
    next: Option<Box<Node<T>>>,
}

/// This is a struct that represents the whole graph data structure for
/// particular instance.
pub struct Vector<T>(pub Vec<Node<T>>);

pub struct Graph<'a, T>(Vec<Vector<T>>, &'a [T]);

/// Trait to restrict what the generic could be, and the following lines
/// of code also allows the user to the turbo fish ::<>, instead of type
/// annotation.
pub trait Restriction<'a, T> {
    fn alloc(template: &'a [T]) -> Graph<'a, Self>
    where
        Self: Sized;
}

// macro to expand to include 7 data types
macro_rules! make_restriction {
    ($($datatype:ty),*) => {
        $(
            impl<'a> Restriction<'a, $datatype> for $datatype {
                fn alloc(template: &'a [$datatype]) -> Graph<'a, Self> {
                    let data = Vec::<Vector<$datatype>>::with_capacity(template.len());
                    // The compiler can infer the type here, so there is no need for
                    // annotation for types... but here done it anyway
                    // Annotation goes after Vec b/c generic was with Vec
                    Graph(data, template)
                }
            }
        )*
    };
}
// Lifetime for str needs to be 'a' to match the lifetime in the macro
make_restriction![u8, u32, u64, f32, f64, String, &'a str];

/// Methods for Vector data
impl<'a, T: Restriction<'a, T>> Graph<'a, T> {
    pub fn new(template: &'a [T]) -> Graph<'a, T> {
        Restriction::alloc(template)
    }

    //pub fn get_node(&self) -> &Vec<Node<T>> {
    //&self.0
    //}

    pub fn add_edge(&mut self, src: T, des: T, dir: bool, weight: u8)
    where
        T: PartialEq,
    {
        match dir {
            true => {}
            false => {
                // Where direction does not matter
                let des_index = self
                    .1
                    .iter()
                    .position(|x| x == &des)
                    .expect("Could not find.");
                let src_index = self
                    .1
                    .iter()
                    .position(|x| x == &src)
                    .expect("Could not find.");

                self.0[des_index].0.push(Node {
                    data: src,
                    weight,
                    next: None,
                });
                self.0[src_index].0.push(Node {
                    data: des,
                    weight,
                    next: None,
                });
            }
        }
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

impl<'a, T> fmt::Display for Graph<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
}
