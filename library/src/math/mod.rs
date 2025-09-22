use num::zero;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

// Find GCD
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq + Rem<Output = T> + PartialOrd + Copy + num::One + num::Zero,
{
    while b != zero() {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Eq
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + From<u64>
        + Div
        + Mul<<T as Div>::Output, Output = T>
        + num::One
        + num::Zero,
{
    a * (b / gcd(a, b))
}

pub fn sawtooth<T>(number: T, max: T) -> T
where
    T: PartialOrd + Rem<Output = T> + Add<Output = T> + Copy + num::Zero,
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
