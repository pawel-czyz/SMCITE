use smcite::tree::{create_chain_tree, Tree};

fn main() {
    let mut tree = create_chain_tree([0, 1, 2, 3]).unwrap();
    tree.print();

    match tree.swap_labels(1, 3) {
        Ok(_) => println!("Worked properly."),
        Err(e) => println!("Error caught! {:?}", e),
    };

    tree.print();
}
