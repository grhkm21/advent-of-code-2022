use std::ops::{Div, Mul, Rem};

pub fn gcd<T>(x: T, y: T) -> T
where
    T: PartialEq + PartialOrd + PartialOrd<i64> + Rem<Output = T> + Copy,
{
    if x < y {
        return gcd(y, x);
    }

    let mut x = x;
    let mut y = y;
    while y > 0 {
        let tmp = x % y;
        x = y;
        y = tmp;
    }

    x
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: PartialEq
        + PartialOrd
        + PartialOrd<i64>
        + Div<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + Copy,
{
    // Reduces chance of overflow
    let mut g = gcd(x, y);
    let mut x = x;
    let g1 = gcd(g, x);
    g = g / g1;
    x = x / g1;
    y / g * x
}
