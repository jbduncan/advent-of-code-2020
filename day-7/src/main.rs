use petgraph::prelude::*;
use petgraph::visit::Walker;

fn main() {
    let mut graph = DiGraphMap::<i32, ()>::new();
    graph.add_edge(-1, 2, ());
    graph.add_edge(-1, 3, ());
    graph.add_edge(-1, 5, ());
    graph.add_edge(5, 8, ());

    let bfs = Bfs::new(&graph, -1);
    let results: Vec<_> = bfs.iter(&graph).collect();
    println!("{:?}", results);
}
