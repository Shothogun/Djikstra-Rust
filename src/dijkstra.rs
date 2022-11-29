use std::{cmp::Reverse, collections::LinkedList};

use priority_queue::PriorityQueue;

const INFINITY: u64 = 1000;

pub struct Graph {
    // Every vertex has a list of (Node, Weight)
    pub vertexes: Vec<LinkedList<(usize, u64)>>,
}

pub fn shortest_path(graph: Graph, source: usize, target: usize) -> (u64, LinkedList<usize>) {
    let n = graph.vertexes.len();
    let mut dist = vec![INFINITY; n];
    let mut visited = vec![false; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    let mut pq: PriorityQueue<usize, Reverse<u64>> = PriorityQueue::new();

    // Initial values for source
    dist[source] = 0;
    pq.push(source, Reverse(0));

    // Runs |V|+1 times, takes Tpop
    while let Some((u, _)) = pq.pop() {
        // Runs |V| times, takes 1
        visited[u] = true;

        // Runs |V| times, takes 1
        println!("Visiting {}", u);

        // Runs |V| * (|Ev|+1) times, takes 1
        for &(v, w) in &graph.vertexes[u] {
            // Runs |V| * |Ev| times, takes 1
            println!(
                "Checking edge from u={} to v={} with weight w={}, dist[u] = {}, dist[v] = {}",
                u, v, w, dist[u], dist[v]
            );

            // Runs |V| * |Ev| times, takes 1
            if dist[v] > dist[u] + w {
                // Runs |V| * |Ev| times, takes 1
                dist[v] = dist[u] + w;

                // Runs |V| * |Ev| times, takes 1
                prev[v] = Some(u);

                // Runs |V| * |Ev| times, takes 1
                println!("Updated dist[{}] to {}", v, dist[v]);

                // Runs |V| * |Ev| times, takes 1
                if !visited[v] {
                    // Runs |V| * |Ev| times, takes Tpush
                    pq.push(v, Reverse(dist[v]));
                }
            }
        }
    }

    // Compute path to target
    let mut path: LinkedList<usize> = LinkedList::from([target]);
    let mut current = target;
    while let Some(&Some(vertex)) = prev.get(current) {
        path.push_front(vertex);
        current = vertex;
    }

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
        assert_eq!(path, LinkedList::from([0, 2, 3, 4]));
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
        assert_eq!(path, LinkedList::from([0]));
    }
}
