use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

use crate::undirected_graph::UndirectedGraph;

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: i32,
    position: String,
}

// implement Ord and PartialOrd for the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse order for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implements Dijkstra's algorithm.
/// Created by ChatGPT: https://chatgpt.com/c/67496bd6-74b0-8008-9b71-6871ce1613e5
/// 
/// # Arguments
/// 
/// - `graph` - the undirected graph
/// - `start` - the starting node
/// - `goal` - the end node
pub fn dijkstra(graph: &UndirectedGraph, start: &str, goal: &str)
    -> Option<(Vec<String>, i32)>
{
    // min-heap for priority queue
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut distances: HashMap<String, i32> = HashMap::new();
    let mut predecessors: HashMap<String, String> = HashMap::new();

    // initialize distances to all nodes as infinity, except the start node
    for node in graph.get_nodes().iter() {
        distances.insert(node.clone(), i32::MAX);
    }
    distances.insert(start.to_string(), 0);
    
    // push the start node into the heap
    heap.push(State { cost: 0, position: start.to_string() });

    while let Some(State { cost, position }) = heap.pop() {
        // if we reached the goal, construct the path
        if position == goal {
            let mut path = vec![];
            let mut current = goal.to_string();

            while let Some(pred) = predecessors.get(&current) {
                path.push(current);
                current = pred.clone();
            }

            path.push(start.to_string());
            path.reverse();

            return Some((path, cost));
        }

        // skip if we've already found a better way
        if cost > *distances.get(&position).unwrap() {
            continue;
        }

        // explore neighbors
        for (to, weight_to) in graph.get_edges(&position) {
            let next = State {
                cost: cost + weight_to,
                position: to.clone(),
            };

            if next.cost < *distances.get(&to).unwrap_or(&i32::MAX) {
                distances.insert(to.clone(), next.cost);
                predecessors.insert(to, position.clone());
                heap.push(next);
            }
        }
    }

    // if the goal is not reachable, return None
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_should_return_shortest_path_for_single_node() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");

        assert_eq!(
            dijkstra(&graph, "London", "London"),
            Some((vec!["London".to_string()], 0)));
    }

    #[test]
    fn dijkstra_should_return_shortest_path_for_two_nodes() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");

        graph.add_edge("London", "Dublin", 464);

        assert_eq!(
            dijkstra(&graph, "London", "Dublin"),
            Some((vec!["London".to_string(), "Dublin".to_string()], 464)));
    }

    #[test]
    fn dijkstra_should_return_shortest_path_for_three_nodes() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");
        graph.add_node("Paris");

        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("Dublin", "Paris", 500);

        assert_eq!(
            dijkstra(&graph, "London", "Paris"),
            Some((vec![
                "London".to_string(), "Dublin".to_string(), "Paris".to_string()],
            964)));
    }

    #[test]
    fn dijkstra_should_return_shortest_path() {
        let mut graph = UndirectedGraph::new();

        graph.add_node("London");
        graph.add_node("Dublin");
        graph.add_node("Paris");
        graph.add_node("Berlin");

        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("Dublin", "Paris", 500);
        graph.add_edge("Dublin", "Berlin", 1200);
        graph.add_edge("Paris", "Berlin", 600);

        assert_eq!(
            dijkstra(&graph, "London", "Berlin"),
            Some((vec![
                "London".to_string(), "Dublin".to_string(), "Paris".to_string(),
                "Berlin".to_string()],
            1564)));
    }
}
