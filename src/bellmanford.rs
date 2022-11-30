use crate::{common::find_path_from_prev, common::Graph, common::INFINITY};

use std::collections::LinkedList;

pub fn shortest_path(graph: &Graph, source: usize, target: usize) -> (u64, LinkedList<usize>) {
    let mut edges: LinkedList<(usize, usize, u64)> = LinkedList::new();

    for (u, u_edges) in graph.vertexes.iter().enumerate() {
        for &(v, w) in u_edges {
            edges.push_back((u, v, w));
        }
    }

    let n = graph.vertexes.len();
    let mut dist = vec![INFINITY; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];

    dist[source] = 0;

    for _ in 1..n {
        for &(u, v, w) in &edges {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                prev[v] = Some(u);
            }
        }
    }

    return (dist[target], find_path_from_prev(prev, target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_shortest_path() {
        let graph = Graph {
            vertexes: Vec::from([
                LinkedList::from([(1, 5), (2, 6)]), // 0
                LinkedList::from([(4, 100)]),       // 1
                LinkedList::from([(3, 5)]),         // 2
                LinkedList::from([(4, 7)]),         // 3
                LinkedList::from([]),               // 4
            ]),
        };

        let (dist, path) = shortest_path(&graph, 0, 4);

        assert_eq!(dist, 18);
        assert_eq!(path, LinkedList::from([0, 2, 3, 4]));
    }

    #[test]
    fn should_find_shortest_path_cormen() {
        let graph = Graph {
            vertexes: Vec::from([
                LinkedList::from([(1, 10), (4, 5)]),        // 0
                LinkedList::from([(2, 1), (4, 2)]),         // 1
                LinkedList::from([(3, 4)]),                 // 2
                LinkedList::from([(0, 7), (2, 6)]),         // 3
                LinkedList::from([(1, 3), (2, 9), (3, 2)]), // 4
            ]),
        };

        let (dist, path) = shortest_path(&graph, 0, 1);

        assert_eq!(dist, 8);
        assert_eq!(path, LinkedList::from([0, 4, 1]));
    }

    #[test]
    fn should_not_find_shortest_path() {
        let graph = Graph {
            vertexes: Vec::from([
                LinkedList::from([(1, 5), (2, 6)]), // 0
                LinkedList::from([(4, 100)]),       // 1
                LinkedList::from([(3, 5)]),         // 2
                LinkedList::from([(4, 7)]),         // 3
                LinkedList::from([]),               // 4
            ]),
        };

        let (dist, path) = shortest_path(&graph, 4, 0);

        assert_eq!(dist, INFINITY);
        assert_eq!(path, LinkedList::from([0]));
    }
}
