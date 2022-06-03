use std::{fmt::Display, ops::Mul};

use crate::{
    linear_algebra::vectors::{Vector, VectorGeneric},
    vector,
};

use super::matrices::Matrix;

impl<T: VectorGeneric<T>> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{}", self.contents[0]).unwrap();
        } else {
            if self.contents.len() == 1 {
                write!(f, "{}", format!("{:?}", self.contents).replace(',', "")).unwrap();
            } else if self.contents.len() == 2 {
                write!(
                    f,
                    "⎡ {} ⎤\n",
                    format!("{}", self.contents[0])
                        .replace("[", "")
                        .replace("]", "")
                )
                .unwrap();
                write!(
                    f,
                    "⎣ {} ⎦",
                    format!("{}", self.contents[1])
                        .replace("[", "")
                        .replace("]", "")
                )
                .unwrap();
            } else {
                write!(
                    f,
                    "⎡ {} ⎤\n",
                    format!("{}", self.contents[0])
                        .replace("[", "")
                        .replace("]", "")
                )
                .unwrap();
                for x in 1..self.contents.len() - 1 {
                    write!(
                        f,
                        "⎢ {} ⎥\n",
                        format!("{}", self.contents[x])
                            .replace("[", "")
                            .replace("]", "")
                    )
                    .unwrap();
                }
                write!(
                    f,
                    "⎣ {} ⎦",
                    format!("{}", self.contents[self.contents.len() - 1])
                        .replace("[", "")
                        .replace("]", "")
                )
                .unwrap();
            }
        }
        f.write_str("")
    }
}

impl<T: VectorGeneric<T>> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        let mut temp_vec = vector![];
        for x in 0..self.len() {
            temp_vec.push(self.contents[x].clone() * rhs.clone());
        }
        temp_vec
    }
}
