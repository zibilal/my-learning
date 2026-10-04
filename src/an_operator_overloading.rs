use std::ops::Add;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,

    /// Imaginary portion of the complex number
    im: T,
}

impl<L, R> Add<Complex<R>> for Complex<L>
where
L: Add<R>,{
    type Output = Complex<L::Output>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
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