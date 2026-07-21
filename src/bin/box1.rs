#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}
use List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    println!("-----------");
    print_all(&list);
}

fn print_all(list: &List) {
    match list {
        Cons(value, rest) => {
            print!("value: {:?}", value);
            print!(" rest: {:?} ", rest);
            print_all(rest);
        },
        Nil => {},
    }
}