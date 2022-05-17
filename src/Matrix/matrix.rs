use std::{fmt::Display, ops::Mul};

use crate::{
    vector,
    vectors::{Vector, VectorGeneric},
};

#[macro_export]
macro_rules! matrix {
    ($inputtype:ty => $intotype:ty, $($($x:expr),*)=>*) => {
        {
            use $crate::vectors::{Vector, VectorType};
            use $crate::matrix::{Matrix};
            let mut temp_matrix = Matrix::new(vec![]);
            $(
                let mut temp_vec = Vector::new(VectorType::Row);
                $(
                    temp_vec.push(<$intotype>::from($x as $inputtype) );
                )*
                temp_matrix.push(temp_vec);
            )*
            temp_matrix
        }
    };
    ($intotype:ty, $($($x:expr),*)=>*) => {
        {
            use $crate::vectors::{Vector, VectorType};
            use $crate::matrix::{Matrix};
            let mut temp_matrix = Matrix::new(vec![]);
            $(
                let mut temp_vec = Vector::new(VectorType::Row);
                $(
                    temp_vec.push($x as $intotype);
                )*
                temp_matrix.push(temp_vec);
            )*
            temp_matrix
        }
    };
    ($($($x:expr),*)=>*) => {
        {
            use $crate::vectors::{Vector, VectorType};
            use $crate::matrix::{Matrix};
            let mut temp_matrix = Matrix::new(vec![]);
            $(
                let mut temp_vec = Vector::new(VectorType::Row);
                $(
                    temp_vec.push($x);
                )*
                temp_matrix.push(temp_vec);
            )*
            temp_matrix
        }
    }
}

#[derive(Debug)]
pub struct Matrix<T: VectorGeneric<T>> {
    contents: Vec<Vector<T>>,
}

impl<T: VectorGeneric<T>> Matrix<T> {
    pub fn new(contents: Vec<Vector<T>>) -> Self {
        Self { contents }
    }
    pub fn len(&self) -> usize {
        self.contents.len()
    }
    pub fn get_(&self, index: usize) -> Option<&Vector<T>> {
        self.contents.get(index)
    }
    pub fn get_mut_(&mut self, index: usize) -> Option<&mut Vector<T>> {
        self.contents.get_mut(index)
    }
    pub fn index_(&self, index: usize) -> &Vector<T> {
        self.contents.get(index).unwrap()
    }
    pub fn index_mut_(&mut self, index: usize) -> &mut Vector<T> {
        self.contents.get_mut(index).unwrap()
    }
    pub fn push(&mut self, value: Vector<T>) {
        if self.contents.len() != 0 {
            if value.len() != self.contents[0].len() {
                panic!(
                    "The vector push length must be the same length as the original vector length."
                );
            }
        }
        self.contents.push(value);
    }
    pub fn is_identity(&self) -> bool {
        if self.contents.get(0).is_some() {
            if self.contents.len() == self.contents[0].len() {
                return true;
            }
        }
        false
    }
}

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

#[test]
fn matrix_test() {
    let x = matrix!(10, 20, 30 => 40, 50, 60 => 70, 80, 90);
    println!("{x}");
    let y = vector![1, 2, 3];
    println!("{}", x * y);
}
