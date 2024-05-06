use crate::graph::{Vertex, Graph};
use std::collections::VecDeque;

impl Graph{
    // Breadth-First Search (BFS) algorithm
    pub fn bfs(&self, start: Vertex) -> Vec<Option<u32>> {
        let mut distances: Vec<Option<u32>> = vec![None; self.n];
        distances[start] = Some(0);
        let mut next: VecDeque<Vertex> = VecDeque::new();
        next.push_back(start);
        while let Some(v) = next.pop_front() {
            for u in &self.outer[v] {
                if distances[*u].is_none() {
                    distances[*u] = Some(distances[v].unwrap_or(0)+1);
                    next.push_back(*u);
                }
            }
        }
        distances
    }
    
    // Calculates average distance between all pairs of vertices in the graph
    pub fn average_distance(&self) -> f64 {
        let mut total = 0; 
        let mut num = 0;
        for start in 0..self.n {
            let distances = self.bfs(start);
            for end in 0..self.n {
                if let Some(dist) = distances[end] {
                    total += dist;
                    num += 1;
                }
            }
        }
        if num > 0 {
            total as f64 / num as f64
        } else {
            0.0
        }
    }
    
    // Calculates average number of friends per vertex (degree of graph)
    pub fn average_friends(&self) -> usize {
        let mut sum = 0;
        let nodes = self.n;
        for i in &self.outer {
            sum += i.len();
        }
        sum / (2*nodes) 
        // Graph is undirected, each edge is counted twice
        // Must divide by 2 for accurate # of unique edges
    }

    // Finds the vertex with the most number of friends in graph (highest degree in graph)
    pub fn most_friends(&self) -> Option<Vertex> {
        let most_popular = (0..self.n)
        .max_by_key(|&v| self.outer[v].len())
        .map(|v| v);
        most_popular
    }

    // Finds the vertex with the fewest number of friends in the graph (lowest degree in graph)
    pub fn fewest_friends(&self) -> Option<Vertex> {
        let least_popular = (0..self.n)
        .min_by_key(|&v| self.outer[v].len())
        .map(|v| v);
        least_popular
    }
       
}
