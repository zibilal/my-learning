fn main() {
    let q = &mut Queue::new();
    q.push_back('z');
    q.push_back('i');
    q.push_back('b');
    q.push_back('i');
    q.push_back('l');
    q.push_back('a');
    q.push_back('l');

    println!("{:?}", q);
    assert_eq!(q.pop_front(), Some('l'));
    println!("{:?}", q);
}

#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push_back(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop_front(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            std::mem::swap(&mut self.older, &mut self.younger);
        }

        self.older.pop()
    }
}