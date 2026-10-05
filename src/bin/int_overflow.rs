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

    #[test]
    fn test_wrapping_ops() {
        assert_eq!(100_u16.wrapping_mul(200), 20000);
        assert_eq!(500_u16.wrapping_mul(200), 34464);

        assert_eq!(500_i16.wrapping_mul(500), -12144);
    }

    #[test]
    fn test_strict_ops() {
        let result = std::panic::catch_unwind(|| {
            let x = usize::MAX.strict_mul(2);
        });
        assert!(result.is_err(), "expected midpoint to panic on overflow");
    }

    #[test]
    fn test_checked_ops() {
        assert_eq!(10_u8.checked_add(20), Some(30));
        assert_eq!(100_u8.checked_add(200), None);
        assert_eq!((-128_i8).checked_div(-1), None);
    }

    #[test]
    fn test_overflowing_ops() {
        assert_eq!(255_u8.overflowing_sub(2), (253, false));
        assert_eq!(255_u8.overflowing_add(2), (1, true));
    }
}