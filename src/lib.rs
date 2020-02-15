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

pub struct Graph<'a ,T>(Vec<Vector<T>>, &'a [T]);

/// Trait to restrict what the generic could be, and the following lines
/// of code also allows the user to the turbo fish ::<>, instead of type
/// annotation.
pub trait Restriction<'a, T> {
    fn alloc(init_size: usize, template: &'a [T]) -> Graph<'a, Self>
    where
        Self: Sized;
}

// macro to expand to include 6 data types
macro_rules! make_restriction {
    ($($datatype:ty),*) => {
        $(
            impl<'a> Restriction<'a, $datatype> for $datatype {
                fn alloc(init_size: usize, template: &'a [$datatype]) -> Graph<'a, Self> {
                    let data: Vec<Vector<$datatype>> = Vec::with_capacity(init_size);
                    Graph(data, template)
                }
            }
        )*
    };
}
make_restriction![u8, u32, u64, f32, f64, String];

/// Methods for Vector data
impl<'a, T: Restriction<'a, T>> Graph<'a, T> {
    pub fn new(init_size: usize, template: &'a [T]) -> Graph<'a, T> {
        Restriction::alloc(init_size, template)
    }

    //pub fn get_node(&self) -> &Vec<Node<T>> {
    //&self.0
    //}

    pub fn add_edge(&mut self, src: T, des: T, dir: bool, weight: u8) {
        match dir {
            true => {
            },
            false => {
               let n = self.0.len();
               if n == self.0.capacity() { panic!("There is no more space left on the vector.") };


            },
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
