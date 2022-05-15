use std::{iter::Step, ops::AddAssign};

use crate::traitbounds::{Integer, Real};

pub fn sum<T: Default + Integer + Step, R: Default + AddAssign>(
    from: T,
    to: T,
    function: fn(T) -> R,
) -> R {
    if from > to {
        panic!("From must be larger than to.");
    }
    let mut result = R::default();
    for i in from..=to {
        result += function(i);
    }
    result
}

pub fn sum_extra<T: Default + Integer + Step, E, R: Default + Real>(
    from: T,
    to: T,
    function: fn(T, &E) -> R,
    other: E,
) -> R {
    if from > to {
        panic!("From must be larger than to.");
    }
    let mut result = R::default();
    for i in from..=to {
        result += function(i, &other);
    }
    result
}

#[test]
fn summation_test() {
    let x = sum(1, 4, |x| 2 * x);
    assert_eq!(20, x);

    let y = sum_extra(1, 4, |x, e| x * e[x - 1], vec![1, 2, 3, 4]);
    assert_eq!(30, y)
}
