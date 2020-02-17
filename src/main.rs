use graph::Graph;

fn main() {
    let record = ["John", "Dan", "Riley", "Faith", "Logan"];
    let mut gdata = Graph::<&str>::new(&record[..]);
    gdata.add_edge("Logan", "John", false, 0);
}
