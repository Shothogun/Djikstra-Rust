pub mod djikstra;
use std::collections::LinkedList;

use djikstra::{shortest_path, Graph};

fn main() {
    let graph = Graph {
        vertexes: Vec::from([
            LinkedList::from([(1, 5), (2, 6)]), // 0
            LinkedList::from([(4, 100)]),       // 1
            LinkedList::from([(3, 5)]),         // 2
            LinkedList::from([(4, 7)]),         // 3
            LinkedList::from([]),               // 4
        ]),
    };

    let (dist, path) = shortest_path(graph, 0, 4);

    println!("best dist = {}", dist);
    print!("Shortest path = ");

    if path.len() > 1 {
        for vertex in path {
            print!("{} ", vertex);
        }
    } else {
        println!("No path");
    }
}
