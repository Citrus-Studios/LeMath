use std::ops::{Add, Mul};

use crate::matrix_items::matrix::Matrix;

use super::vectors::{Vector, VectorGeneric};

/// Dot Product
impl<T: VectorGeneric<T>> Mul<Self> for Vector<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot_product(rhs)
    }
}

/// Scalar Product
impl<T: VectorGeneric<T>> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.scalar_mul(rhs)
    }
}

impl<T: VectorGeneric<T>> Mul<Matrix<T>> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: VectorGeneric<T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let larger = {
            if self.len() > rhs.len() {
                (self.len(), false);
            }
            (rhs.len(), true)
        };
        if larger.1 {
            let mut cloned = self.clone();
            for (i, _) in self.into_iter().enumerate() {
                cloned[i] += rhs[i];
            }
            return cloned;
        } else {
            let mut cloned = rhs.clone();
            for (i, _) in rhs.into_iter().enumerate() {
                cloned[i] += self[i];
            }
            return cloned;
        }
    }
}
