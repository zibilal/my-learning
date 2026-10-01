use std::f64::consts::FRAC_PI_2;

fn main() {}

struct Polynomial<const N: usize> {
    coefficients: [f64;N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64;N]) -> Self {
        Self { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}



#[test]
fn test_polynomial() {
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0/6.0, 0.0, 1.0/120.0]);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() < 0.005);
}