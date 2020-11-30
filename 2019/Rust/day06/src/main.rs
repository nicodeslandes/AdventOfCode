use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct GraphNode {
    key: String,
    parent: Option<String>,
    children: HashSet<String>,
}

impl<'a> GraphNode {
    fn new(key: &String, parent: Option<String>) -> GraphNode {
        GraphNode {
            key: key.clone(),
            parent: parent,
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
        self.add_or_get_node(child, &Some(parent.clone()));
        let parent_node = self.add_or_get_node(parent, &None);

        parent_node.children.insert(child.clone());
    }

    fn add_or_get_node(&mut self, key: &String, parent: &Option<String>) -> &mut GraphNode {
        let node = self
            .nodes
            .entry(key.clone())
            .or_insert(GraphNode::new(key, parent.clone()));

        match parent {
            Some(_p) => node.parent = parent.clone(),
            _ => (),
        };

        node
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

    println!("Total count: {}", orbit_counts.values().sum::<u32>());

    let mut distance_to_root: HashMap<String, u32> = HashMap::new();
    for key in orbits.nodes.keys() {
        compute_distance_to_root_for(&orbits, &mut distance_to_root, key);
    }

    let position_you = "YOU".to_string();
    let position_san = "SAN".to_string();

    let common_parents = find_common_parents(&orbits, &position_you, &position_san);

    println!("Common parent: {:?}", common_parents);

    // Find the one common parent that is a leaf is this sub-graph, ie that doesn't have any children in the set
    // let closest_node = common_parents.iter().find(|x| {
    //     orbits
    //         .get_node(x)
    //         .unwrap()
    //         .children
    //         .iter()
    //         .all(|c| !common_parents.contains(c))
    // });
    // println!("Leaf: {:?}", closest_node);

    // Or get the node farthest from the origin
    let closest_node = common_parents
        .iter()
        .max_by_key(|k| distance_to_root.get(&k.to_string()).unwrap())
        .expect("No common node found!");
    println!("Leaf: {:?}", closest_node);

    let orbital_transfer_count = distance_to_root[&position_you]
        - distance_to_root[&closest_node.to_string()]
        + distance_to_root[&position_san]
        - distance_to_root[&closest_node.to_string()]
        - 2; // Don't count the hop to the closest planet

    println!("Hops: {}", orbital_transfer_count);
    Ok(())
}
fn find_common_parents<'a>(orbits: &'a Graph, key1: &String, key2: &String) -> HashSet<&'a String> {
    let key1_parents: HashSet<&String> = get_node_parents(&orbits, key1);
    let key2_parents: HashSet<&String> = get_node_parents(&orbits, key2);

    println!("Key1 parents: {:?}", key1_parents);
    println!("Key2 parents: {:?}", key2_parents);

    key1_parents
        .intersection(&key2_parents)
        .map(|s| *s)
        .collect()
}

fn get_node_parents<'a>(orbits: &'a Graph, key: &String) -> HashSet<&'a String> {
    let mut key_parents: HashSet<&'a String> = HashSet::new();

    let mut node = orbits.get_node(key);
    while node.is_some() {
        let parent = &node.unwrap().parent;
        match parent {
            Some(p) => {
                key_parents.insert(&p);
                node = orbits.get_node(&p);
            }
            None => node = None,
        }
    }

    key_parents
}

fn compute_orbit_count_for(orbits: &Graph, orbit_counts: &mut HashMap<String, u32>, key: &String) {
    let node = orbits.get_node(key).unwrap();

    // First, compute the orbit counts for the children
    for child in node.children.iter() {
        compute_orbit_count_for(orbits, orbit_counts, &child);
    }

    // Then for the node
    let count = match node.parent {
        None => 0, // No orbit for the graph root
        Some(_) => {
            1 // direct orbit
            + node.children.iter().map(|c| orbit_counts.get(c).unwrap()).sum::<u32>()
        } // indirect orbits
    };

    orbit_counts.insert(node.key.clone(), count);
}

fn compute_distance_to_root_for(
    orbits: &Graph,
    distances_to_root: &mut HashMap<String, u32>,
    key: &String,
) -> u32 {
    match distances_to_root.get(key) {
        Some(value) => *value,
        None => {
            let node = orbits.get_node(key).unwrap();

            let distance_to_root = match &node.parent {
                Some(p) => 1 + compute_distance_to_root_for(orbits, distances_to_root, &p),
                None => 0,
            };

            distances_to_root.insert(key.clone(), distance_to_root);
            distance_to_root
        }
    }
}

fn parse_line<'a>(line: String) -> Vec<String> {
    let split = line.split(")").map(|s| s.to_string());
    split.collect::<Vec<_>>()
}
