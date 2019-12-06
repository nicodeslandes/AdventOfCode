use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct GraphNode<'a> {
    key: String,
    children: HashSet<&'a GraphNode<'a>>,
}

impl<'a> Hash for GraphNode<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<'a> PartialEq for GraphNode<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<'a> Eq for GraphNode<'a> {}

impl<'a> GraphNode<'a> {
    fn new(key: &String) -> GraphNode<'a> {
        GraphNode {
            key: key.clone(),
            children: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<String, GraphNode<'a>>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node_link(&mut self, parent: &String, child: &String) {
        let child_node = self.nodes.entry(*child).or_insert(GraphNode::new(child));
        let parent_node = self.nodes.entry(*parent).or_insert(GraphNode::new(parent));

        parent_node.children.insert(&child_node);
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

    println!("Values: {:?}", orbits);
    Ok(())
}

fn parse_line<'a>(line: String) -> Vec<String> {
    let split = line.split(")").map(|s| s.to_string());
    split.collect::<Vec<_>>()
}
