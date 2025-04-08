use::std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.adjacency_list.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_list.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        self.adjacency_list.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
    }

    fn dfs(&self, start: &str) {
        let mut visited = HashSet::new();
        println!("DFS");
        self.dfs_recursive(start, &mut visited);

    }
    
    fn dfs_recursive(&self, vertex: &str, visited: &mut HashSet<String>) {
        if visited.contains(vertex) {
            return;
        }
        println!("{}", vertex);
        visited.insert((vertex.to_string()));

        if let Some(neighbors) = self.adjacency_list.get(vertex) {
            for neighbor in neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }
    }
}