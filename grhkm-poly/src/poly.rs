use itertools::Itertools;
use std::cmp::max;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Numeric =
    Sized + Copy + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Default;

#[derive(Clone)]
pub struct Poly<T: Numeric> {
    coef: Vec<T>,
}

impl<T> Poly<T>
where
    T: Numeric,
{
    pub fn new(coef: &Vec<T>) -> Poly<T> {
        if coef.is_empty() {
            return Poly {
                coef: vec![Default::default()],
            };
        }
        Poly { coef: coef.clone() }
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

    pub fn eval<U>(&self, x: U) -> T
    where
        U: Into<T>,
    {
        let x = x.into();
        let mut res = Default::default();
        for coef in self.coef.iter().rev() {
            res = res * x + *coef;
        }
        res
    }

    pub fn eval_as<U>(&self, x: U) -> U
    where
        U: Numeric + From<T>,
    {
        let mut res = Default::default();
        for coef in self.coef.iter().rev() {
            res = res * x + Into::<U>::into(*coef);
        }
        res
    }
}

// Solving polynomials over signed integers
impl<T> Poly<T>
where
    T: Numeric + Into<i128>,
    i128: From<T>,
{
    pub fn roots(&self) -> Vec<i128> {
        // Roots must be in the form a / b where a | coef[0] and b | coef[d - 1]
        // For integer case it suffices to consider b = 1
        if self.deg() == 0 {
            // TODO: Change this to return error (using `anyhow` or something)
            return vec![];
        }

        let coef_first = self.get(0 as usize).into();
        println!("{coef_first:?}, ft: {:?}", Factor::positive_divisors(coef_first).iter().map(|r| vec![-(*r),*r]).flatten().collect::<Vec<_>>());

        let mut res: Vec<i128> = Factor::positive_divisors(coef_first)
            .iter()
            .map(|r| vec![-(*r), *r])
            .flatten()
            .filter(|r| self.eval_as(*r) == 0)
            .collect();
        if coef_first == 0 {
            res.push(0);
        }
        res
    }
}

// impl<T, U> Into<Poly<U>> for Poly<T> where T: Into<U> {
//     fn into(self) -> Poly<U> {
//         Poly {
//             coef: self.coef.into_iter().map(|t| t.into()).collect(),
//         }
//     }
// }

// Overlading operators
impl<T> Add for Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl<T> Add for &Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn add(self, other: Self) -> Self::Output {
        Poly::add(&self, &other)
    }
}

impl<T> Sub for Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl<T> Sub for &Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn sub(self, other: Self) -> Self::Output {
        Poly::sub(&self, &other)
    }
}

impl<T> Mul for Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl<T> Mul for &Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn mul(self, other: Self) -> Self::Output {
        Poly::mul(&self, &other)
    }
}

// Overloading assignment operators
impl<T> AddAssign for Poly<T>
where
    T: Numeric,
{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T> SubAssign for Poly<T>
where
    T: Numeric,
{
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T> MulAssign for Poly<T>
where
    T: Numeric,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

// Factor numbers

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct FactorItem(i128, u128);
struct Factor;

impl Iterator for FactorItem {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == u128::MAX {
            None
        } else if self.1 == 0 {
            self.1 = u128::MAX;
            Some(1)
        } else {
            self.1 -= 1;
            Some(self.0.pow((self.1 + 1) as u32))
        }
    }
}

impl Factor {
    fn prime_factorisation(n: i128) -> Vec<FactorItem> {
        // Turns 1 -> {}
        // Turns 12 -> {(2, 2), (3, 1)}
        // Turns -15 -> {(-1, 1), (3, 1), (5, 1)}
        // Raises error for 0

        if n == 0 {
            panic!("err: Attempting to factor 0");
        }

        let mut n = n;
        let mut res = Vec::new();
        let mut p = 2;

        if n < 0 {
            res.push(FactorItem(-1, 1));
            n *= -1;
        }

        while p * p <= n {
            if n % p == 0 {
                let mut c = 0;
                while n % p == 0 {
                    n /= p;
                    c += 1;
                }
                res.push(FactorItem(p, c));
            }
            p += 1;
        }

        if n > 1 {
            res.push(FactorItem(n, 1));
        }

        res
    }

    fn positive_divisors(n: i128) -> Vec<i128> {
        // Turns 1 -> {1}
        // Turns 12 -> {1, 2, 3, 4, 6, 12}
        // Turns -15 -> {1, 3, 5, 15}
        let mut iter = Self::prime_factorisation(n.unsigned_abs() as i128);
        if iter.is_empty() {
            iter.push(FactorItem(1, 1));
        }
        let mut res = iter.into_iter()
            .multi_cartesian_product()
            .map(|v| v.iter().copied().reduce(|a, b| a * b).unwrap())
            .collect::<Vec<i128>>();
        
        // TODO: Make this optional (and fix tests)
        res.sort();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::{Factor, FactorItem, Poly};
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    macro_rules! assert_eq_vec {
        ($left:expr, $right:expr) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    assert_eq!(left_val.len(), right_val.len());
                    for (left_elem, right_elem) in left_val.iter().zip(right_val.iter()) {
                        assert_eq!(left_elem, right_elem);
                    }
                }
            }
        };
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

    // Polynomial Arithmetic

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

    // Polynomail root finding
    #[test]
    fn test_roots_integer() {
        // (x + 1)(x + 2), (2x + 1)(x + 2)
        // TODO: Actually write it properly when I am not in a hurry
        // TODO: Fix .roots() return roots in wrong order
        let poly = Poly::new(&vec![2, 3, 1]);
        assert_eq!(poly.roots(), vec![-2, -1]);
        assert_eq!(Poly::new(&vec![2, 5, 2]).roots(), vec![-2]);
        assert_eq!(Poly::new(&vec![1, 0, 1]).roots(), vec![]);
    }

    // Factoring

    fn into_factoritem_vec(v: Vec<(i128, u128)>) -> Vec<FactorItem> {
        v.iter()
            .map(|(p, e)| FactorItem(*p, *e))
            .collect::<Vec<_>>()
    }

    #[test_case(12, vec![(2, 2), (3, 1)] ; "small positive case")]
    #[test_case(-15, vec![(-1, 1), (3, 1), (5, 1)] ; "negative case")]
    #[test_case(162179607919826590230182726147616596160, vec![(2, 6), (3, 8), (5, 1), (7, 5), (23, 13), (97, 4), (103, 1)]; "large case")]
    #[test_case(835991099723193079, vec![(835991099723193079, 1)] ; "60-bit prime case")]
    fn test_factorisation(num: i128, expected: Vec<(i128, u128)>) {
        assert_eq_vec!(
            Factor::prime_factorisation(num),
            into_factoritem_vec(expected)
        );
    }

    #[test]
    fn test_positive_divisors() {
        // TODO: Finish this
        println!("{:?}", Factor::positive_divisors(12));
        assert_eq_vec!(Factor::positive_divisors(12), vec![1, 2, 3, 4, 6, 12]);
        todo!();
    }
}
