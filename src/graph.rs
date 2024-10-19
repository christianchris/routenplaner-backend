use std::fs::File;

#[derive(Clone)]
struct Vertice {
    first_edge_idx: usize,
}
#[derive(Clone)]
struct Edge {
    //TODO testen, ob casting von kleineren typen akzeptabel ist
    from: usize,
    to: usize,
    cost: f32,
}
pub struct Graph<const NUM_V: usize, const NUM_E: usize> {
    vertices: [Vertice; NUM_V],
    edges: [Edge; NUM_E],
}

impl<const NUM_V: usize, const NUM_E: usize> Graph<NUM_V, NUM_E> {
    pub fn new(graph_file: File) -> Result<Self, &'static str> {
        todo!()
    }
    pub fn shortest_path(&self, from: usize, to: usize) -> Result<Vec<usize>, &'static str> {
        todo!()
    }
    pub fn shortest_to_all(&self, from: usize, prev_buffer: &mut [usize; NUM_V]) {
        todo!()
    }
    pub fn k_nearest_neighbour(&self, from: (f32, f32)) -> usize {
        todo!()
    }
}
