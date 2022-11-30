pub mod bellmanford;
pub mod common;
pub mod dijkstra;
use std::collections::LinkedList;

use dijkstra::shortest_path;

use crate::common::Graph;

fn main() {
    let graph_cormen = Graph {
        vertexes: Vec::from([
            LinkedList::from([(1, 10), (4, 5)]),        // 0
            LinkedList::from([(2, 1), (4, 2)]),         // 1
            LinkedList::from([(3, 4)]),                 // 2
            LinkedList::from([(0, 7), (2, 6)]),         // 3
            LinkedList::from([(1, 3), (2, 9), (3, 2)]), // 4
        ]),
    };

    let (dist, path) = shortest_path(graph_cormen, 0, 1);

    println!("best dist = {}", dist);
    print!("Shortest path = ");

    if path.len() > 1 {
        for vertex in path {
            print!("{} ", vertex);
        }
        print!("\n");
    } else {
        println!("No path");
    }
}
