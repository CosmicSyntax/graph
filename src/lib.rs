/// This is a struct used to represent each node
pub struct Node<T> {
    pub data: T,
    weight: Option<u8>,
    next: Option<Box<Node<T>>>,
}

pub struct Graph<T>(Vec<Node<T>>);

impl<T> Graph<T> {
    pub fn new(data: T, weight: Option<u8>) -> Vec<Node<T>> {
        vec![
            Node {
                data,
                weight,
                next: None,
            }
        ]
    }

    pub fn with_capacity(data: T, weight: Option<u8>, capacity: usize)
    -> Vec<Node<T>>
    {
        let mut node: Vec<Node<T>> = Vec::with_capacity(capacity);
        node.push(Node {
            data,
            weight,
            next: None,
        });
        node
    }
}
