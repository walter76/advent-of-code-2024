use std::collections::HashMap;

/// A directed graph implementation.
#[derive(Debug)]
pub struct DirectedGraph {
    adjacency_list: HashMap<String, Vec<(String, i32)>>,
}

impl DirectedGraph {
    /// Creates a new graph.
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    /// Adds a node to the graph.
    /// Will not add the node if it already exists and will not throw an error.
    /// 
    /// # Arguments
    /// 
    /// - `node` - The node to add to the graph.
    pub fn add_node(&mut self, node: &str) {
        self.adjacency_list.entry(node.to_string()).or_insert(vec![]);
    }

    /// Adds an edge to the graph.
    /// Will not add the edge if it already exists and will not throw an error.
    /// 
    /// # Arguments
    /// 
    /// - `from` - The node the edge is from.
    /// - `to` - The node the edge is to.
    /// - `weight` - The weight of the edge.
    pub fn add_edge(&mut self, from: &str, to: &str, weight: i32) {
        self.adjacency_list
            .entry(from.to_string())
            .or_insert(vec![])
            .push((to.to_string(), weight));
    }

    /// Gets the edges of a node.
    /// 
    /// # Arguments
    /// 
    /// - `node` - The node to get the edges of.
    pub fn get_edges(&self, node: &str) -> Option<&Vec<(String, i32)>> {
        self.adjacency_list.get(node)
    }
}
