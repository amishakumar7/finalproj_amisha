use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


//Graph struct to store the adjacency list, where each node maps to a vector of neighboring nodes
pub struct Graph {
    adjacency_list: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    /// Creates an empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Adds an undirected edge between two nodes
    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.adjacency_list.entry(u).or_insert_with(HashSet::new).insert(v);
        self.adjacency_list.entry(v).or_insert_with(HashSet::new).insert(u);
    }

    // the neighborgs function returns the neighbors of a given node, if the node exists
    pub fn neighbors(&self, node: u32) -> Option<&HashSet<u32>> {
        self.adjacency_list.get(&node)
    }

    //  the load_graph_from_file function loads a graph from an edge list file
    pub fn load_graph_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut graph = Graph::new();
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            if let (Some(u), Some(v)) = (parts.next(), parts.next()) {
                let u = u.parse::<u32>().unwrap();
                let v = v.parse::<u32>().unwrap();
                graph.add_edge(u, v);
            }
        }
        Ok(graph)
    }

    // returns the degree of a node
    pub fn degree(&self, node: u32) -> usize {
        self.neighbors(node).map_or(0, |neighbors| neighbors.len())
    }

    // returns an iterator over all nodes in the graph
    pub fn nodes(&self) -> impl Iterator<Item = &u32> {
        self.adjacency_list.keys()
    }
}