/// Implementation of a tree together with its core utilities.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

pub type Node = u32;

/// Tree data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    root: Node,
    nodes: HashSet<Node>,
    #[serde(serialize_with = "_map_to_vec", deserialize_with = "_vec_to_map")]
    children: HashMap<Node, HashSet<Node>>,
    parents: HashMap<Node, Node>,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
            && self.children == other.children
            && self.parents == other.parents
            && self.nodes == other.nodes
    }
}

impl Eq for Tree {}


#[derive(Debug)]
pub enum TreeError {
    NodeNotFound,
    NodeAlreadyExists,
}

impl Tree {
    /// Creates a new rooted tree with a single node `root`.
    pub fn new(root: Node) -> Self {
        let mut nodes = HashSet::new();
        nodes.insert(root);

        Tree {
            root: root,
            nodes: nodes,
            children: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    /// Returns the root of the tree.
    pub fn get_root(&self) -> Node {
        self.root
    }

    pub fn contains(&self, node: &Node) -> bool {
        self.nodes.contains(node)
    }

    fn unsafe_add_node(&mut self, parent: &Node, child: &Node) {
        self.nodes.insert(*child);

        self.children
            .entry(*parent)
            .or_insert_with(HashSet::new)
            .insert(*child);

        self.parents.insert(*child, *parent);
    }

    /// Adds a child node to a parent.
    pub fn add_node(&mut self, parent: &Node, child: &Node) -> Result<(), TreeError> {
        // Parent does not exist -> Error
        if !self.contains(parent) {
            return Err(TreeError::NodeNotFound);
        }
        // Child already is in the tree -> Error
        if self.contains(child) {
            return Err(TreeError::NodeAlreadyExists);
        }

        self.unsafe_add_node(parent, child);
        Ok(())
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

    pub fn subtree_size(&self, node: Node) -> Result<usize, TreeError> {
        fn dfs(tree: &Tree, node: &Node) -> usize {
            let mut size = 1; // Count the current node
            if let Some(children) = tree.children.get(&node) {
                for &child in children {
                    size += dfs(tree, &child);
                }
            }
            size
        }

        if self.contains(&node) {
            Ok(dfs(self, &node))
        } else {
            Err(TreeError::NodeNotFound)
        }
    }

    pub fn swap_labels(&mut self, i: &Node, j: &Node) -> Result<(), TreeError> {
        println!("Swapping labels...");
        
        if !self.contains(&i) || !self.contains(&j) {
            return Err(TreeError::NodeNotFound);
        }

        println!("Nodes exist...");

        // If either node is the root, handle root swapping
        if self.root == *i {
            self.root = *j;
        } else if self.root == *j {
            self.root = *i;
        }

        // Now we have update children and parents
        // Note that for either node these may not exist.
        let children_i = self.children.remove(i);
        let children_j = self.children.remove(j);

        let parent_i = self.parents.remove(i);
        let parent_j = self.parents.remove(j);

        if let Some(set) = children_i {
            // Node i has children
            for child in set.iter() {
                self.unsafe_add_node(j, child);
            }
        }
        if let Some(set) = children_j {
            // Node j has children
            for child in set.iter() {
                self.unsafe_add_node(i, child);
            }
        }

        // Finally: we need to fix the parents.
        if let Some(parent) = parent_i {
            self.parents.insert(*j, parent);
            // Update parent's children list
            if let Some(children) = self.children.get_mut(&parent) {
                children.remove(i);
                children.insert(*j);
            }
        }
        if let Some(parent) = parent_j {
            self.parents.insert(*i, parent);
            // Update parent's children list
            if let Some(children) = self.children.get_mut(&parent) {
                children.remove(j);
                children.insert(*i);
            }
        }

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Generates a tree
    /// 0– 1 – 10
    /// └─ 2
    fn simple_tree() -> Tree {
        let mut tree = Tree::new(0);
        tree.add_child(0, 1);
        tree.add_child(0, 2);
        tree.add_child(1, 10);
        tree
    }

    #[test]
    fn test_get_root() {
        let tree = simple_tree();
        assert_eq!(tree.get_root(), 0);
    }

    #[test]
    fn test_change_labels_root_right() {
        let mut tree = simple_tree();

        println!("Before swapping:");
        tree.print();

        tree.swap_labels(1, 10);

        println!("After swapping:");
        tree.print();

        assert_eq!(tree.get_root(), 2);
    }
}
