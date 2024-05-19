/// Implementation of a tree together with its core utilities.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Alias for the node label type.
/// Note that it's a small type implementing Copy
/// so that it's easier to pass than reference.
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
    TopologyError,
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

    /// Returns true if `node` is already contained in the tree.
    pub fn contains(&self, node: Node) -> bool {
        self.nodes.contains(&node)
    }

    /// See `add_node`. This method does not do checks.
    fn unsafe_add_node(&mut self, parent: Node, child: Node) {
        self.nodes.insert(child);

        self.children
            .entry(parent)
            .or_insert_with(HashSet::new)
            .insert(child);

        self.parents.insert(child, parent);
    }

    /// Adds a child node to a parent.
    pub fn add_node(&mut self, parent: Node, child: Node) -> Result<(), TreeError> {
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
            let mut sorted_children: Vec<&Node> = children.iter().collect();
            sorted_children.sort();

            let count = sorted_children.len();
            for (i, &child) in sorted_children.iter().enumerate() {
                _print_tree(self, *child, "", i == count - 1);
            }
        }
    }

    /// Calculates the size of subtree starting at `node`
    /// (inclusive, i.e., the `subtree_size` of a leaf is 1).
    pub fn subtree_size(&self, node: Node) -> Result<usize, TreeError> {
        fn dfs(tree: &Tree, node: Node) -> usize {
            let mut size = 1; // Count the current node
            if let Some(children) = tree.children.get(&node) {
                for &child in children {
                    size += dfs(tree, child);
                }
            }
            size
        }

        if self.contains(node) {
            Ok(dfs(self, node))
        } else {
            Err(TreeError::NodeNotFound)
        }
    }

    /// Validates the tree.
    /// TODO: THIS FUNCTION IS UNTRUSTED YET.
    pub fn is_valid(&self) -> bool {
        let root = self.get_root();

        // 1. Check if all nodes except the root have a parent.
        for node in &self.nodes {
            if *node != root && !self.parents.contains_key(node) {
                return false;
            }
        }

        // 2. Check if the parent for all children is set properly.
        for (parent, children) in &self.children {
            for child in children {
                if let Some(child_parent) = self.parents.get(child) {
                    if *child_parent != *parent {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        // 3. Check if no node has a parent equal to itself or a child equal to itself.
        for node in &self.nodes {
            if let Some(parent) = self.parents.get(node) {
                if parent == node {
                    return false;
                }
            }
            if let Some(children) = self.children.get(node) {
                if children.contains(node) {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_parent(&self, node: Node) -> Option<Node> {
        self.parents.get(&node).copied()
    }

    /// Checks if `child` is a child of `parent`
    pub fn is_child(&self, child: Node, parent: Node) -> bool {
        if let Some(node) = self.parents.get(&child) {
            *node == parent
        } else {
            false
        }
    }

    pub fn is_parent(&self, parent: Node, child: Node) -> bool {
        self.is_child(child, parent)
    }

    /// Swaps two nodes in the tree, leaving the rest
    /// of the tree topology unchanged.
    pub fn swap_labels(&mut self, i: Node, j: Node) -> Result<(), TreeError> {
        if !self.contains(i) || !self.contains(j) {
            return Err(TreeError::NodeNotFound);
        }

        // When the nodes are the same one, just do nothing.
        if i == j {
            return Ok(());
        }

        // If either node is the root, handle root swapping
        if self.root == i {
            self.root = j;
        } else if self.root == j {
            self.root = i;
        }

        // Now the case where the nodes are adjacent
        let is_i_child_j = self.is_child(i, j);
        let is_j_child_i = self.is_child(j, i);
        if is_i_child_j || is_j_child_i {
            // Both can't be children simultaneously in a tree
            if is_i_child_j && is_j_child_i {
                return Err(TreeError::TopologyError);
            }
            let child = if is_i_child_j { i } else { j };
            let parent = if is_i_child_j { j } else { i };

            // Get the children of the child (grandchildren)
            // and the vector of parent's children
            // (i.e., the child node and its siblings)

            let grandchildren = self.children.remove(&child);
            let siblings = self.children.remove(&parent).unwrap();

            // Remove the current parent of the child
            self.parents.remove(&child);

            // Make sure the grandparent is properly connected
            if let Some(grandparent) = self.parents.remove(&parent) {
                self.children.get_mut(&grandparent).unwrap().remove(&parent);
                self.unsafe_add_node(grandparent, child);
            }
            self.unsafe_add_node(child, parent);

            // Now we need to fix grandchildren and siblings vectors.

            //   Fixing grandchildren, so they point to `parent`
            //   (and are added as `parent`'s children)
            if let Some(set) = grandchildren {
                for node in set.iter() {
                    self.unsafe_add_node(parent, *node);
                }
            }
            //    Fixing the siblings vector.
            for node in siblings.iter() {
                if *node != child {
                    self.unsafe_add_node(child, *node);
                }
            }
            return Ok(());
        }

        // Now the case when the nodes are not parent and child
        // Note that they can still be siblings

        // Now we have update children and parents
        // Note that for either node these may not exist.
        let children_i = self.children.remove(&i);
        let children_j = self.children.remove(&j);

        let parent_i = self.parents.remove(&i);
        let parent_j = self.parents.remove(&j);

        // We add (old) children of i as (new) children of j
        // and at the same time update their parent to j
        if let Some(set) = children_i {
            // Node i has children
            for child in set.iter() {
                self.unsafe_add_node(j, *child);
            }
        }
        if let Some(set) = children_j {
            // Node j has children
            for child in set.iter() {
                self.unsafe_add_node(i, *child);
            }
        }

        // Finally: we need to fix the parents of both nodes.
        // There is a case here to consider: they may have a common parent.
        if let (Some(p1), Some(p2)) = (parent_i, parent_j) {
            if p1 == p2 {
                // We don't have to update the children list.
                // We just need to update the `parents` map:
                self.parents.insert(i, p1);
                self.parents.insert(j, p1);
                return Ok(());
            }
        }
        // These nodes have different parents. We can update them separately:
        if let Some(parent) = parent_i {
            self.parents.insert(j, parent);
            // Update parent's children list
            if let Some(children) = self.children.get_mut(&parent) {
                children.remove(&i);
                children.insert(j);
            }
        }
        if let Some(parent) = parent_j {
            self.parents.insert(i, parent);
            // Update parent's children list
            if let Some(children) = self.children.get_mut(&parent) {
                children.remove(&j);
                children.insert(i);
            }
        }

        Ok(())
    }


    /// Calculates the height of the tree.
    pub fn calculate_height(&self) -> usize {
        self.calculate_height_from_node(self.get_root())
    }

    /// Calculates the height of the subtree starting at `node`.
    pub fn calculate_height_from_node(&self, node: Node) -> usize {
        if let Some(children) = self.children.get(&node) {
            let max_height = children.iter()
                .map(|&child| self.calculate_height_from_node(child))
                .max()
                .unwrap_or(0);
            max_height + 1
        } else {
            1 // Leaf node
        }
    }
}

fn _print_tree(tree: &Tree, node: Node, prefix: &str, is_last: bool) {
    // Print the current node
    println!("{}{}{}", prefix, if is_last { "└─" } else { "├─" }, node);

    // Update the prefix for children
    let new_prefix = if is_last { "  " } else { "│ " };

    // Recursively print each child
    if let Some(children) = tree.children.get(&node) {
        let mut sorted_children: Vec<&Node> = children.iter().collect();
        sorted_children.sort();

        let count = sorted_children.len();
        for (i, &child) in sorted_children.iter().enumerate() {
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
    /// 0–1–2–3
    /// └─10–11
    fn simple_tree() -> Tree {
        let mut tree = Tree::new(0);

        tree.add_node(0, 1).unwrap();
        tree.add_node(1, 2).unwrap();
        tree.add_node(2, 3).unwrap();

        tree.add_node(0, 10).unwrap();
        tree.add_node(10, 11).unwrap();
        tree
    }

    #[test]
    fn test_get_root() {
        let tree = simple_tree();
        assert_eq!(tree.get_root(), 0);
    }

    #[test]
    fn test_swap_labels_10_11() {
        let mut tree = Tree::new(0);

        tree.add_node(0, 1).unwrap();
        tree.add_node(1, 2).unwrap();
        tree.add_node(2, 3).unwrap();

        tree.add_node(0, 11).unwrap();
        tree.add_node(11, 10).unwrap();

        let mut new_tree = simple_tree();
        new_tree.swap_labels(10, 11).unwrap();
        assert_eq!(tree, new_tree);
    }

    #[test]
    fn test_swap_labels_1_10() {
        let mut tree = Tree::new(0);

        tree.add_node(0, 10).unwrap();
        tree.add_node(10, 2).unwrap();
        tree.add_node(2, 3).unwrap();

        tree.add_node(0, 1).unwrap();
        tree.add_node(1, 11).unwrap();

        let mut new_tree = simple_tree();
        new_tree.swap_labels(1, 10).unwrap();
        assert_eq!(tree, new_tree);
    }

    #[test]
    fn test_swap_labels_1_2() {
        let mut tree = Tree::new(0);

        tree.add_node(0, 2).unwrap(); // Node 1 becomes 2
        tree.add_node(2, 1).unwrap(); // Child of new 2 (was 1)
        tree.add_node(1, 3).unwrap(); // Child of new 1 (was 2)

        tree.add_node(0, 10).unwrap();
        tree.add_node(10, 11).unwrap();

        let mut new_tree = simple_tree();
        new_tree.swap_labels(1, 2).unwrap();
        assert_eq!(tree, new_tree);
    }

    #[test]
    fn test_swap_labels_0_1() {
        let mut tree = Tree::new(1); // Node 0 becomes 1 (new root)

        tree.add_node(1, 0).unwrap(); // Node 1 becomes 0 (child of new root)
        tree.add_node(0, 2).unwrap(); // Child of new 0 (was 1)
        tree.add_node(2, 3).unwrap(); // Child of 2

        tree.add_node(1, 10).unwrap(); // Sibling of new 0 (was root)
        tree.add_node(10, 11).unwrap(); // Child of 10

        let mut new_tree = simple_tree();
        new_tree.swap_labels(0, 1).unwrap();
        assert_eq!(tree, new_tree);
    }

    #[test]
    fn test_swap_labels_0_2() {
        let mut tree = Tree::new(2); // Node 0 becomes 2 (new root)

        tree.add_node(2, 1).unwrap(); // Node 1 remains the same
        tree.add_node(1, 0).unwrap(); // Node 2 becomes 0 (child of new root)
        tree.add_node(0, 3).unwrap(); // Child of new 0 (was 2)

        tree.add_node(2, 10).unwrap(); // Sibling of new 1 (was root)
        tree.add_node(10, 11).unwrap(); // Child of 10

        let mut new_tree = simple_tree();
        new_tree.swap_labels(0, 2).unwrap();
        assert_eq!(tree, new_tree);
    }

    #[test]
    fn test_swap_labels_1_3() {
        let mut tree = Tree::new(0);

        tree.add_node(0, 3).unwrap(); // Node 1 becomes 3
        tree.add_node(3, 2).unwrap(); // Child of new 3 (was 1)
        tree.add_node(2, 1).unwrap(); // Node 3 becomes 1 (child of 2)

        tree.add_node(0, 10).unwrap();
        tree.add_node(10, 11).unwrap();

        let mut new_tree = simple_tree();
        new_tree.swap_labels(1, 3).unwrap();

        assert_eq!(tree, new_tree);
    }
}
