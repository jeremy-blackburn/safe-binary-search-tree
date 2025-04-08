use std::{borrow::Borrow, fmt::Display};

type BoxedNode<T> = Box<Node<T>>;

#[derive(Debug, Default)]
struct Node<T: Ord + Display> {
    val: T,
    left: Option<BoxedNode<T>>,
    right: Option<BoxedNode<T>>,
}

impl<T: Ord + Default + Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

// impl<T: Ord + Default + Display> Node<T> {
//     pub fn pretty_print(&self, leading_spaces: usize) {
//         let spaces = leading_spaces + 1;
//         for _n in 0..spaces {
//             print!(" ");
//         }
//         println!("{}", self.val);
//         if let Some(left) = &self.left {
//             left.pretty_print(spaces)
//         }

//         if let Some(right) = &self.right {
//             right.pretty_print(spaces)
//         }

//         // match &self.left {
//         //     Some(n) => n.print(space),
//         //     None => (),
//         // }
//         // for _n in crate::COUNT..=space {
//         //     print!(" ");
//         // }
//         // self.root.print();
//         // println!("{}", self.val);

//         // match &self.right {
//         //     Some(n) => n.print(space),
//         //     None => (),
//         // }
//     }
// }

#[derive(Debug, Default)]
pub struct BinarySearchTree<T: Ord + Default + Display> {
    root: Option<Node<T>>,
}

// impl<T: Ord + Default + Display> Display for BinarySearchTree<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // let space = spacing + crate::COUNT;
//         // let space = 1;

//         // if self.root.is_none() {
//         //     write!(f, "");
//         // }

//         // match &self.left {
//         //     Some(n) => n.print(space),
//         //     None => (),
//         // }
//         // for _n in crate::COUNT..=space {
//         //     print!(" ");
//         // }
//         // self.root.print();

//         // match &self.right {
//         //     Some(n) => n.print(space),
//         //     None => (),
//         // }
//         todo!()
//     }
// }

impl<T: Ord + Default + Display> BinarySearchTree<T> {
    pub fn exists(&self, val: T) -> bool {
        if let Some(root) = &self.root {
            exists_internal(root, val)
        } else {
            false
        }
    }

    pub fn insert(&mut self, val: T) {
        if let Some(root) = self.root.as_mut() {
            insert_internal(root, val)
        } else {
            self.root = Some(Node {
                val,
                ..Default::default()
            })
        }
    }

    pub fn delete(&mut self, val: T) {
        if let Some(root) = self.root.as_mut() {
            // special case for checking if the root node is the one to delete.
            if root.val == val {
                // we need to delete the root node!
                // the left (if exists) will become our new root.
                // the right will then be merged in
                // if the left node does not exist, the right node will
                // be the new root.
                // if neither exist, there is no new root!
                let left = root.left.take();
                let right = root.right.take();
                if let Some(new_root) = left {
                    // note we are derefencing the box to pull the value out of it.
                    self.root.replace(*new_root);

                    // now deal with the new right.
                    if let Some(new_right) = right {
                        let root = self.root.as_mut().unwrap();
                        merge(root, new_right)
                    }
                } else if let Some(new_root) = right {
                    // we never have to deal with a dangling left pointer if
                    // we got here
                    self.root.replace(*new_root);
                }
            } else {
                delete_internal(root, val)
            }
        }
    }

    pub fn pretty_print(&self) {
        if let Some(root) = &self.root {
            let height = self.height();

            // NB: we can wrap around here because of usize to u32 conversion!!!!
            let width = 2_i32.pow((height - 1) as u32);

            // now we need to print our root node at whatever width/2?
            println!("{}{root}", " ".repeat((width / 2).try_into().unwrap()));
        } else {
            println!();
            return;
        }

        // now we need to basically do a bfs and print the next level

        // todo!()
    }

    pub fn height(&self) -> usize {
        height_internal(self.root.as_ref())
    }
}

// fn pretty_print_internal<T: Ord + Display>(root: Option<&Node<T>>, curr_level) {

