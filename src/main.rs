pub mod dijkstra;
use std::collections::LinkedList;

use dijkstra::{shortest_path, Graph};

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

    let graphCormen = Graph {
        vertexes: Vec::from([
            LinkedList::from([(1, 10), (4, 5)]), // 0
            LinkedList::from([(2, 1), (4, 2)]),  // 1
            LinkedList::from([(3, 4)]),          // 2
            LinkedList::from([(0, 7), (2, 6)]),  // 3
            LinkedList::from([(1,3),(2,9),(3,2)]),              // 4
        ]),
    };

    let (dist, path) = shortest_path(graphCormen, 0, 1);

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
