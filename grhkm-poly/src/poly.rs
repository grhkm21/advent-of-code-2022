use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::max;

pub trait Numeric = Sized + Copy + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Default;

#[derive(Clone)]
pub struct Poly<T: Numeric> {
    coef: Vec<T>,
}

impl<T> Poly<T> where T: Numeric {
    pub fn new(coef: &Vec<T>) -> Poly<T> {
        if coef.is_empty() {
            return Poly {
                coef: vec![Default::default()]
            }
        }
        Poly {
            coef: coef.clone()
        }
    }

    pub fn deg(&self) -> usize {
        self.coef.len() - 1
    }

    pub fn get(&self, i: impl Into<usize>) -> T {
        self.coef[i.into()]
    }

    pub fn add(&self, other: &Poly<T>) -> Poly<T> {
        let m = self.deg();
        let n = other.deg();
        let mut coef = vec![Default::default(); max(m, n) + 1];
        for i in 0..=max(m, n) {
            if i <= m {
                coef[i] = coef[i] + self.get(i);
            }
            if i <= n {
                coef[i] = coef[i] + other.get(i);
            }
        }
        Poly::new(&coef)
    }

    pub fn sub(&self, other: &Poly<T>) -> Poly<T> {
        let m = self.deg();
        let n = other.deg();
        let mut coef = vec![Default::default(); max(m, n) + 1];
        for i in 0..=max(m, n) {
            if i <= m {
                coef[i] = coef[i] + self.get(i);
            }
            if i <= n {
                coef[i] = coef[i] - other.get(i);
            }
        }
        Poly::new(&coef)
    }

    pub fn mul(&self, other: &Poly<T>) -> Poly<T> {
        let m = self.deg();
        let n = other.deg();
        let mut coef = vec![Default::default(); m + n + 1];
        for i in 0..=m {
            for j in 0..=n {
                coef[i + j] = coef[i + j] + self.get(i) * other.get(j);
            }
        }
        Poly::new(&coef)
    }
}

// Overlading operators
impl<T> Add for Poly<T> where T: Numeric {
    type Output = Poly<T>;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl<T> Add for &Poly<T> where T: Numeric {
    type Output = Poly<T>;

    fn add(self, other: Self) -> Self::Output {
        Poly::add(&self, &other)
    }
}

impl<T> Sub for Poly<T> where T: Numeric {
    type Output = Poly<T>;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl<T> Sub for &Poly<T> where T: Numeric {
    type Output = Poly<T>;

    fn sub(self, other: Self) -> Self::Output {
        Poly::sub(&self, &other)
    }
}

impl<T> Mul for Poly<T> where T: Numeric {
    type Output = Poly<T>;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl<T> Mul for &Poly<T> where T: Numeric {
    type Output = Poly<T>;

    fn mul(self, other: Self) -> Self::Output {
        Poly::mul(&self, &other)
    }
}

// Overloading assignment operators
impl<T> AddAssign for Poly<T> where T: Numeric {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T> SubAssign for Poly<T> where T: Numeric {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T> MulAssign for Poly<T> where T: Numeric {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

#[cfg(test)]
mod tests {
    use super::Poly;
    use lazy_static::lazy_static;
    use pretty_assertions::{assert_eq, assert_ne};

    macro_rules! assert_eq_vec {
        ($left:expr, $right:expr) => {
            match (&$left, &$right) {
                (left_val,right_val) => {
                    assert_eq!(left_val.len(), right_val.len());
                    for (left_elem, right_elem) in left_val.iter().zip(right_val.iter()) {
                        assert_eq!(left_elem, right_elem);
                    }
                }
            }
        }
    }

    lazy_static! {
        // TODO: Add f64 polynomials
        static ref EMPTY_COEF: Vec<i64> = vec![];
        static ref ONE_COEF: Vec<i64> = vec![1];
        static ref POLY1_COEF: Vec<i64> = vec![1, 2, 3];
        static ref POLY2_COEF: Vec<i64> = vec![6, 0, 4, 3];

        static ref ZERO: Poly<i64> = Poly::new(&*EMPTY_COEF);
        static ref ONE: Poly<i64> = Poly::new(&*ONE_COEF);
        static ref POLY1: Poly<i64> = Poly::new(&*POLY1_COEF);
        static ref POLY2: Poly<i64> = Poly::new(&*POLY2_COEF);
    }

    #[test]
    fn test_constructors() {
        assert_eq!(ZERO.coef, vec![0]);
        assert_eq_vec!(POLY1.coef, POLY1_COEF);
    }

    #[test]
    fn test_deg() {
        assert_eq!(ZERO.deg(), 0);
        assert_eq!(ONE.deg(), 0);
        assert_eq!(POLY1.deg(), 2);
        assert_eq!(POLY2.deg(), 3);
    }

    #[test]
    fn test_add() {
        let add_poly1_poly2 = &*POLY1 + &*POLY2;
        let add_poly2_poly1 = &*POLY2 + &*POLY1;
        assert_eq!(add_poly1_poly2.coef, vec![7, 2, 7, 3]);
        assert_eq!(add_poly2_poly1.coef, vec![7, 2, 7, 3]);
    }

    #[test]
    fn test_sub() {
        let sub_poly1_poly2 = &*POLY1 - &*POLY2;
        let sub_poly2_poly1 = &*POLY2 - &*POLY1;
        assert_eq!(sub_poly1_poly2.coef, vec![-5, 2, -1, -3]);
        assert_eq!(sub_poly2_poly1.coef, vec![5, -2, 1, 3]);
    }

    #[test]
    fn test_mul() {
        let mul_poly1_poly2 = &*POLY1 * &*POLY2;
        let mul_poly2_poly1 = &*POLY2 * &*POLY1;
        assert_eq!(mul_poly1_poly2.coef, vec![6, 12, 22, 11, 18, 9]);
        assert_eq!(mul_poly2_poly1.coef, vec![6, 12, 22, 11, 18, 9]);
    }
}

