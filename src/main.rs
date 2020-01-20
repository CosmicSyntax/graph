use graph::*;

fn main() {
    let a = Graph::new("hello", 2);
    let b = Graph::with_capacity("world!", 5, 50); 

    println!("{}", a[0].data);
    println!("{}", b.capacity());
    println!("{}", a[0]);
}
