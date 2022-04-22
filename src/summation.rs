use std::iter::Step;

use crate::traitbounds::{Integer};

#[test]
fn summation_test() {
    let x = sum(1, 4, |x| { 2*x });
    assert_eq!(20, x);
}

pub fn sum<T: From<u8> + Integer + Step>(from: T, to: T, function: fn(T) -> T) -> T {
    if from > to {
        return T::from(0);
    }
    let mut result = T::from(0);
    for i in from..=to {
        result += function(i);
    }
    result
}