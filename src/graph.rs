use std::collections::{HashMap, HashSet};

pub struct Graph {
    adj_list: HashMap<i32, Vec<i32>>,
}

impl  Graph {
    // Creates a new graph
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    // Adds a node to the graph
    pub fn add_node(&mut self, node: i32) {
        self.adj_list.entry(node).or_insert(Vec::new());
    }

    // Adds an edge between two nodes
    pub fn add_edge(&mut self, node1: i32, node2: i32) {
        self.adj_list.entry(node1).or_insert(Vec::new()).push(node2);
        self.adj_list.entry(node2).or_insert(Vec::new()).push(node1);
    }

    // Traverse the search using DFS and return nodes in the order they were visited
    pub fn traverse(&self, start: i32) -> Vec<i32>{
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs(start, &mut visited, &mut result);
        result
    }

    // Helper functions for DFS traversal
    fn dfs(&self, node: i32, visited: &mut HashSet<i32>, result: &mut Vec<i32>) {
        if !visited.insert(node) {
            return;
        }
        result.push(node);
        if let Some(neighbors) = self.adj_list.get(&node) {
            for &neighbor in neighbors {
                self.dfs(neighbor, visited, result);
            }
        }
    }
}