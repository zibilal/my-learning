use std::ops::{Add, Sub};
use std::ops::Mul;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,

    /// Imaginary portion of the complex number
    im: T,
}

impl <T> Add for Complex<T>
where
    T: Add<Output=T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl <T> Sub for Complex<T>
where
    T: Sub<Output=T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Mul<Output=T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: (self.re * rhs.re) - (self.im * rhs.im),
            im: (self.re * rhs.im) + (self.im * rhs.re),
        }
    }
}

impl<T> Neg for Complex<T>
where
    T:Neg<Output = T>
{
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

#[test]
fn test_add_traits() {
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
}

#[test]
fn test_add_complex_numbers() {
    let a = Complex { re: 1.0, im: 2.0};
    let b = Complex { re: 3.5, im: -1.0};

    assert_eq!(a.add(b), a + b);
}

#[test]
fn test_neg_complex_numbers() {
    let a = Complex { re: 1.0, im: -2.0};
    assert_eq!(a.neg(), -a);
}

#[test]
fn test_partial_eq() {
    let x = Complex { re: 3, im: 4 };
    let y = Complex { re: 0, im: 1};
    assert_eq!(x * y, Complex { re: -4, im: 3 });
}

#[test]
fn test_equal_string() {
    let s = "d\x6fv\x65t\x61i\x6c".to_string();
    let t = "\x64o\x76e\x74a\x69l".to_string();

    assert!(s == t); // as and are only borrowed...

    // ... so they still have their values here
    assert_eq!(format!("{s} {t}"), "dovetail dovetail");
}