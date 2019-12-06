use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct GraphNode {
    key: String,
    children: HashSet<String>,
}

impl<'a> GraphNode {
    fn new(key: &String) -> GraphNode {
        GraphNode {
            key: key.clone(),
            children: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, GraphNode>,
}

impl<'a> Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node_link(&mut self, parent: &String, child: &String) {
        self.add_or_get_node(child);
        let parent_node = self.add_or_get_node(parent);

        parent_node.children.insert(child.clone());
    }

    fn add_or_get_node(&mut self, key: &String) -> &mut GraphNode {
        self.nodes.entry(key.clone()).or_insert(GraphNode::new(key))
    }

    fn get_node(&self, key: &String) -> Option<&GraphNode> {
        self.nodes.get(key)
    }
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut orbits = Graph::new();

    for (_, line) in reader.lines().enumerate() {
        let orbit = parse_line(line.unwrap());
        orbits.add_node_link(&orbit[0], &orbit[1])
    }
    //println!("Values: {:?}", orbits);

    let mut orbit_counts: HashMap<String, u32> = HashMap::new();
    for key in orbits.nodes.keys() {
        if !orbit_counts.contains_key(key) {
            compute_orbit_count_for(&orbits, &mut orbit_counts, key)
        }
    }

    println!("Orbit counts: {:?}", orbit_counts);

    Ok(())
}

fn compute_orbit_count_for(orbits: &Graph, orbit_counts: &mut HashMap<String, u32>, key: &String) {
    let node = orbits.get_node(key).unwrap();
    if !orbit_counts.contains_key(key) {
        for child in node.children.iter() {
            compute_orbit_count_for(orbits, orbit_counts, &child);
        }
        children
    }
}

fn parse_line<'a>(line: String) -> Vec<String> {
    let split = line.split(")").map(|s| s.to_string());
    split.collect::<Vec<_>>()
}
