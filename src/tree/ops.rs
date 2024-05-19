use super::core::{Node, Tree, TreeError};

pub fn create_star_tree<I>(root: Node, nodes: I) -> Result<Tree, TreeError>
where
    I: IntoIterator<Item = Node>,
{
    let mut tree = Tree::new(root);
    for node in nodes {
        tree.add_node(root, node)?;
    }
    Ok(tree)
}

pub fn create_chain_tree<I>(nodes: I) -> Result<Tree, TreeError>
where
    I: IntoIterator<Item = Node>,
{
    let mut iter = nodes.into_iter();
    if let Some(root) = iter.next() {
        let mut tree = Tree::new(root);
        let mut current = root;
        for node in iter {
            tree.add_node(current, node)?;
            current = node;
        }
        Ok(tree)
    } else {
        panic!("The input list must contain at least one node.");
    }
}
