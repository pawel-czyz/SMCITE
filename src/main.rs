use std::collections::{HashMap, HashSet};

type Node = u32;

#[derive(Debug, Clone)]
struct Tree {
    root: Node,
    children: HashMap<Node, HashSet<Node>>,
    parents: HashMap<Node, Node>,
}

impl Tree {
    fn new(root: Node) -> Self {
        Tree {
            root: root,
            children: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    /// Adds a child to a parent
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

    println!("{:?}", tree);

    tree.print();

    let tree = create_star_tree(0, vec![10, 14, 20]);
    tree.print();
}
