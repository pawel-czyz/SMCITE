use rand::Rng;
use std::fmt::Debug;

fn main() {
    // Example usage
    let leaf1 = TreeNode::new("leaf1");
    let leaf2 = TreeNode::new("leaf2");
    let leaf3 = TreeNode::new("leaf3");

    let middle2 = TreeNode::new_with_children("middle2", vec![leaf2, leaf3]);
    let middle1 = TreeNode::new_with_children("middle1", vec![leaf1, middle2]);

    let root = TreeNode::new_with_children("root", vec![middle1]);

    println!("Ordinary printing:");
    println!("{:#?}", root);

    println!("\n\nPretty printing:");

    root.print_tree();
    println!("Tree size: {}", root.tree_size());

    let root = new_star_tree("root", vec!["A", "B", "C", "D", "E"]);
    root.print_tree();
    println!("Tree size: {}", root.tree_size());

    let root = new_chain_tree(vec!["A", "B", "C"]);
    root.print_tree();
    println!("Tree size: {}", root.tree_size());

    let temp = TreeNode::new("leaf");
    let root = TreeNode::new_with_children("root", vec![temp]);
    let leaf = &root.children[0]; // Create a reference to the leaf
    println!("Root tree size: {}", root.tree_size()); // 2
    println!("Leaf tree size: {}", leaf.tree_size()); // 1
}

#[derive(Debug, Clone)]

/// Represents a tree node with data `S`.
/// Note that `S` should be immutable and implement `Debug` and `Clone` traits.
struct TreeNode<S> {
    payload: S,
    children: Vec<TreeNode<S>>,
}

impl<S: Clone + Debug> TreeNode<S> {
    /// Creates a new leaf node (without children) storing data `S`.
    fn new(payload: S) -> TreeNode<S> {
        TreeNode {
            payload: payload,
            children: vec![],
        }
    }

    /// Creates a new node with payload `S` and children `children`.
    fn new_with_children(payload: S, children: Vec<TreeNode<S>>) -> TreeNode<S> {
        let mut node = TreeNode::new(payload);
        for v in children {
            node.add_child(v);
        }
        node
    }

    /// Adds a new `child`.
    fn add_child(&mut self, child: TreeNode<S>) {
        self.children.push(child);
    }

    /// Prints out the tree to the standard output.
    fn print_tree(&self) {
        _print_tree(self, "", "");
    }

    /// Calculates the subtree size (with the node included).
    ///
    /// # Examples
    ///
    /// ```rust
    /// let temp = TreeNode::new("leaf");
    /// let root = TreeNode::new_with_children("root", vec![temp]);
    /// let leaf = &root.children[0];  // Create a reference to the leaf
    /// assert_eq!(root.tree_size(), 2);
    /// assert_eq!(leaf.tree_size(), 1);
    /// ```
    fn tree_size(&self) -> usize {
        // Start with the current node
        let mut count = 1;
        // Add the count of each child subtree
        for child in self.children.iter() {
            count += child.tree_size();
        }
        count
    }
}

fn new_star_tree<S: Clone + Debug>(root_payload: S, children_payloads: Vec<S>) -> TreeNode<S> {
    let children = children_payloads
        .iter()
        .map(|payload| TreeNode::new(payload.clone()))
        .collect();
    TreeNode::new_with_children(root_payload, children)
}

fn new_chain_tree<S: Clone + Debug>(payloads: Vec<S>) -> TreeNode<S> {
    if payloads.is_empty() {
        panic!("payloads cannot be empty");
    }

    let mut root = TreeNode::new(payloads[0].clone());
    let mut current_node = &mut root;

    // Iterate over the remaining elements and create a chain
    for payload in &payloads[1..] {
        let new_node = TreeNode::new(payload.clone());
        current_node.add_child(new_node);
        current_node = current_node.children.last_mut().unwrap();
    }

    root
}