// }

fn height_internal<T: Ord + Display>(root: Option<&Node<T>>) -> usize {
    // if root.is_none() {
    //     0
    // } else {
    //     let left_height = height_internal(root.left)
    // }

    if let Some(root) = root {
        let left_height = height_internal(root.left.as_deref());
        let right_height = height_internal(root.right.as_deref());

        1 + std::cmp::max(left_height, right_height)

        // i think below is probably less readable although it does look fancier.
        // 1 + std::cmp::max(
        //     height_internal(root.left.as_deref()),
        //     height_internal(root.right.as_deref()),
        // )
    } else {
        0
    }
}

fn insert_internal<T: Ord + Default + Display>(root: &mut Node<T>, val: T) {
    // we are doing an allocation up front. yolo
    let to_merge = Box::new(Node {
        val,
        ..Default::default()
    });

    merge(root, to_merge);

    // if val < root.val {
    //     // go left
    //     if let Some(left) = root.left.as_mut() {
    //         insert_internal(left, val)
    //     } else {
    //         root.left = Some(Box::new(Node {
    //             val,
    //             ..Default::default()
    //         }))
    //     }
    // } else {
    //     // go right
    //     if let Some(right) = root.right.as_mut() {
    //         insert_internal(right, val)
    //     } else {
    //         root.right = Some(Box::new(Node {
    //             val,
    //             ..Default::default()
    //         }))
    //     }
    // }
}

fn merge<T: Ord + Display>(root: &mut Node<T>, to_merge: BoxedNode<T>) {
    // there is extra work to make sure you don't merge with
    // identical nodes so i'm just going to assert we never get
    // that condition
    assert!(to_merge.val != root.val);

    if to_merge.val < root.val {
        // go left
        if let Some(left) = root.left.as_mut() {
            merge(left, to_merge)
        } else {
            root.left = Some(to_merge)
        }
    } else {
        // go right
        if let Some(right) = root.right.as_mut() {
            merge(right, to_merge)
        } else {
            root.right = Some(to_merge)
        }
    }
}

fn delete_internal<T: Ord + Display>(root: &mut Node<T>, val: T) {
    match val.cmp(&root.val) {
        std::cmp::Ordering::Equal => {
            panic!("something is broken?")
        }
        // it would be nice to use if let in a match guard, but we can't at time of writing
        // see: https://github.com/rust-lang/rust/issues/51114
        // suggested work around is to use the matches!() macro (https://doc.rust-lang.org/std/macro.matches.html)
        // but idk how to make it work.
        // std::cmp::Ordering::Less if matches!(root.left.as_mut(), Some(left)) => {
        //     // go left
        //     // let x = root.left;
        //     // ?delete_internal(root, val)
        //     delete_internal(left, val);
        //     todo!()
        // },
        std::cmp::Ordering::Less if root.left.is_some() => {
            // let left = root.left.as_mut().unwrap();
            if val == root.left.as_ref().unwrap().val {
                // we need to delete the left node
                eprintln!(
                    "node to delete ({val}) is left child of current node ({})",
                    root.val
                );
                // we can go ahead and just take ownership of the value that we
                let mut to_delete = root.left.take().unwrap();
                // this would leave any children orphaned, so we need to deal with those.

                // first we see if the there is any left child, if there is, then we replace our left with that
                if let Some(new_left) = to_delete.left.take() {
                    // make this root's new left node
                    root.left.replace(new_left);
                }

                // now merge in the right side of our deleted node if it exists
                if let Some(new_right) = to_delete.right.take() {
                    merge(root.left.as_mut().unwrap(), new_right)
                }
            } else {
                // recurse down left side of tree
                delete_internal(root.left.as_mut().unwrap(), val)
            }
        }
        std::cmp::Ordering::Greater if root.right.is_some() => {
            // go right
            // we can do this by using merge insead of the replace stuff.
            // idk if that gives us any performance benfit or not
            // but this is an alternate implementation
            if val == root.right.as_ref().unwrap().val {
                // we need to delete the left node
                eprintln!(
                    "node to delete ({val}) is right child of current node ({})",
                    root.val
                );

                let mut to_delete = root.right.take().unwrap();

                let new_right = to_delete.right.take();
                let new_left = to_delete.left.take();

                // new_right.map(|new_right| merge(root, new_right));
                if let Some(new_right) = new_right {
                    merge(root, new_right);
                }

                // we could make this slightly more efficient if we wanted
                // we are starting at the root of our original subtree, and we know that we
                // can really start down to the new right subtree.
                // but the code is a bit uglier without more refactoring
                if let Some(new_left) = new_left {
                    merge(root, new_left);
                }
            } else {
                // recurse down left side of tree
                delete_internal(root.right.as_mut().unwrap(), val)
            }
        }
        _ => (), // value to delete didn't exist
    }
}

