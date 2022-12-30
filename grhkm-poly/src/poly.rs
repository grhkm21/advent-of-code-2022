use itertools::Itertools;
use std::cmp::max;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

pub trait NumOps<Rhs, Output> = Sized
    + Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>;

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

pub trait Neg {
    type Output;
    fn neg(&self) -> Self::Output;
}

// Automatically implemented
pub trait SignedNumeric: Numeric + Neg<Output = Self> {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn abs(&self) -> Self;
}

impl<T> SignedNumeric for T
where
    T: Numeric + Neg<Output = Self>,
{
    fn is_positive(&self) -> bool {
        *self > T::zero()
    }

    fn is_negative(&self) -> bool {
        *self < T::zero()
    }

    fn abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            *self
        }
    }
}

// TODO: Make this into a macro, much work needed
pub trait SignedInt: SignedNumeric + Ord + Rem<Self, Output = Self> {
    fn two() -> Self;
    fn unsigned_abs(&self) -> Self;
}

impl SignedInt for i8 {
    fn two() -> Self {
        2
    }

    fn unsigned_abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            *self
        }
    }
}

impl SignedInt for i16 {
    fn two() -> Self {
        2
    }

    fn unsigned_abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            *self
        }
    }
}

impl SignedInt for i32 {
    fn two() -> Self {
        2
    }

    fn unsigned_abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            *self
        }
    }
}

impl SignedInt for i64 {
    fn two() -> Self {
        2
    }

    fn unsigned_abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            *self
        }
    }
}

impl SignedInt for i128 {
    fn two() -> Self {
        2
    }

    fn unsigned_abs(&self) -> Self {
        if self.is_negative() {
            self.neg()
        } else {
            *self
        }
    }
}

// TODO: Somehow make Copy / Clone not necessary
pub trait Numeric = Zero + One + Copy + Clone + NumOps<Self, Self> + PartialOrd<Self> + Display;

pub enum DisplayCoefType {
    Pos(String),
    Neg(String),
    Zero,
    One,
}

pub trait DisplayCoef {
    fn display_coef(&self) -> DisplayCoefType;
}

impl<T> DisplayCoef for T
where
    T: Numeric,
{
    default fn display_coef(&self) -> DisplayCoefType {
        if self.is_zero() {
            DisplayCoefType::Zero
        } else if self.is_one() {
            DisplayCoefType::One
        } else {
            DisplayCoefType::Pos(format!("{self}"))
        }
    }
}

impl<T> DisplayCoef for T
where
    T: SignedNumeric,
{
    fn display_coef(&self) -> DisplayCoefType {
        if self.is_zero() {
            DisplayCoefType::Zero
        } else if self.is_one() {
            DisplayCoefType::One
        } else if self.is_positive() {
            DisplayCoefType::Pos(format!("{self}"))
        } else {
            if self.abs().is_one() {
                DisplayCoefType::Neg("".to_string())
            } else {
                DisplayCoefType::Neg(format!("{}", self.abs()))
            }
        }
    }
}

// Implements Zero, One, Numeric
macro_rules! numeric_trait_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                0
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }

        impl One for $t {
            #[inline]
            fn one() -> Self {
                1
            }

            #[inline]
            fn is_one(&self) -> bool {
                *self == 1
            }
        }
    )*)
}

// Implements Zero, One, Numeric, SignedNumeric
macro_rules! signed_numeric_trait_impl {
    ($($t:ty)*) => {
        $(
            numeric_trait_impl!($t);
            impl Neg for $t {
                type Output = Self;

                #[inline]
                fn neg(&self) -> Self {
                    -self
                }
            }
    )*
    }
}

numeric_trait_impl!(u8 u16 u32 u64 u128);
signed_numeric_trait_impl!(i8 i16 i32 i64 i128);

pub trait Solvable<T> {
    fn roots(&self) -> Vec<T>;
}

#[derive(Clone)]
pub struct Poly<T: Numeric> {
    coef: Vec<T>,
}

// Specialized Debug for integer-like polynomials
impl<T> Display for Poly<T>
where
    T: Numeric,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.coef.len() == 1 && self.coef[0].is_zero() {
            return write!(f, "0");
        }

        let mut coef_iter = self.coef.iter().enumerate();

        while let Some((i, c)) = coef_iter.next() {
            let c = (*c).display_coef();
            if let DisplayCoefType::Zero = c {
                continue;
            } else {
                match c {
                    DisplayCoefType::One => write!(f, " + ")?,
                    DisplayCoefType::Pos(val) => write!(f, " + {val}")?,
                    DisplayCoefType::Neg(val) => write!(f, " - {val}")?,
                    _ => unreachable!(),
                };
            }

            if i > 0 {
                write!(f, "x")?;
                if i > 1 {
                    write!(f, "^{}", i)?;
                }
            }
        }
        Ok(())
    }
}

impl<T> Debug for Poly<T>
where
    T: Numeric + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.coef)
    }
}

