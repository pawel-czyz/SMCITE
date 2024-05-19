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

#[cfg(test)]
mod tests {
    use super::*;

    mod test_create_star_tree {
        use super::*;

        #[test]
        fn star_4_nodes() {
            let root = 15;
            let tree = create_star_tree(root, [3, 5, 8]).unwrap();
            let mut new_tree = Tree::new(root);
            new_tree.add_node(root, 3).unwrap();
            new_tree.add_node(root, 5).unwrap();
            new_tree.add_node(root, 8).unwrap();
            assert_eq!(tree, new_tree);
        }
    }

    mod test_create_chain_tree {
        use super::*;

        #[test]
        fn chain_3_nodes() {
            let tree = create_chain_tree([3, 5, 8]).unwrap();
            let mut new_tree = Tree::new(3);
            new_tree.add_node(3, 5).unwrap();
            new_tree.add_node(5, 8).unwrap();
            assert_eq!(tree, new_tree);
        }
    }
}
