use std::{
    fmt::{Debug, Display},
    ops::{Add, Index, IndexMut, Mul},
    slice::SliceIndex,
};

use crate::{matrix_items::Matrix, summation::sum_extra, traitbounds::Real};

/// Vector macro for creating a vector of a given type.
///
/// There is 3 forms of the macro:
/// vector![Row, 1f32, 2.0, 3.0]
/// vector![Row, f32, 1, 2, 3]
/// vector![Row, i16 => f32, 1, 2, 3]
///
/// The first form is one where you can specify the type for each element or Rust will infer.
/// The second form is one where you specify the type for the vector and you can specify the type for each element or rust will infer.
/// The third form is one where you specify the type for the vector and the type for each element, this one can panic because you can convert a type to another type that is not a valid conversion. (e.g. u16::MAX to u8)
#[macro_export]
macro_rules! vector {
    ($vectype:ident, $inputtype:ty => $intotype:ty, $($x:expr),*) => {
        {
            use $crate::vectors::{Vector, VectorType};
            let x: Vector<$intotype> = {
                let mut temp_vec = Vector::new(VectorType::$vectype);
                $(
                    temp_vec.push(<$intotype>::from($x as $inputtype));
                )*
                temp_vec
            };
            x
        }
    };
    ($vectype:ident, $intotype:ty, $($x:expr),*) => {
        {
            use $crate::vector::{Vector, VectorType};
            let x: Vector<$intotype> = {
                let mut temp_vec = Vector::new(VectorType::$vectype);
                $(
                    temp_vec.push(<$intotype>::from($x));
                )*
                temp_vec
            };
            x
        }
    };
    ($vectype:ident, $($x:expr),*) => {
        {
            use $crate::vector::{Vector, VectorType};
            let mut temp_vec = Vector::new(VectorType::$vectype);
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    ($($x:expr),*) => {
        {
            use $crate::vector::{Vector, VectorType};
            let mut temp_vec = Vector::new(VectorType::Row);
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// The type of vector for the math vector.
#[derive(Clone, Debug, PartialEq)]
pub enum VectorType {
    Row,
    Column,
}

pub trait VectorGeneric<T> = Clone + Default + Real + Mul<Output = T> + Copy + Debug;

/// A Math Vector.
#[derive(Clone, Debug, PartialEq)]
pub struct Vector<T: VectorGeneric<T>> {
    vec_type: VectorType,
    contents: Vec<T>,
}

impl<T: VectorGeneric<T>> Vector<T> {
    pub fn new(vec_type: VectorType) -> Vector<T> {
        Vector {
            vec_type,
            contents: vec![],
        }
    }
    pub fn push(&mut self, value: T) {
        self.contents.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }
    pub fn len(&self) -> usize {
        self.contents.len()
    }
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.contents.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.contents.iter_mut()
    }
    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.contents.into_iter()
    }
    pub fn as_slice(&self) -> &[T] {
        self.contents.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.contents.as_mut_slice()
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        self.contents.as_mut_slice()
    }
    pub fn as_slice_ref(&self) -> &[T] {
        self.contents.as_slice()
    }
    pub fn as_slice_ref_mut(&mut self) -> &mut [T] {
        self.contents.as_mut_slice()
    }
    #[inline]
    pub fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<&I::Output> {
        self.contents.get(index)
    }
    #[inline]
    pub fn get_mut<I: SliceIndex<[T]>>(&mut self, index: I) -> Option<&mut I::Output> {
        self.contents.get_mut(index)
    }
    #[inline]
    pub fn index_<I: SliceIndex<[T]>>(&self, index: I) -> &I::Output {
        self.contents.get(index).unwrap()
    }
    #[inline]
    pub fn index_mut_<I: SliceIndex<[T]>>(&mut self, index: I) -> &mut I::Output {
        self.contents.get_mut(index).unwrap()
    }
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    // Math Functions
    pub fn dot_product(self, rhs: Self) -> T {
        sum_extra(
            1,
            self.len(),
            |x, y| y.0[x - 1] * y.1[x - 1],
            (self.clone(), rhs.clone()),
        )
    }

    pub fn scalar_mul(mut self, rhs: T) -> Self {
        for x in 0..self.contents.len() {
            self.contents[x] *= rhs;
        }
        self
    }

    pub fn transpose(mut self) -> Self {
        if self.vec_type == VectorType::Row {
            self.vec_type = VectorType::Column
        } else {
            self.vec_type = VectorType::Row
        }
        self
    }
}

impl<T: VectorGeneric<T>, I: SliceIndex<[T]>> Index<I> for Vector<T> {
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.index_(index)
    }
}

impl<T: VectorGeneric<T>, I: SliceIndex<[T]>> IndexMut<I> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.index_mut_(index)
    }
}

impl<T: VectorGeneric<T>> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vec_type == VectorType::Row {
            write!(f, "{}", format!("{:?}", self.contents).replace(',', "")).unwrap();
        } else {
            if self.contents.len() == 1 {
                write!(f, "{}", format!("{:?}", self.contents).replace(',', " ")).unwrap();
            } else if self.contents.len() == 2 {
                write!(f, "⎡ {:?} ⎤\n", self.contents[0]).unwrap();
                write!(f, "⎣ {:?} ⎦", self.contents[1]).unwrap();
            } else {
                write!(f, "⎡ {:?} ⎤\n", self.contents[0]).unwrap();
                for x in 1..self.contents.len() - 1 {
                    write!(f, "⎢ {:?} ⎥\n", self.contents[x]).unwrap();
                }
                write!(f, "⎣ {:?} ⎦", self.contents[self.contents.len() - 1]).unwrap();
            }
        }

        f.write_str("")
    }
}

#[test]
fn vector_test() {
    let x = vector![Row, 1, 2, 3];
    let y = vector![Row, 2, 4, 6];
    assert_eq!(vector![Row, 3, 6, 9], x.clone() + y.clone());
    assert_eq!(vector![Row, 2, 4, 6], x.clone() * 2);
    assert_eq!(28, x.clone() * y.clone());
    println!("{}\n{}", x, y);
}

#[test]
fn vector_macro_test() {
    use crate::vector_items;

    let v = vector![Row, 1f64, 2.0, 3.0];
    let v2 = vector![Row, f64, 1, 2, 3];
    assert_eq!(v, v2);

    let v3 = vector![Row, f32, 1i16, 2i16, 3i16];
    let v4 = vector![Row, i16 => f32, 1, 2, 3];
    assert_eq!(v3, v4);
}
