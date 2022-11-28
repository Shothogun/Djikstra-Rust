use std::{cmp::Reverse, collections::LinkedList};

use priority_queue::PriorityQueue;

const INFINITY: u64 = 1000;

pub struct Graph {
    // Every vertex has a list of (Node, Weight)
    pub vertexes: Vec<LinkedList<(usize, u64)>>,
}

pub fn shortest_path(graph: Graph, source: usize, target: usize) -> (u64, Vec<usize>) {
    let n = graph.vertexes.len();
    let mut dist = vec![INFINITY; n];
    let mut visited = vec![false; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    let mut pq: PriorityQueue<usize, Reverse<u64>> = PriorityQueue::new();

    // Initial values for source
    dist[source] = 0;
    pq.push(source, Reverse(0));

    loop {
        match pq.pop() {
            Some((u, _)) => match visited.get(u) {
                Some(false) => {
                    visited[u] = true;
                    println!("Visiting {}", u);

                    for (v, w) in &graph.vertexes[u] {
                        println!(
                            "Checking edge from u={} to v={} with weight w={}, dist[u] = {}, dist[v] = {}",
                            u, v, w, dist[u], dist[*v]
                        );
                        if dist[*v] > dist[u] + w {
                            dist[*v] = dist[u] + w;
                            prev[*v] = Some(u);

                            println!("Updated dist[{}] to {}", v, dist[*v]);

                            pq.push(*v, Reverse(dist[*v]));
                        }
                    }
                }
                _ => {}
            },
            None => break,
        }
    }

    let mut path: Vec<usize> = Vec::from([target]);
    let mut current = target;
    loop {
        match prev.get(current) {
            Some(Some(vertex)) => {
                path.push(*vertex);
                current = *vertex;
            }
            _ => {
                break;
            }
        }
    }
    path.reverse();

    return (dist[target], path);
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

        let (dist, path) = shortest_path(graph, 0, 4);

        assert_eq!(dist, 18);
        assert_eq!(path, vec![0, 2, 3, 4]);
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

        let (dist, path) = shortest_path(graph, 4, 0);

        assert_eq!(dist, INFINITY);
        assert_eq!(path, vec![0]);
    }
}
