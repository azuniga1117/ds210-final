mod reader;
mod graph;
mod analysis;

fn main() {
    let file_path = "facebook_combined.txt";
    let n: usize = 4039; //Taken from dataset statistics 
    let edges = reader::read_txt(file_path); // Create list of edges from file
    let graph = graph::Graph::create_undirected_graph(n, &edges); // Creates graph from edges and n of dataset
    
    let avg_distance = graph.average_distance();
    println!("The average distance between Facebook friends is {:.2}.\n", avg_distance);

    if let Some(most_popular) = graph.most_friends() {
        println!("The most popular user is represented by node #{:?}\n", most_popular);
    } else {
        println!("Error: graph is empty. ");
    };

    if let Some(least_popular) = graph.fewest_friends() {
        println!("The least popular user is represented by node #{:?}\n", least_popular);
    } else {
        println!("Error: graph is empty:");
    }

    let average_facebook_friends = graph.average_friends();
    println!("The average number of Facebook friends per user is {} friends\n", average_facebook_friends);

}

#[cfg(test)]
mod tests {
    use self::graph::Graph;
    use super::*;

    // Test case for average distance calculation
    #[test]
    fn test_average_distance() {
        let test_n: usize = 4;
        let test_edges: Vec<(usize, usize)> = vec![(0,1),(1,2),(2,3)];
        let test_graph = graph::Graph::create_undirected_graph(test_n, &test_edges);
        let test_average_distance = graph::Graph::average_distance(&test_graph);
        assert!(test_average_distance > 0.0, "Average distance cannot be calculated");

    }

    // Test case for average number of friends calculation
    #[test]
    fn test_average_friends() {
        let test_n: usize = 4;
        let test_edges: Vec<(usize, usize)> = vec![(0,1), (1,2), (2,3), (3,0)];
        let test_graph = Graph::create_undirected_graph(test_n, &test_edges);
        assert_eq!(test_graph.average_friends(), 1);
    }

     // Test case for popularity analysis
    #[test]
    fn test_popularity() {
        let test_n: usize = 4;
        let test_edges: Vec<(usize, usize)> = vec![(0,1),(0,2),(0,3),(1,3)];
        let test_graph = Graph::create_undirected_graph(test_n, &test_edges);
        assert_eq!(test_graph.most_friends(), Some(0));
        assert_eq!(test_graph.fewest_friends(), Some(2));
    }

}