impl<T> Poly<T>
where
    T: Numeric,
{
    pub fn new(coef: &Vec<T>) -> Poly<T> {
        if coef.is_empty() {
            return Poly {
                coef: vec![T::zero()],
            };
        }
        Poly { coef: coef.clone() }
    }

    pub fn from_const(c: T) -> Poly<T> {
        Poly { coef: vec![c] }
    }

    pub fn is_const(&self) -> bool {
        self.deg() == 0
    }

    pub fn deg(&self) -> usize {
        self.coef.len() - 1
    }

    pub fn get(&self, i: usize) -> T {
        self.coef[i]
    }

    pub fn add(&self, other: &Poly<T>) -> Poly<T> {
        let m = self.deg();
        let n = other.deg();
        let mut coef = vec![T::zero(); max(m, n) + 1];

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
        let mut coef = vec![T::zero(); max(m, n) + 1];

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
        let mut coef = vec![T::zero(); m + n + 1];
        for i in 0..=m {
            for j in 0..=n {
                coef[i + j] = coef[i + j] + self.get(i) * other.get(j);
            }
        }
        Poly::new(&coef)
    }

    pub fn div(&self, other: &Poly<T>) -> Poly<T> {
        let m = self.deg();
        let n = other.deg();

        if n != 0 {
            todo!("TODO: Cannot divide by non-constant polynomial {other}");
        }

        let mut coef = self.coef.clone();
        let c = other.coef[0];

        for i in 0..=m {
            coef[i] = coef[i] / c;
        }

        Poly::new(&coef)
    }

    pub fn eval<U>(&self, x: U) -> T
    where
        U: Into<T>,
    {
        let x = x.into();
        let mut res = T::zero();
        for coef in self.coef.iter().rev() {
            res = res * x + *coef;
        }
        res
    }

    pub fn eval_as<U>(&self, x: U) -> U
    where
        U: Numeric + From<T>,
    {
        let mut res = U::zero();
        for coef in self.coef.iter().rev() {
            res = res * x + Into::<U>::into(*coef);
        }
        res
    }
}

// Solving polynomials over signed integers
impl<T> Solvable<T> for Poly<T>
where
    T: SignedInt,
{
    fn roots(&self) -> Vec<T> {
        // Roots must be in the form a / b where a | coef[0] and b | coef[d - 1]
        // For integer case it suffices to consider b = 1
        if self.deg() == 0 {
            // TODO: Change this to return error (using `anyhow` or something)
            return vec![];
        }

        let coef_first = self.get(0usize);

        let factors = Factor::positive_divisors(coef_first);
        let neg_factors = factors.clone();
        let neg_factors = neg_factors.iter().rev().map(|c| c.neg());

        let mut res = (neg_factors.chain(factors.into_iter()))
            .filter(|r| self.eval_as(*r).is_zero())
            .collect::<Vec<T>>();
        if coef_first.is_zero() {
            res.push(T::zero());
        }

        res
    }
}

// Solving polynomials over signed coefficients
// TODO: (ETA: Never) Replace "Signed" with "Complete"
impl<T> Solvable<T> for Poly<T>
where
    T: SignedNumeric,
{
    default fn roots(&self) -> Vec<T> {
        if self.deg() == 0 {
            // TODO: Change this to return error (using `anyhow` or something)
            return vec![];
        }

        if self.deg() == 1 {
            let root = self.get(0).neg() / self.get(1);
            return vec![root.into()];
        }

        todo!()
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

impl<T> Div for Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn div(self, other: Self) -> Self::Output {
        &self / &other
    }
}

impl<T> Div for &Poly<T>
where
    T: Numeric,
{
    type Output = Poly<T>;

    fn div(self, other: Self) -> Self::Output {
        Poly::div(&self, &other)
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
        *self = self.clone() - other;
    }
}

impl<T> MulAssign for Poly<T>
where
    T: Numeric,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T> DivAssign for Poly<T>
where
    T: Numeric,
{
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

// Factor numbers

#[derive(Copy, Clone)]
struct PowerIter<T>
where
    T: SignedInt,
{
    base: T,
    buf: T,
    cur_exp: u128,
    final_exp: u128,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct FactorItem<T>(T, u128)
where
    T: SignedInt;

struct Factor;

impl<T> Iterator for PowerIter<T>
where
    T: SignedInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_exp > self.final_exp {
            None
        } else {
            let res = Some(self.buf);
            self.buf = self.buf * self.base;
            self.cur_exp += 1;
            res
        }
    }
}

impl<T> IntoIterator for FactorItem<T>
where
    T: SignedInt,
{
    type Item = T;
    type IntoIter = PowerIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        PowerIter {
            base: self.0,
            buf: T::one(),
            cur_exp: 0,
            final_exp: self.1,
        }
    }
}

