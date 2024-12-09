use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Graph struct to store the adjacency list where each node maps to a vector of neighboring nodes
pub struct Graph {
    adjacency_list: HashMap<u32, Vec<u32>>,
} 

// HashMap to store each node’s connections and Vec for neighbors to allow efficient traversal
impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.adjacency_list.entry(u).or_insert_with(Vec::new).push(v);
        self.adjacency_list.entry(v).or_insert_with(Vec::new).push(u);
    }
    
    //load_graph_from_file reads the file, processes each line and populates the graph

    pub fn load_graph_from_file(file_path: &str) -> io::Result<Self> { 
        let mut graph = Graph::new();
        let file = File::open(Path::new(file_path))?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<u32> = line.split_whitespace()
                                      .filter_map(|s| s.parse().ok())
                                      .collect();
            if parts.len() == 2 {
                graph.add_edge(parts[0], parts[1]);
            }
        }

        Ok(graph)
    }

    pub fn neighbors(&self, node: u32) -> Option<&Vec<u32>> {
        self.adjacency_list.get(&node)
    }
}
