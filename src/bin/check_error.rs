fn main() {
    let mut data = vec![1, 2, 3];

    let borrow1 = &data;
    println!("First {:?}", borrow1);

    data.push(4);
    println!("Mutation: {:?}", data);
}