use smcite::tree::{Tree, create_chain_tree};

fn main() {   
    let mut tree = create_chain_tree([0, 1, 2, 3]);
    tree.print();

    match tree.swap_labels(&1, &3) {
        Ok(value) => println!("Worked properly."),
        Err(e) => println!("Error caught! {:?}", e),
    };

    tree.print();
}