use num::zero;
use std::ops::{Add, Rem};

// Find GCD
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Eq + std::ops::Rem<Output = T> + PartialOrd + TryFrom<u64> + Copy,
{
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > T::try_from(0).ok().unwrap() {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Eq
        + std::ops::Rem<Output = T>
        + PartialOrd
        + Copy
        + From<u64>
        + std::ops::Div
        + std::ops::Mul<<T as std::ops::Div>::Output, Output = T>,
{
    return a * (b / gcd(a, b));
}

pub fn sawtooth<T>(number: T, max: T) -> T
where
    T: PartialOrd
    + Rem<Output = T>
    + Add<Output = T>
    + Copy
    + num::Zero,
{
    if number < zero() {
        (max + number % max) % max
    } else {
        number % max
    }
}