impl Factor {
    fn prime_factorisation<T>(mut n: T) -> Vec<FactorItem<T>>
    where
        T: SignedInt,
    {
        // Turns 1 -> {}
        // Turns 12 -> {(2, 2), (3, 1)}
        // Turns -15 -> {(-1, 1), (3, 1), (5, 1)}
        // Raises error for 0

        if n.is_zero() {
            panic!("err: Attempting to factor 0");
        }

        let mut res = Vec::new();
        let mut p = T::two();

        if n.is_negative() {
            res.push(FactorItem(T::one().neg(), 1));
            n = n.neg();
        }

        while p * p <= n {
            let mut c = 0;
            while n % p == T::zero() {
                n = n / p;
                c += 1;
            }
            if c > 0 {
                res.push(FactorItem(p, c));
            }
            p = p + T::one();
        }

        if n > T::one() {
            res.push(FactorItem(n, 1));
        }

        res
    }

    fn positive_divisors<T>(n: T) -> Vec<T>
    where
        T: SignedInt,
    {
        // Turns 1 -> {1}
        // Turns 12 -> {1, 2, 3, 4, 6, 12}
        // Turns -15 -> {1, 3, 5, 15}
        let mut iter = Self::prime_factorisation(n.unsigned_abs());
        if iter.is_empty() {
            iter.push(FactorItem(T::one(), 1));
        }
        let mut res = iter
            .into_iter()
            .multi_cartesian_product()
            .map(|v| v.iter().copied().reduce(|a, b| a * b).unwrap())
            .collect::<Vec<T>>();

        // TODO: Make this optional (and fix tests)
        res.sort();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::{Factor, FactorItem, Poly, Solvable};
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
        static ref EMPTY_COEF: Vec<i128> = vec![];
        static ref ONE_COEF: Vec<i128> = vec![1];
        static ref POLY1_COEF: Vec<i128> = vec![1, 2, 3];
        static ref POLY2_COEF: Vec<i128> = vec![6, 0, 4, 3];

        static ref ZERO: Poly<i128> = Poly::new(&*EMPTY_COEF);
        static ref ONE: Poly<i128> = Poly::new(&*ONE_COEF);
        static ref POLY1: Poly<i128> = Poly::new(&*POLY1_COEF);
        static ref POLY2: Poly<i128> = Poly::new(&*POLY2_COEF);
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

    // Polynomial root finding
    #[test]
    fn test_roots_integer() {
        // (x + 1)(x + 2), (2x + 1)(x + 2)
        // TODO: Actually write it properly when I am not in a hurry
        assert_eq!(Poly::<i128>::new(&vec![2, 3, 1]).roots(), vec![-2, -1]);
        assert_eq!(Poly::<i128>::new(&vec![2, 5, 2]).roots(), vec![-2]);
        assert_eq!(Poly::<i128>::new(&vec![1, 0, 1]).roots(), vec![]);
    }

    // Factoring

    fn into_factoritem_vec<const N: usize>(v: [(i128, u128); N]) -> Vec<FactorItem<i128>> {
        v.iter()
            .map(|(p, e)| FactorItem(*p, *e))
            .collect::<Vec<_>>()
    }

    #[test_case(12, [(2, 2), (3, 1)] ; "positive")]
    #[test_case(-15, [(-1, 1), (3, 1), (5, 1)] ; "negative")]
    #[test_case(162179607919826590230182726147616596160, [(2, 6), (3, 8), (5, 1), (7, 5), (23, 13), (97, 4), (103, 1)]; "large")]
    #[test_case(835991099723193079, [(835991099723193079, 1)] ; "60-bit prime")]
    fn test_factorisation<const N: usize>(num: i128, expected: [(i128, u128); N]) {
        assert_eq_vec!(
            Factor::prime_factorisation(num),
            into_factoritem_vec(expected)
        );
    }

    #[test_case(20, 6, Some(vec![1, 2, 4, 5, 10, 20]) ; "positive")]
    #[test_case(-18, 6, Some(vec![1, 2, 3, 6, 9, 18]) ; "negative")]
    #[test_case(620212131694574261856000000000000, 131040, None ; "lots of divisors")]
    #[test_case(835991099723193079, 2, Some(vec![1, 835991099723193079]) ; "60-bit prime")]
    fn test_positive_divisors(num: i128, divisors_len: usize, divisors: Option<Vec<i128>>) {
        let num_divisors = Factor::positive_divisors(num);
        assert_eq!(num_divisors.len(), divisors_len);
        if let Some(divisors) = divisors {
            assert_eq_vec!(num_divisors, divisors);
        }
    }

    #[test]
    // TODO: Test negative coefficients, floats
    fn test_debug() {
        assert_eq!(format!("{}", *POLY1), "1 + 2x + 3x^2");
        assert_eq!(format!("{}", *POLY2), "6 + 4x^2 + 3x^3");
        assert_eq!(format!("{}", *ZERO), "0");
    }

    // TODO: (Probably in separate file) add tests for Fraction and float polynomials
}
