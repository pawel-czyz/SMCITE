use smcite::tree::{create_chain_tree, create_star_tree, Tree};

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
