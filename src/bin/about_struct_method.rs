struct Counter {
    value: std::cell::Cell<i32>, // interior mutability, so &Counter can still "mutate
}

trait Bump {
    fn bump(&mut self);
}

// impl 1: Self = Counter
impl Bump for Counter {
    fn bump(&mut self) {
        println!("bumping via &mut Counter");
        self.value.set(self.value.get() + 1);
    }
}

impl Bump for &Counter {
    fn bump(&mut self) {
        // here `self` has type `&mut &Counter`
        println!("bumping via &mut &Counter");
        self.value.set(self.value.get() + 1);
    }
}
fn main() {}