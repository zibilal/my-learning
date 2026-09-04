fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", sum_arr(&a));
}

fn sum_arr(a: &[i32]) -> i32 {
    a.iter().sum()
}