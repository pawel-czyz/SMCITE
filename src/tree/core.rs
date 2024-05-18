/// Implementation of a tree together with its core utilities.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

pub type Node = u32;

/// Tree data structure.
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

    /// Returns the root of the tree.
    pub fn get_root(&self) -> Node {
        self.root
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

    pub fn swap_labels(&mut self, i: Node, j: Node) -> Result<(), String> {
        if !self.children.contains_key(&i) || !self.children.contains_key(&j) {
            return Err("One or both nodes do not exist".to_string());
        }

        // If either node is the root, handle root swapping
        if self.root == i {
            self.root = j;
        } else if self.root == j {
            self.root = i;
        }

        // Swap children of i and j
        let children_i = self.children.remove(&i).unwrap_or_default();
        let children_j = self.children.remove(&j).unwrap_or_default();
        self.children.insert(i, children_j.clone());
        self.children.insert(j, children_i.clone());

        // Update parents of the children
        for child in &children_i {
            self.parents.insert(*child, j);
        }
        for child in &children_j {
            self.parents.insert(*child, i);
        }

        // Swap parents of i and j if they are not the root
        if let Some(parent_i) = self.parents.remove(&i) {
            self.parents.insert(j, parent_i);
        }
        if let Some(parent_j) = self.parents.remove(&j) {
            self.parents.insert(i, parent_j);
        }

        // Update the parent's children set
        if let Some(parent_i) = self.parents.get(&j) {
            if let Some(children) = self.children.get_mut(parent_i) {
                children.remove(&i);
                children.insert(j);
            }
        }
        if let Some(parent_j) = self.parents.get(&i) {
            if let Some(children) = self.children.get_mut(parent_j) {
                children.remove(&j);
                children.insert(i);
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
