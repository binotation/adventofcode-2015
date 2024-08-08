/// This is a Hamiltonian path problem. We know that multiple Hamiltonian
/// paths exist and want to determine the length of the shortest one.
use rustc_hash::FxHashMap;

/// Use a variation of the dynamic programming approach to solving the Hamiltonian path problem.
/// https://www.geeksforgeeks.org/hamiltonian-path-using-dynamic-programming/
#[allow(dead_code)]
fn length_hamiltonian_paths<F>(adj: Vec<Vec<u32>>, comparator: F) -> Vec<Vec<Option<u32>>>
where
    F: Fn(u32, u32) -> u32,
{
    let n: usize = adj.len();
    // Represents length of shortest Hamiltonian path of each subset of nodes ending with node i
    let mut dp = vec![vec![None; n]; 1 << n];

    // Initialize trivial case: every subset containing exactly one node has a Hamiltonian path of length 0.
    for node in 0..n {
        dp[1 << node][node] = Some(0);
    }

    for subset in 0..(1 << n) {
        // For each node in subset
        for node in (0..n).filter(|node| subset & (1 << node) > 0) {
            // For each neighbor of node
            for neighbor in (0..n).filter(|neighbor| {
                subset & (1 << neighbor) > 0 // neighbor in subset
                    && node != *neighbor
                    && adj[*neighbor][node] > 0 // neighbor is connected to node
            }) {
                // If the subset excluding node contains a hamiltonian path ending with neighbor
                if dp[subset ^ (1 << node)][neighbor].is_some() {
                    // Update length of path ending with node if neighbor path + dist[neighbor][node] is shorter
                    if let Some(d) = dp[subset][node] {
                        dp[subset][node] = Some(comparator(
                            d,
                            dp[subset ^ (1 << node)][neighbor].unwrap() + adj[neighbor][node],
                        ));
                    } else {
                        dp[subset][node] =
                            Some(dp[subset ^ (1 << node)][neighbor].unwrap() + adj[neighbor][node]);
                    }
                }
            }
        }
    }
    dp
}

/// Build the adjacency matrix adj. This tells us the distance between every node.
/// It is symmetric because the graph is undirected.
#[allow(dead_code)]
fn build_adjacency_matrix(locations: &str) -> Vec<Vec<u32>> {
    let mut adj: Vec<Vec<u32>> = Vec::new();
    // Asign each location name an index starting with 0
    let mut index = 0;
    let mut locations_index: FxHashMap<&str, usize> = FxHashMap::default();

    for line in locations.lines() {
        // Line example: "AlphaCentauri to Snowdin = 66"
        let mut line_split = line.split(" = ");
        let location_names = line_split.next().unwrap();
        let mut location_names_split = location_names.split(" to ");
        let location1 = location_names_split.next().unwrap();
        let location2 = location_names_split.next().unwrap();
        let distance: u32 = line_split.next().unwrap().parse::<u32>().unwrap();

        // Assign location name an index if unassigned
        if !locations_index.contains_key(location1) {
            locations_index.insert(location1, index);
            // Add row into adj, column is not needed because of the special ordering of the input text
            adj.push(vec![0; index + 1]);
            index += 1;
        }
        if !locations_index.contains_key(location2) {
            locations_index.insert(location2, index);
            // Add row and column into adj
            adj.push(vec![0; index + 1]);
            for row in adj.iter_mut().take(index) {
                row.push(0);
            }
            index += 1;
        }

        // Set distance
        let location1_index = locations_index.get(location1).unwrap();
        let location2_index = locations_index.get(location2).unwrap();
        adj[*location1_index][*location2_index] = distance;
        adj[*location2_index][*location1_index] = distance;
    }
    adj
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn get_length_of_shortest_hamiltonian_path() {
        let locations = get_input("locations").unwrap();
        let adj = build_adjacency_matrix(&locations);
        let n = adj.len();
        let dp = length_hamiltonian_paths(adj, std::cmp::min);
        // Retrieve shortest path length from subset containing all nodes
        let length_shortest_path = dp[(1 << n) - 1]
            .iter()
            .map(|v| v.unwrap()) // Path ending with all nodes will exist
            .min()
            .unwrap();
        assert_eq!(length_shortest_path, 141);
    }

    #[test]
    fn get_length_of_longest_hamiltonian_path() {
        let locations = get_input("locations").unwrap();
        let adj = build_adjacency_matrix(&locations);
        let n = adj.len();
        let dp = length_hamiltonian_paths(adj, std::cmp::max);
        // Retrieve longest path length from subset containing all nodes
        let length_longest_path = dp[(1 << n) - 1]
            .iter()
            .map(|v| v.unwrap()) // Path ending with all nodes will exist
            .max()
            .unwrap();
        assert_eq!(length_longest_path, 736);
    }

    #[test]
    fn test_length_of_shortest_hamiltonian_path_simple() {
        let adj = vec![vec![0, 464, 518], vec![464, 0, 141], vec![518, 141, 0]];
        let n = adj.len();
        let dp = length_hamiltonian_paths(adj, std::cmp::min);
        // Retrieve shortest path length from subset containing all nodes
        let length_shortest_path = dp[(1 << n) - 1]
            .iter()
            .map(|v| v.unwrap()) // Path ending with all nodes will exist
            .min()
            .unwrap();
        assert_eq!(length_shortest_path, 605);
    }
}
