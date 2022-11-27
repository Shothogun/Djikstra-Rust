use priority_queue::PriorityQueue;
// Max size of the grapth's vertex amount
const N: i64 = 100002;

struct DjikstraData {
    // Array of distances from start vertex to each vertex indexed
    distances: [i64; N as usize],

    // A matrix that represents the input graph,
    // where the 1st dimension indicates the vertices
    // and the 2nd the associated vertex and its
    // edge weight (vertice_index,weight)
    graph: Vec<Vec<(i64, i64)>>,

    // List of vertex from the path between
    // start to end vertex
    predecessors: [i64; N as usize],

    // Priority queue sorted by the weight associated
    // to the vertex:
    // item = vertex's index
    // priority = vertex's weigth
    // see documentation https://docs.rs/priority-queue/latest/priority_queue/priority_queue/struct.PriorityQueue.html
    pq: PriorityQueue<i64, i64>,
}

fn djikstra(start: i64, data: DjikstraData) -> (i64, PriorityQueue<i64, i64>, [i64; N as usize]) {
    (start, data.pq, data.predecessors)
}

fn main() {
    let mut g: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0)]; N as usize];
    let mut data: DjikstraData = DjikstraData {
        graph: g,
        predecessors: [0; N as usize],
        pq: PriorityQueue::new(),
    };

    djikstra(2, data);

    println!("Hello good!")
}
