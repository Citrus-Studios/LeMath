use std::{iter::Step};

use crate::traitbounds::{Integer, Real};

#[test]
fn summation_test() {
    let x = sum(1, 4, |x| { 2*x });
    assert_eq!(20, x);
}

pub fn sum<T: Default + Integer + Step>(from: T, to: T, function: fn(T) -> T) -> T {
    if from > to {
        panic!("From must be larger than to.");
    }
    let mut result = T::default();
    for i in from..=to {
        result += function(i);
    }
    result
}

#[test]
fn summation_output_test() {
    let x = sum_output(1, 4, |x| -> i64 { 2*x as i64 });
    assert_eq!(20, x);
}

pub fn sum_output<T: Integer + Step, U: Default + Real>(from: T, to: T, function: fn(T) -> U) -> U {
    if from > to {
        panic!("From must be larger than to.");
    }
    let mut result = U::default();
    for i in from..=to {
        result += function(i);
    }
    result
}

#[test]
fn summation_extra_test() {
    let y = vec![10, 20, 30, 40];
    let x = sum_extra(1, 4, |x, y| { 
        2*x + y[x-1]
    }, y);
    assert_eq!(120, x);
}

pub fn sum_extra<T: Default + Integer + Step, U>(from: T, to: T, function: fn(T, &U) -> T, other: U) -> T {
    if from > to {
        panic!("From must be larger than to.");
    }
    let mut result = T::default();
    for i in from..=to {
        result += function(i, &other);
    }
    result
}

pub fn sum_extra_output<T: Default + Integer + Step, U, V: Default + Real>(from: T, to: T, function: fn(T, &U) -> V, other: U) -> V {
    if from > to {
        panic!("From must be larger than to.");
    }
    let mut result = V::default();
    for i in from..=to {
        result += function(i, &other);
    }
    result
}