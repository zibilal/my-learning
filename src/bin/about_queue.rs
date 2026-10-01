fn main() {
    let q = &mut Queue::new();
    q.push_back('b');
    q.push_back('i');

    println!("{:?}", q);

    println!("{:?}", q.pop_front().unwrap());
    println!("{:?}", q);
}

#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Self {
        Self { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push_back(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop_front(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            std::mem::swap(&mut self.older,&mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

#[test]
fn test_queue() {
    let mut q = Queue::new();

    q.push_back('0');
    q.push_back('1');
    assert_eq!(q.pop_front(), Some('0'));
    assert_eq!(q.younger, []);
    assert_eq!(q.older, ['1']);

    q.push_back('∞');
    assert_eq!(q.older, ['1']);
    assert_eq!(q.younger, ['∞']);
    assert_eq!(q.pop_front(), Some('1'));
    assert_eq!(q.older, []);
    assert_eq!(q.younger, ['∞']);
    assert_eq!(q.pop_front(), Some('∞'));
    assert_eq!(q.pop_front(), None);
}