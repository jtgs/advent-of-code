use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

type Object = String;

#[derive(Debug)]
struct Orbit {
    left: Object,
    right: Object
}

impl Orbit {
    fn from_str(input: String) -> Self {
        let parts: Vec<&str> = input.split(')').collect();
        Self {left: parts[0].to_owned(), right: parts[1].to_owned()}
    }
}

struct Node {
    this: Object,
    children: Vec<Object>
}

struct Tree {
    nodes: HashMap<Object, Node>
}

impl Tree {
    fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    /// Add a new Orbit to the tree. 
    /// 
    /// Finds the object on the LHS of the Orbit, and adds the RHS as 
    /// a child of it. 
    fn add_orbit(&mut self, orbit: Orbit) {
        // If the left object in this orbit isn't already in the tree,
        // add it now.
        if !self.nodes.contains_key(&orbit.left) {
            let new_node = Node {this: orbit.left.clone(), children: Vec::new()};
            self.nodes.insert(new_node.this.clone(), new_node);
        }
        
        // Get the node in the tree corresponding to the left object.
        // (This cannot fail since we just added it above.)
        let node: &mut Node = self.nodes.get_mut(&orbit.left).unwrap();

        node.children.push(orbit.right.clone());
    }

    fn walk_from_com(&self) -> i32 {
        let com = self.nodes.get("COM").unwrap();
        
        self.walk(com.this.clone(), 0)
    }

    fn walk(&self, obj: Object, depth: i32) -> i32 {
        debug!("Walk from {}, depth {}", obj, depth);
        // Find the object referenced.
        if let Some(the_obj) = self.nodes.get(&obj) {
            let mut count = depth;
            let depth = depth + 1;

            // Call walk on each of its children.
            for child in the_obj.children.clone() {
                debug!(" -> child of {}", obj);
                count += self.walk(child, depth);
            }
            
            debug!("{} contributes {}", obj, count);
            return count;
        } 

        // If the node is not in the tree, it has no children - so return the 
        // current depth.
        debug!("End of the line: {} contributes {}", obj, depth);
        depth
    }
}

pub fn part_a() -> i32 {
    let tree = get_tree("input6.txt");

    tree.walk_from_com()
}

fn get_tree(filename: &str) -> Tree {
    let orbits: Vec<Orbit> = BufReader::new(File::open(filename).expect("Unable to open file"))
    .lines()
    .map(Result::unwrap)
    .map(Orbit::from_str)
    .collect();

    let mut tree: Tree = Tree::new();

    for orbit in orbits {
        debug!("{:?}", orbit);
        tree.add_orbit(orbit);
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn six_example_one() {
        init();

        let tree = get_tree("test6.txt");

        assert_eq!(42, tree.walk_from_com());
    }
}