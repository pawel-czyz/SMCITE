use std::collections::{BTreeMap, HashMap, HashSet};
use serde::{Deserialize, Serialize};
use serde_json;

type Node = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tree {
    root: Node,
    #[serde(serialize_with = "_map_to_vec", deserialize_with = "_vec_to_map")]
    children: HashMap<Node, HashSet<Node>>,
    parents: HashMap<Node, Node>,
}

impl Tree {
    /// Creates a new rooted tree with a single node `root`.
    fn new(root: Node) -> Self {
        Tree {
            root: root,
            children: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    /// Adds a child node to a parent.
    fn add_child(&mut self, parent: Node, child: Node) {
        self.children.entry(parent).or_insert_with(HashSet::new).insert(child);
        self.parents.insert(child, parent);
    }

    /// Prints out the tree to the standard output.
    fn print(&self) {
        println!("{}", self.root);

        // Recursively print children of the root node
        if let Some(children) = self.children.get(&(self.root)) {
            let count = children.len();
            for (i, child) in children.iter().enumerate() {
                _print_tree(self, *child, "", i == count - 1);
            }
        }
    }

    fn subtree_size(&self, i: Node) -> Result<usize, String> {
        fn dfs(tree: &Tree, node: Node) -> usize {
            let mut size = 1; // Count the current node
            if let Some(children) = tree.children.get(&node) {
                for &child in children {
                    size += dfs(tree, child);
                }
            }
            size
        }

        if self.parents.contains_key(&i) || self.root == i {
            Ok(dfs(self, i))
        } else {
            Err(format!("Node {} is not in the tree.", i))
        }
    }
}


fn create_star_tree<I>(root: Node, nodes: I) -> Tree
where
    I: IntoIterator<Item = Node>,
{
    let mut tree = Tree::new(root);
    for node in nodes {
        tree.add_child(root, node);
    }
    tree
}

fn create_chain_tree<I>(nodes: I) -> Tree
where
    I: IntoIterator<Item = Node>,
{
    let mut iter = nodes.into_iter();
    if let Some(root) = iter.next() {
        let mut tree = Tree::new(root);
        let mut current = root;
        for node in iter {
            tree.add_child(current, node);
            current = node;
        }
        tree
    } else {
        panic!("The input list must contain at least one node.");
    }
}

fn _print_tree(tree: &Tree, node: Node, prefix: &str, is_last: bool) {
    // Print the current node
    println!("{}{}{}", prefix, if is_last { "└─" } else { "├─" }, node);

    // Update the prefix for children
    let new_prefix = if is_last { "  " } else { "│ " };

    // Recursively print each child
    if let Some(children) = tree.children.get(&node) {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            _print_tree(tree, *child, &format!("{}{}", prefix, new_prefix), i == count - 1);
        }
    }
}


fn _map_to_vec<S>(map: &HashMap<Node, HashSet<Node>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let map: BTreeMap<_, Vec<_>> = map.iter().map(|(&k, v)| (k, v.iter().cloned().collect())).collect();
    map.serialize(serializer)
}

fn _vec_to_map<'de, D>(deserializer: D) -> Result<HashMap<Node, HashSet<Node>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: BTreeMap<Node, Vec<Node>> = BTreeMap::deserialize(deserializer)?;
    Ok(map.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect())
}



fn main() {
    let mut tree = Tree::new(0);

    // Construct the tree
    tree.add_child(0, 1);
    tree.add_child(0, 2);
    tree.add_child(1, 3);
    tree.add_child(1, 4);
    tree.add_child(2, 5);
    tree.add_child(2, 6);
    tree.add_child(6, 12);
    tree.add_child(12, 1001);
    tree.add_child(1001, 100011);

    tree.print();

    for idx in [0, 1, 2, 6, 1001, 100011, 108] {
        match tree.subtree_size(idx) {
            Ok(size) => println!("Subtree size starting at node {}: {}", idx, size),
            Err(err) => println!("{}", err),
        }
    }

    let tree = create_star_tree(0, vec![10, 14, 20]);
    tree.print();

    let tree = create_chain_tree(vec![5, 10, 14, 20]);
    tree.print();

    let serialized = serde_json::to_string(&tree).unwrap();
    println!("Serialized tree: {}", serialized);

    // Deserialize the JSON string back to a Tree
    let deserialized: Tree = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized:");
    deserialized.print();
}