fn exists_internal<T: Ord + Display>(root: &Node<T>, val: T) -> bool {
    if val == root.val {
        true
    } else if val < root.val && root.left.is_some() {
        // go left
        // note that we can use an `unwrap()`` here because our if statement
        // makes this makes left being `Some` an _invariant_.
        //
        // There is probably some more elegant construct we can use, but whatever
        let left = root.left.as_ref().unwrap();
        exists_internal(left, val)
    } else if val > root.val && root.right.is_some() {
        exists_internal(root.right.as_ref().unwrap(), val)
    } else {
        false
    }
}

// enum Child {
//     Left(usize),
//     Right(usize)
// }

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_TREE_1: [i32; 12] = [87, 1, 3, 58, 99, 69, 70, 31, 41, 59, 26, 18];
    const TEST_TREE_2: [i32; 10] = [100, 10, 500, 50, 25, 75, 20, 30, 60, 80];

    #[test]
    fn test_height_full_right() {
        let mut bst = BinarySearchTree::default();

        for i in 0..100 {
            bst.insert(i);
        }

        let height_expected = 100;

        assert_eq!(bst.height(), height_expected);
    }

    #[test]
    fn test_height_empty() {
        let bst = BinarySearchTree::<i32>::default();
        assert_eq!(bst.height(), 0);
    }

    #[test]
    fn test_height_1() {
        let mut bst = BinarySearchTree::default();

        for val in TEST_TREE_1 {
            bst.insert(val);
        }

        assert_eq!(bst.height(), 7);

        // get rid of node 58 on the left side.
        bst.delete(58);
        println!("{bst:?}");
        assert_eq!(bst.height(), 8);
    }

    #[test]
    fn test_height_2() {
        let mut bst = BinarySearchTree::default();

        for val in TEST_TREE_2 {
            bst.insert(val);
        }

        // eprintln!{"{bst:?}"};
        assert_eq!(bst.height(), 5);

        bst.delete(25);
        eprintln!("{bst:?}\n");
        assert_eq!(bst.height(), 5);

        bst.delete(50);
        eprintln!("{bst:?}\n");
        assert_eq!(bst.height(), 6);

        bst.insert(5);
        eprintln!("{bst:?}\n");
        assert_eq!(bst.height(), 6);

        bst.delete(10);
        eprintln!("{bst:?}");
        assert_eq!(bst.height(), 6);
    }

    #[test]
    fn test_pretty_print() {
        let mut bst = BinarySearchTree::default();

        for val in TEST_TREE_2 {
            bst.insert(val);
        }

        // eprintln!("{}", bst.pretty_print())
        bst.pretty_print();
    }

    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::default();
        eprintln!("Test values supplied by my friend Grimgar");
        let numbers = vec![87, 1, 3, 58, 99, 69, 70, 31, 41, 59, 26, 18];
        for num in numbers {
            println!("{num} added to the bst");
            bst.insert(num);
        }
        // eprintln!("A print of the tree, before Any Tests");
        // bst.pretty_print();
        eprint!("Tree: {bst:?}\n");

        bst.delete(31);
        eprint!("Tree with 31 deleted: {bst:?}\n");
    }
}
