use self::BinaryTree::*;
fn main() {

    let jupyter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));

    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupyter_tree,
        right: mercury_tree,
    }));

    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    }));

    let saturn_tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    saturn_tree.print();
    println!();
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

use std::fmt::Debug;
impl<T: Debug> BinaryTree<T> {
    fn print(&self) {
        match self {
            Empty => {}
            NonEmpty(node) => {
                node.left.print();
                print!("\t{:?}", node.element);
                node.right.print();
            }
        }
    }
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}