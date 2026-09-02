fn main() {

}

fn midpoint(lo: i32, hi: i32) -> i32 {
    lo + (hi - lo) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow() {
        let m = midpoint(7, i32::MAX);
        assert!(m >= 7 && m <= i32::MAX);
    }
}