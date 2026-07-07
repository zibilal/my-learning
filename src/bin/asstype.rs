trait Container {
    type Item; // associated type placeholder
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn first(&self) -> Option<&Self::Item> {
        self.get(0) // default method - works for ANY implementor
    }
}

struct IntBox(Vec<i32>);
impl Container for IntBox {
    type Item = i32;
    fn get(&self, index: usize) -> Option<&i32> {
        self.0.get(index)
    }
}

struct StringBox(Vec<String>);
impl Container for StringBox {
    type Item = String;

    fn get(&self, index: usize) -> Option<&String> {
        self.0.get(index)
    }
}
fn main() {
    let ints = IntBox(vec![10, 20, 30]);
    println!("{:?}", ints.get(1));
    println!("{:?}", ints.first());

    let strings = StringBox(vec![String::from("hello"), String::from("world")]);
    println!("{:?}", strings.get(1));
    println!("{:?}", strings.first());
}