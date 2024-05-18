use super::core::{Node, Tree};

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
