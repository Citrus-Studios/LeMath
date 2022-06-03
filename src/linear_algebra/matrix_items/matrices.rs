use crate::linear_algebra::vectors::{Vector, VectorGeneric};

#[macro_export]
macro_rules! matrix {
    ($inputtype:ty => $intotype:ty, $($($x:expr),*)=>*) => {
        {
            use $crate::linear_algebra::vectors::{Vector, VectorType};
            use $crate::linear_algebra::matrices::{Matrix};
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
            use $crate::linear_algebra::vectors::{Vector, VectorType};
            use $crate::linear_algebra::matrices::{Matrix};
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
            use $crate::linear_algebra::vectors::{Vector, VectorType};
            use $crate::linear_algebra::matrices::{Matrix};
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
    pub(crate) contents: Vec<Vector<T>>,
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

#[test]
fn matrix_test() {
    use crate::vector;
    let x = matrix!(10, 20, 30 => 40, 50, 60 => 70, 80, 90);
    println!("{x}");
    let y = vector![1, 2, 3];
    println!("{}", x * y);
}
