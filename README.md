# SMCITE

The `Tree` class supports the following operations:

`tree.add_node()`
`tree.get_nodes() -> HashSet<Node>` returns the nodes in the tree.
`tree.contains(node) -> bool` returns whether a node is in the tree.
`tree.is_child(chi, par) -> bool` returns whether `chi` is the child of `par`
`tree.get_children(node) -> Result<HashSet<Node>, TreeError>` returns the set of children of `Node`. If the `node` is not in the tree, then errors.
`tree.get_parent(node) -> Result<Option<Node>, TreeError>` returns the parent of node (for root there is no parent) or errors if the node was not found
`tree.swap_labels(node1, node2) -> Result<(), TreeError>` swaps the labels. If either of the nodes is not in the tree, then errors.

