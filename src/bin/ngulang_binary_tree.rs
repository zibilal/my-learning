use std::fmt::Debug;
use BinaryTree::*;
fn main() {

}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

impl<T: Debug> BinaryTree<T> {
    fn print(&self) {
        match self {
            Empty => {}
            NonEmpty(node) => {
                node.left.print();
                print!("\t:{:?}", node.element);
                node.right.print();
            }
        }
    }
}