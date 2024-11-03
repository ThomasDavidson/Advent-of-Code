use num::{zero, one};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

// Find GCD
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq
    + std::ops::Rem<Output = T>
    + PartialOrd
    + Copy
    + num::One,
{
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > one() {
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
        + std::ops::Mul<<T as std::ops::Div>::Output, Output = T>
        + num::One,
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

pub fn round_to<T>(num_to_round: T, multiple: T) -> T
where
    T: PartialOrd
        + Rem<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + From<u8>
        + Copy,
{
    ((num_to_round + multiple - T::from(1)) / multiple) * multiple
}
