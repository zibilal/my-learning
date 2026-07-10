#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // value + pointer to next node,
    Nil,                  // terminator (like nil in Go)
}

// Singly Linked List
impl List {
    fn new() -> Self {
        List::Nil
    }

    // Prepend a value - consumes self, returns new head
    fn push(self, val: i32) -> Self {
        List::Cons(val, Box::new(self))
    }

    fn print(&self) {
        match self {
            List::Nil => println!(),
            List::Cons(val, next) => {
                if matches!(**next, List::Nil) {
                    print!("{val}");
                } else {
                    print!("{val} -> ");
                }
                next.print();
            }
        }
    }
}

// Binary Search Tree (BST)
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>, // recursive
    right: Option<Box<TreeNode>>, // recursive
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {value, left: None, right: None}
    }

    fn insert(node: Option<Box<TreeNode>>, val: i32) -> Box<TreeNode> {
        match node {
            None => Box::new(TreeNode::new(val)),
            Some(mut n) => {
                if val < n.value {
                    n.left = Some(TreeNode::insert(n.left.take(), val));
                } else if val > n.value {
                    n.right = Some(TreeNode::insert(n.right.take(), val));
                }
                n
            }
        }
    }

    // In-order traversal: left -> root -> right
    fn in_order(&self) {
        if let Some(left) = &self.left {
            left.in_order();
        }
        print!("{} ", self.value);
        if let Some(right) = &self.right {
            right.in_order();
        }
    }

    fn search(&self, val: i32) -> bool {
        if val == self.value {
            return true;
        }
        if val < self.value {
            self.left.as_ref().map_or(false, |n| n.search(val))
        } else {
            self.right.as_ref().map_or(false, |n| n.search(val))
        }
    }
}

// JSON-like Tree (recursive enum - Rust's strength)
enum JSONValue {
    Primitive(String),
    Object(Vec<(String, JSONValue)>),  // recursive: values are JSONValues
    Array(Vec<JSONValue>),             // recursive: values are JSONValues
}

impl JSONValue {
    fn print(&self, indent: usize) {
        let pad = " ".repeat(indent);
        match self {
            JSONValue::Primitive(raw) => println!("{pad}{raw}"),
            JSONValue::Object(pairs) => {
                println!("{pad}{{");
                for (key, val) in pairs {
                    println!("{pad} {key:?}:");
                    val.print(indent + 1);
                }
                println!("{pad}}}");
            }
            JSONValue::Array(items) => {
                println!("{pad}[");
                for item in items {
                    item.print(indent + 2);
                }
                println!("{pad}]");
            }
        }
    }
}

fn main() {}