use tracing::info;

mod alternate;

type BoxedNode<T> = Box<Node<T>>;

#[derive(Debug, Default)]
pub(crate) struct Node<T> {
    val: T,
    left: Option<BoxedNode<T>>,
    right: Option<BoxedNode<T>>,
}

impl<T: Default> Node<T> {
    pub(crate) fn new(val: T) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }
}

/// This is the end user facing struct that has the api
/// for working with and manipulating binary search trees.
#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Node<T>>,
}

/// the api for interacting with a tree
impl<T: Ord + Default> BinarySearchTree<T> {
    /// creates an empty tree
    pub fn new() -> Self {
        info!("creating new BST");
        Self { root: None }
    }

    /// whether or not the tree is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()

        // if let Some(root) = &self.root {
        //     true
        // } else {
        //     false
        // }

        // if self.root.is_none() {
        //     true
        // } else {
        //     false
        // }
    }

    /// inserts a value into the tree
    pub fn insert(&mut self, val: T) {
        // if the tree is empty, then we just make the root
        // a new node with the value in the parameter `val`

        if self.is_empty() {
            // let node = Node {
            //     val,
            //     left: None,
            //     right: None,
            // };

            let node = Node::new(val);

            self.root = Some(node)
        } else {
            // we have a non empty tree,
            // we need to insert the value at the correct place

            // we need to walk down the tree and find the correct place.
            let root = self.root.as_mut().unwrap();
            insert_internal(root, val)
        }
    }

    pub fn delete(&mut self, val: T) {
        // walk the tree and find the node has the value
        // if it exists, remove it, and reposition any children it had

        if self.is_empty() {
            return;
        }

        let root_val = self.root.as_mut();

        if let Some(root_val) = root_val {
            if val == root_val.val {
                // we need to delete the root node
                let old_root = self.root.take().unwrap();

                if let Some(left_child) = old_root.left {
                    // left child is the new root
                    self.root.replace(*left_child);
                    // now we need to deal with merging in the right
                    // child if it exists
                    if let Some(right_child) = old_root.right {
                        merge(self.root.as_mut().unwrap(), *right_child);
                    }
                } else if let Some(right_child) = old_root.right {
                    // right child is the new root
                    // left child is the new root
                    self.root.replace(*right_child);
                    // now we need to deal with merging in the right
                    // child if it exists
                    if let Some(left_child) = old_root.left {
                        merge(self.root.as_mut().unwrap(), *left_child);
                    }
                }
            }
        }
    }
}

