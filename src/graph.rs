pub type Vertex = usize; // Type alias for vertex in graph
pub type EdgesList = Vec<(Vertex, Vertex)>; // Type alias for a list of edges, represented as a tuple of vertices
pub type AdjacencyList = Vec<Vec<Vertex>>; // Type alias for adjacency list representation of graph

#[derive(Debug)]
pub struct Graph {
    pub n: usize,   // Number of vertices in graph
    pub outer: AdjacencyList,   // Outer representation of graph
}

// Function to reverse direction of a list of edges to help create undirected graph
pub fn reverse_edges(list: &EdgesList) -> EdgesList {
    let mut reversed = vec![];
    for (u, v) in list {
        reversed.push((*v, *u));
    }
    reversed
}

impl Graph {
    
    // Adds directed edges to graph
    pub fn add_directed_edges(&mut self, edges: &EdgesList) {
        for (u, v) in edges {
            self.outer[*u].push(*v);
        }
    }

    // Sorts the adjacency lists of all vertices in graph
    pub fn sort_graphs(&mut self) {
        for line in self.outer.iter_mut() {
            line.sort();
        }
    }

    // Method to create directed graph from a number of vertices and a list of edges
    pub fn create_directed_graph(n: usize, edges:&EdgesList) -> Graph {
        let mut directed = Graph {
            n,
            outer: vec![vec![];n],
        };
        directed.add_directed_edges(edges);
        directed.sort_graphs();
        directed
    }

    // Creates undirected graph from given number of vertices and list of edges 
    pub fn create_undirected_graph(n:usize, edges:&EdgesList) -> Graph {
        let mut undirected = Self::create_directed_graph(n, edges);
        undirected.add_directed_edges(&reverse_edges(edges));
        undirected.sort_graphs();
        undirected
    }

}
