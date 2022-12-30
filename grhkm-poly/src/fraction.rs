use super::poly::{Neg, One, Zero};
use std::fmt::Display;
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq)]
pub struct Fraction {
    pub num: i128,
    pub denom: i128,
}

impl Fraction {
    // Users MUST use the new constructor instead of the {} constructor
    // Otherwise, unexpected behaviours may occur
    pub fn new(num: i128, denom: i128) -> Fraction {
        let mut frac = Fraction { num, denom };

        // Handles denom == 0
        frac.reduce();
        frac
    }

    pub fn num(&self) -> i128 {
        self.num
    }

    pub fn denom(&self) -> i128 {
        self.denom
    }

    fn gcd(x: i128, y: i128) -> i128 {
        let mut x = x.abs();
        let mut y = y.abs();

        if x < y {
            swap(&mut x, &mut y);
        }

        while y > 0 {
            let tmp = x % y;
            x = y;
            y = tmp;
        }

        x
    }

    fn reduce(&mut self) {
        let mut g = Fraction::gcd(self.num, self.denom);
        if g == 0 {
            panic!("Error: gcd is 0 for {} / {}", self.num, self.denom);
        }

        if self.denom < 0 {
            g *= -1;
        }

        self.num /= g;
        self.denom /= g;
    }

    fn add(lhs: Fraction, rhs: Fraction) -> Fraction {
        Fraction::new(
            lhs.num * rhs.denom + lhs.denom * rhs.num,
            lhs.denom * rhs.denom,
        )
    }

    fn sub(lhs: Fraction, rhs: Fraction) -> Fraction {
        Fraction::new(
            lhs.num * rhs.denom - lhs.denom * rhs.num,
            lhs.denom * rhs.denom,
        )
    }

    fn mul(lhs: Fraction, rhs: Fraction) -> Fraction {
        Fraction::new(lhs.num * rhs.num, lhs.denom * rhs.denom)
    }

    fn div(lhs: Fraction, rhs: Fraction) -> Fraction {
        Fraction::new(lhs.num * rhs.denom, lhs.denom * rhs.num)
    }
}

// TODO: Implement == and other operations for references
impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let cross1 = self.num * other.denom;
        let cross2 = self.denom * other.num;
        cross1 == cross2
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cross1 = self.num * other.denom;
        let cross2 = self.denom * other.num;
        Some(cross1.cmp(&cross2))
    }
}

impl Add for Fraction {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub for Fraction {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul for Fraction {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Div for Fraction {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = Fraction::add(*self, rhs);
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Fraction::sub(*self, rhs);
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Fraction::mul(*self, rhs);
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = Fraction::div(*self, rhs);
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.num, self.denom)
    }
}

// Numeric Traits

impl Zero for Fraction {
    fn zero() -> Self {
        Fraction::new(0, 1)
    }
    fn is_zero(&self) -> bool {
        self.num() == 0
    }
}

impl One for Fraction {
    fn one() -> Self {
        Fraction::new(1, 1)
    }
    fn is_one(&self) -> bool {
        self.num() == self.denom()
    }
}

impl Neg for Fraction {
    type Output = Self;
    fn neg(&self) -> Self::Output {
        Fraction::new(-self.num(), self.denom())
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    // TODO: Organise this better
    #[test]
    fn test_ordering() {
        let f1 = Fraction::new(1, 3);
        let f2 = Fraction::new(1, 2);
        let f3 = Fraction::new(2, 4);

        assert!(f2 == f3);
        assert!(f1 < f2);
        assert!(f2 > f1);
        assert!(f1 != f2);
        assert!(f2 >= f3);
    }

    #[test]
    fn test_arithmetic() {
        let f1 = Fraction::new(1, 3);
        let f2 = Fraction::new(1, 2);
        // let f3 = Fraction::new(2, 4);

        assert_eq!(f1 + f2, Fraction::new(5, 6));
        assert_eq!(f1 - f2, Fraction::new(-1, 6));
        assert_eq!(f2 - f1, Fraction::new(1, 6));
        assert_eq!(f1 * f2, Fraction::new(1, 6));
        assert_eq!(f1 / f2, Fraction::new(2, 3));
        assert_eq!(f2 / f1, Fraction::new(3, 2));
    }
}