/// Deletes a given value from a subtree, it checks to see if either
/// the right or left child of `root` needs to be deleted
fn delete_internal<T: Ord + Default>(root: &mut Node<T>, val: T) {
    // let's peak at our children to determine if they need to be
    // deleted, and if they are, we can handle things nicely
    // if val == root.left.as_ref().unwrap().val {
    //     // we found what we need to delete

    // }

    match val.cmp(&root.val) {
        // we can use a match guard here if we want to get rid of the nested let some
        std::cmp::Ordering::Less if root.left.is_some() => {
            // need to look at left side
            // if let Some(left_child) = root.left.as_mut() {
            let left_child = root.left.as_mut().unwrap();

            if val == left_child.val {
                // we need to delete the left child
                // we want to replace whatever root's left child currently is
                // with anything lower in the tree than the node we are deleting

                // 1) if left child is leaf node
                // 2) if left child has one child
                // 3) if left child has two children --> two subcases (if left, if right)

                let node_to_delete = left_child;

                match (node_to_delete.left.as_ref(), node_to_delete.right.as_ref()) {
                    (None, None) => {
                        // leaf node
                        root.left.take();
                    }
                    (Some(_), None) => {
                        // there is a left child only
                        // and we want to replace the node we are deleting
                        // with this left child
                        let new_left_child = node_to_delete.left.take().unwrap();
                        root.left.replace(new_left_child);
                        // let x = new_left.to_owned();
                        // root.left.replace(node_to_delete.left.take().unwrap());
                    }
                    (None, Some(_)) => {
                        // we only have a right node
                        let new_right_child = node_to_delete.right.take().unwrap();
                        root.right.replace(new_right_child);
                    } // if there is a right child only
                    (Some(_), Some(right_node)) => {
                        // we have two children
                        let new_left_node = node_to_delete.left.take().unwrap();

                        // now we have to deal with the right node.
                        let new_right_node = node_to_delete.right.take().unwrap();

                        root.left.replace(new_left_node);
                        merge(root, *new_right_node);
                    }
                }
                // if node_to_delete.left.is_none() && node_to_delete.right.is_none() {
                //     // node to delete is a leaf node

                //     // we need to not have a reference, but we want to take
                //     // the value out of the the left option.
                //     root.left.take();

                // } else if node. {

                // }
            } else {
                delete_internal(left_child, val)
            }
        }
        std::cmp::Ordering::Equal => {
            panic!("we broke the invariant that root is not the value we are trying to delete")
        }
        std::cmp::Ordering::Greater if root.right.is_some() => {
            let right_child = root.right.as_mut().unwrap();

            if val == right_child.val {
                // we need to delete the left child
                // we want to replace whatever root's left child currently is
                // with anything lower in the tree than the node we are deleting

                // 1) if left child is leaf node
                // 2) if left child has one child
                // 3) if left child has two children --> two subcases (if left, if right)

                let node_to_delete = right_child;

                match (node_to_delete.left.as_ref(), node_to_delete.right.as_ref()) {
                    (None, None) => {
                        // leaf node
                        root.right.take();
                    }
                    (Some(_), None) => {
                        // there is a left child only
                        // and we want to replace the node we are deleting
                        // with this left child
                        let new_left_child = node_to_delete.left.take().unwrap();
                        root.left.replace(new_left_child);
                        // let x = new_left.to_owned();
                        // root.left.replace(node_to_delete.left.take().unwrap());
                    }
                    (None, Some(_)) => {
                        // we only have a right node
                        let new_right_child = node_to_delete.right.take().unwrap();
                        root.right.replace(new_right_child);
                    } // if there is a right child only
                    (Some(_), Some(right_node)) => {
                        // we have two children
                        let new_left_node = node_to_delete.left.take().unwrap();

                        // now we have to deal with the right node.
                        let new_right_node = node_to_delete.right.take().unwrap();

                        root.left.replace(new_left_node);
                        merge(root, *new_right_node);
                    }
                }
            } else {
                delete_internal(right_child, val)
            }
        }
        _ => (), // we have hit a leaf node; the value doesn't exist
    }

    // if val < root.as_ref().unwrap() {

    // }
    // // if there is a left child, then let's see what's up
    // if let Some(left_child) = root.left.as_mut() {
    //     if val == left_child.val {
    //         // we need to delete the left child
    //     } else {
    //         delete_internal(left_child, val)
    //     }
    // }
}

/// merge two subtrees (i.e., nodes)
fn merge<T: Ord + Default>(root: &mut Node<T>, to_merge: Node<T>) {
    assert!(to_merge.val != root.val);

    if to_merge.val < root.val {
        // go left
        if let Some(left) = root.left.as_mut() {
            merge(left, to_merge)
        } else {
            root.left = Some(Box::new(to_merge))
        }
    } else {
        // go right
        if let Some(right) = root.right.as_mut() {
            merge(right, to_merge)
        } else {
            root.right = Some(Box::new(to_merge))
        }
    }
}

/// handles recursive insertion of a node into a tree
fn insert_internal<T: Ord + Default>(root: &mut Node<T>, val: T) {
    // we don't want duplicate values in our tree
    if val == root.val {
        // it's a duplicate!
        return;
    }

    let new_node = Node {
        val,
        ..Default::default()
    };

    // merge our new node into our existing tree
    merge(root, new_node);

    // if val < root.val {
    //     // we need to go down the left side.

    //     // if it's a leaf node we can add it right here
    //     // we need to make a new node, and add it as the left child
    //     if root.left.is_none() {
    //         let node = Node {
    //             val,
    //             ..Default::default()
    //         };
    //         root.left = Some(Box::new(node));
    //         return;
    //     }

    //     let left = root.left.as_mut().unwrap();
    //     insert_internal(left, val)
    // } else {
    //      // we need to go down the right side.
    //     if root.right.is_none() {
    //         let node = Node {
    //             val,
    //             ..Default::default()
    //         };
    //         root.right = Some(Box::new(node));
    //         return;
    //     }

    //     let right = root.right.as_mut().unwrap();
    //     insert_internal(right, val)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        let mut bst = BinarySearchTree::new();
        bst.insert(5_usize);

        eprintln!("bst: {bst:?}");

        bst.insert(6);

        eprintln!("bst: {bst:?}");

        bst.delete(5);

        eprintln!("bst: {bst:?}");
    }
}
