fn main() {}

#[cfg(test)]
mod tests {
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
}