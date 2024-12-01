use std::collections::HashMap;

/// An undirected graph implementation.
#[derive(Debug)]
pub struct UndirectedGraph {
    nodes: Vec<String>,
    edges: Vec<(String, String, i32)>,
}

impl UndirectedGraph {
    /// Creates a new graph.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }

    /// Adds a node to the graph.
    /// Will not add the node if it already exists and will not throw an error.
    /// 
    /// # Arguments
    /// 
    /// - `node` - The node to add to the graph.
    pub fn add_node(&mut self, node: &str) {
        if !self.nodes.contains(&node.to_string()) {
            self.nodes.push(node.to_string());
        }
    }

    /// Gets the nodes of the graph.
    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    /// Adds an edge to the graph.
    /// 
    /// # Arguments
    /// 
    /// - `from` - The node the edge is from.
    /// - `to` - The node the edge is to.
    /// - `weight` - The weight of the edge.
    pub fn add_edge(&mut self, from: &str, to: &str, weight: i32) {
        // FIXME: This is not save, as it will add the same edge twice in case
        // there are different weights.

        if !self.edges.contains(&(from.to_string(), to.to_string(), weight)) {
            self.edges.push((from.to_string(), to.to_string(), weight));
            self.edges.push((to.to_string(), from.to_string(), weight));
        }
    }

    /// Gets the edges of a node.
    /// 
    /// # Arguments
    /// 
    /// - `node` - The node to get the edges of.
    pub fn get_edges(&self, node: &str) -> Vec<(String, i32)> {
        self.edges.iter()
            .filter(|(from, _, _)| from == node)
            .map(|(_, to, weight)| (to.clone(), *weight))
            .collect()
    }

    /// Finds the shortest path from a node to all other possible nodes by using
    /// a bfs algorithm.
    /// 
    /// # Arguments
    /// 
    /// - `from` - The node to start from.
    pub fn find_shortest_path(&self, from: &str) -> (Vec<String>, i32) {
        let mut shortest_path = vec![];
        let mut current = from.to_string();
        let mut total_weight = 0;

        shortest_path.push(from.to_string());

        loop {
            let edges = self.get_edges(&current);

            let mut next = None;
            let mut next_weight = i32::MAX;

            for (node, weight) in edges {
                if !shortest_path.contains(&node) && weight < next_weight {
                    next = Some(node);
                    next_weight = weight;
                }
            }

            if let Some(node) = next {
                total_weight += next_weight;

                shortest_path.push(node.clone());
                current = node;
            } else {
                break;
            }
        }

        (shortest_path, total_weight)
    }

    /// Finds the longest path from a node to all other possible nodes by using
    /// a dfs algorithm.
    /// 
    /// # Arguments
    /// 
    /// - `start` - The node to start from.
    pub fn longest_path(&self, start: &str) -> (Vec<String>, i32) {
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        let mut max_path = Vec::new();
        let mut max_weight = 0;

        self.dfs(
            start, &mut visited, &mut path, 0,
            &mut max_path, &mut max_weight);

        (max_path, max_weight)
    }

    /// Depth-first search algorithm.
    /// 
    /// # Arguments
    /// 
    /// - `node` - The node to start from.
    /// - `visited` - The nodes that have been visited.
    /// - `path` - The current path.
    /// - `current_weight` - The current weight.
    /// - `max_path` - The longest path.
    /// - `max_weight` - The longest path weight.
    pub fn dfs(
        &self, node: &str, visited: &mut HashMap<String, bool>, 
        path: &mut Vec<String>, current_weight: i32, max_path: &mut Vec<String>,
        max_weight: &mut i32,
    ) {
        visited.insert(node.to_string(), true);
        path.push(node.to_string());

        if current_weight > *max_weight {
            *max_weight = current_weight;
            *max_path = path.clone();
        }

        for (neighbor, weight) in self.get_edges(node).iter() {
            if !visited.get(neighbor).unwrap_or(&false) {
                self.dfs(
                    neighbor, visited, path, 
                    current_weight + weight,
                    max_path, max_weight);
            }
        }

        path.pop();
        visited.insert(node.to_string(), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_should_add_two_nodes() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");

        assert_eq!(
            graph.nodes,
            vec!["London".to_string(), "Dublin".to_string()]);
    }

    #[test]
    fn get_nodes_should_return_all_added_nodes() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");

        assert_eq!(
            graph.get_nodes(),
            vec!["London".to_string(), "Dublin".to_string()]);
    }

    #[test]
    fn add_edge_should_add_edge() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");

        graph.add_edge("London", "Dublin", 464);

        assert_eq!(
            graph.edges,
            vec![
                ("London".to_string(), "Dublin".to_string(), 464),
                ("Dublin".to_string(), "London".to_string(), 464)]);
    }

    #[test]
    fn get_edges_should_return_edges_from_source_node() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");

        graph.add_edge("London", "Dublin", 464);

        assert_eq!(
            graph.get_edges("London"), vec![("Dublin".to_string(), 464)]);
    }

    #[test]
    fn find_shortest_path_should_return_shortest_path() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");
        graph.add_node("Paris");
        graph.add_node("Berlin");

        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("Dublin", "Paris", 500);
        graph.add_edge("Paris", "Berlin", 600);

        assert_eq!(
            graph.find_shortest_path("London"),
            (vec![
                "London".to_string(), "Dublin".to_string(), "Paris".to_string(),
                "Berlin".to_string()],
            1564));
    }


    #[test]
    fn longest_path_should_return_longest_path() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");
        graph.add_node("Paris");
        graph.add_node("Berlin");

        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("Dublin", "Paris", 500);
        graph.add_edge("Paris", "Berlin", 600);

        assert_eq!(
            graph.longest_path("London"),
            (vec![
                "London".to_string(), "Dublin".to_string(), "Paris".to_string(),
                "Berlin".to_string()],
            1564));
    }

    #[test]
    fn dfs_should_return_longest_path() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");
        graph.add_node("Paris");
        graph.add_node("Berlin");

        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("Dublin", "Paris", 500);
        graph.add_edge("Paris", "Berlin", 600);

        assert_eq!(
            graph.longest_path("London"),
            (vec![
                "London".to_string(), "Dublin".to_string(), "Paris".to_string(),
                "Berlin".to_string()],
            1564));
    }
}