fn _print_tree<S: Debug>(node: &TreeNode<S>, spaces: &str, prefix: &str) {
    // Print the current node
    println!("{}{}{:?}", spaces, prefix, &(node.payload));

    // Update the prefix for children
    let new_spaces = format!("{}  ", spaces);

    // Recursively print each child
    let count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let new_prefix = if i == count - 1 { "└─" } else { "├─" };

        _print_tree(child, &new_spaces, new_prefix);
    }
}

// impl<S: Clone> TreeNode<S> {
//     fn new(payload: S) -> Arc<Self> {
//         Arc::new(TreeNode {
//             payload,
//             children: Vec::new(),
//         })
//     }

//     fn add_child(parent: &Arc<Self>, child: Arc<TreeNode<S>>) -> Arc<Self> {
//         let mut new_parent = parent.as_ref().clone();
//         new_parent.children.push(child);
//         Arc::new(new_parent)
//     }

//     fn collect_nodes(&self, nodes: &mut Vec<Arc<TreeNode<S>>>) {
//         nodes.push(Arc::new(self.clone()));
//         for child in &self.children {
//             child.collect_nodes(nodes);
//         }
//     }

//     fn swap_payloads(node1: &Arc<Self>, node2: &Arc<Self>) -> (Arc<Self>, Arc<Self>) {
//         let new_node1 = Arc::new(TreeNode {
//             payload: node2.payload.clone(),
//             children: node1.children.clone(),
//         });

//         let new_node2 = Arc::new(TreeNode {
//             payload: node1.payload.clone(),
//             children: node2.children.clone(),
//         });

//         (new_node1, new_node2)
//     }

//     fn deep_clone(&self) -> Arc<TreeNode<S>> {
//         let cloned_node = TreeNode::new(self.payload.clone());
//         let mut cloned_children = Vec::new();
//         for child in &self.children {
//             cloned_children.push(child.deep_clone());
//         }
//         Arc::new(TreeNode {
//             payload: self.payload.clone(),
//             children: cloned_children,
//         })
//     }

//     fn swap_random_payloads(tree: Arc<TreeNode<S>>) -> Arc<TreeNode<S>>
//     where
//         S: Clone,
//     {
//         let mut nodes = Vec::new();
//         tree.collect_nodes(&mut nodes);

//         let len = nodes.len();
//         if len < 2 {
//             return tree; // Not enough nodes to swap
//         }

//         let mut rng = rand::thread_rng();
//         let index1 = rng.gen_range(0..len);
//         let mut index2 = rng.gen_range(0..len);
//         while index2 == index1 {
//             index2 = rng.gen_range(0..len);
//         }

//         let node1 = &nodes[index1];
//         let node2 = &nodes[index2];

//         let (new_node1, new_node2) = Self::swap_payloads(node1, node2);

//         // Reconstruct the tree with the swapped nodes
//         Self::reconstruct_tree(&tree, &node1, new_node1, &node2, new_node2)
//     }

//     fn reconstruct_tree(
//         root: &Arc<TreeNode<S>>,
//         old_node1: &Arc<TreeNode<S>>,
//         new_node1: Arc<TreeNode<S>>,
//         old_node2: &Arc<TreeNode<S>>,
//         new_node2: Arc<TreeNode<S>>,
//     ) -> Arc<TreeNode<S>>
//     where
//         S: Clone,
//     {
//         if Arc::ptr_eq(root, old_node1) {
//             return new_node1;
//         }
//         if Arc::ptr_eq(root, old_node2) {
//             return new_node2;
//         }

//         let mut new_children = Vec::new();
//         for child in &root.children {
//             let new_child = Self::reconstruct_tree(child, old_node1, new_node1.clone(), old_node2, new_node2.clone());
//             new_children.push(new_child);
//         }

//         Arc::new(TreeNode {
//             payload: root.payload.clone(),
//             children: new_children,
//         })
//     }
// }
