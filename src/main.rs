use graph::*;

fn main() {
    let a = Graph::new("hello", 2);
    let b = Graph::with_capacity("world!", 5, 50); 

    println!("{}", a.get_node()[0].data);
    println!("{}", b.get_node().capacity());
    println!("{}", a.get_node()[0]);
}
