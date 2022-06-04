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

impl<T: VectorGeneric<T>> Iterator for Matrix<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.contents.get_mut(self.idx) {
            Some(v) => match v.next() {
                Some(e) => Some(e),
                None => {
                    self.idx += 1;
                    if self.idx >= self.contents.len() {
                        return None;
                    }
                    self.next()
                }
            },
            None => {
                self.idx += 1;
                if self.idx >= self.contents.len() {
                    return None;
                }
                self.next()
            }
        }
    }
}

#[test]
fn matrix_iterator() {
    let x = crate::matrix!(10, 20, 30 => 40, 50, 60 => 70, 80, 90);
    for v in x {
        println!("{v}");
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

impl<T: VectorGeneric<T>> From<(usize, usize)> for Matrix<T> {
    fn from(from: (usize, usize)) -> Self {
        let mut temp = Matrix::<T>::new(vec![]);
        let default_vec = (0..from.1)
            .into_iter()
            .map(|_| T::default())
            .collect::<Vector<T>>();
        for _ in 0..from.0 {
            temp.contents.push(default_vec.clone());
        }
        temp
    }
}

impl<T: VectorGeneric<T>> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let temp_matrix = Matrix::<T>::from((self.contents.len(), self.contents[0].len()));

        todo!();
    }
}
