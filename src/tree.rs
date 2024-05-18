use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

pub type Node = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    root: Node,
    #[serde(serialize_with = "_map_to_vec", deserialize_with = "_vec_to_map")]
    children: HashMap<Node, HashSet<Node>>,
    parents: HashMap<Node, Node>,
}

impl Tree {
    /// Creates a new rooted tree with a single node `root`.
    pub fn new(root: Node) -> Self {
        Tree {
            root: root,
            children: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    /// Adds a child node to a parent.
    pub fn add_child(&mut self, parent: Node, child: Node) {
        self.children
            .entry(parent)
            .or_insert_with(HashSet::new)
            .insert(child);
        self.parents.insert(child, parent);
    }

    /// Prints out the tree to the standard output.
    pub fn print(&self) {
        println!("{}", self.root);

        // Recursively print children of the root node
        if let Some(children) = self.children.get(&(self.root)) {
            let count = children.len();
            for (i, child) in children.iter().enumerate() {
                _print_tree(self, *child, "", i == count - 1);
            }
        }
    }

    pub fn subtree_size(&self, i: Node) -> Result<usize, String> {
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

pub fn create_star_tree<I>(root: Node, nodes: I) -> Tree
where
    I: IntoIterator<Item = Node>,
{
    let mut tree = Tree::new(root);
    for node in nodes {
        tree.add_child(root, node);
    }
    tree
}

pub fn create_chain_tree<I>(nodes: I) -> Tree
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
            _print_tree(
                tree,
                *child,
                &format!("{}{}", prefix, new_prefix),
                i == count - 1,
            );
        }
    }
}

fn _map_to_vec<S>(map: &HashMap<Node, HashSet<Node>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let map: BTreeMap<_, Vec<_>> = map
        .iter()
        .map(|(&k, v)| (k, v.iter().cloned().collect()))
        .collect();
    map.serialize(serializer)
}

fn _vec_to_map<'de, D>(deserializer: D) -> Result<HashMap<Node, HashSet<Node>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: BTreeMap<Node, Vec<Node>> = BTreeMap::deserialize(deserializer)?;
    Ok(map
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect())
}
