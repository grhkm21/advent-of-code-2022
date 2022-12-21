use lazy_static::lazy_static;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};
use std::cmp::max;

pub trait Numeric = Sized + Copy + Clone + Add<Output = Self> + Default;

#[derive(Clone)]
pub struct Poly<T: Numeric> {
    coef: Vec<T>,
}

impl<T> Poly<T> where T: Numeric{
    pub fn new(coef: Vec<T>) -> Poly<T> {
        if coef.is_empty() {
            return Poly {
                coef: vec![Default::default()]
            }
        }
        Poly {
            coef: coef
        }
    }

    fn deg(&self) -> usize {
        self.coef.len() - 1
    }

    pub fn add(&self, other: &Poly<T>) -> Poly<T> {
        let m = self.deg();
        let n = other.deg();
        let mut coef = vec![Default::default(); max(m, n) + 1];
        for i in 0..=max(m, n) {
            if i <= m {
                coef[i] = coef[i] + self.coef[i];
            }
            if i <= n {
                coef[i] = coef[i] + other.coef[i];
            }
        }
        Poly::new(coef)
    }
}

#[cfg(test)]
mod tests {
    lazy_static! {
        // TODO: Add f64 polynomials
        static ref EMPTY_COEF = vec![];
        static ref ONE_COEF = vec![1];
        static ref POLY1_COEF = vec![1, 2, 3];
        static ref POLY2_COEF = vec![6, 0, 4, 3];

        static ref ZERO = Poly::new(EMPTY_COEF);
        static ref ONE = Poly::new(ONE_COEF);
        static ref POLY1 = Poly::new(POLY1_COEF);
        static ref POLY2 = Poly::new(POLY2_COEF);
    }

    #[test]
    fn test_constructors() {
        assert_eq!(ZERO.coef, vec![0]);
        assert_eq!(POLY1.coef, POLY1_COEF);
    }

    #[test]
    fn test_deg() {
        assert_eq!(self.ZERO.deg(), 0);
        assert_eq!(ONE.deg(), 0);
        assert_eq!(POLY1.deg(), 2);
        assert_eq!(POLY2.deg(), 3);
    }
}
