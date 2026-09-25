fn main() {}

fn print_nums(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

#[cfg(test)]
mod tests {
    use crate::print_nums;

    #[test]
    fn test_vector_with_capacity() {
        let mut v : Vec<i32> = Vec::with_capacity(2);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 2);

        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);

        v.push(3);
        assert_eq!(v.len(), 3);

        // Typically prints "capacity is now 4";
        println!("capacity is now {}", v.capacity());
    }

    #[test]
    fn test_vector_with_insert_and_remove() {
        let mut v = vec![10, 20, 30, 40, 50];

        // Make the element at index 3 be 35
        v.insert(3, 35);
        assert_eq!(v, [10, 20, 30, 35, 40, 50]);

        // Remove the element at index 1
        v.remove(1);
        assert_eq!(v, [10, 30, 35, 40, 50]);
    }

    #[test]
    fn test_vector_pop() {
        let mut v = vec!["Snow Puff", "Glass Gem"];
        assert_eq!(v.pop(), Some("Glass Gem"));
        assert_eq!(v.pop(), Some("Snow Puff"));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_slice() {
        let v = vec![0.0, 0.707, 1.0, 0.707];
        let a = [0.0, -0.707, -1.0, -0.707];

        let sv: &[f64] = &v;
        let sa: &[f64] = &a;

        print_nums(&a);
        print_nums(&v);

        print_nums(&v[0..2]);  // print the first two elements of v
    }

    #[test]
    fn test_vector_sort() {
        let mut chaos = vec![3, 5, 4, 1, 2];
        chaos.sort();
        assert_eq!(chaos, [1, 2, 3, 4, 5]);
    }
}