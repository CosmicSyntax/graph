use graph::*;

fn main() {
    let a = Graph::new("hello", Some(2));
    let b = Graph::with_capacity("world!", Some(5), 50); 

    println!("{}", a[0].data);
    println!("{}", b.capacity());
}
