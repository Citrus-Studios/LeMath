use std::{ops::{Index, IndexMut}, slice::SliceIndex};

use crate::traitbounds::Real;

/// Vector macro for creating a vector of a given type.
macro_rules! vector {
    ($type:ident, $($x:expr),*) => {
        {
            let mut temp_vec = Vector::new(VectorType::$type);
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test]
fn vector_test() {
    let _ = vector![Row, 1, 2, 3];
}

/// The type of vector for the math vector.
#[derive(Clone, Debug, PartialEq)]
pub enum VectorType {
    Row,
    Column
}


/// A Math Vector.
#[derive(Clone, Debug, PartialEq)]
pub struct Vector<T: Real> {
    vec_type: VectorType,
    contents: Vec<T>
}

impl<T: Real> Vector<T> {
    pub fn new(vec_type: VectorType) -> Vector<T> {
        Vector {
            vec_type,
            contents: vec![]
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
    pub fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<&I::Output> {
        self.contents.get(index)
    }
    pub fn get_mut<I: SliceIndex<[T]>>(&mut self, index: I) -> Option<&mut I::Output> {
        self.contents.get_mut(index)
    }
    pub fn index<I: SliceIndex<[T]>>(&self, index: I) -> &I::Output {
        self.contents.get(index).unwrap()
    }
    pub fn index_mut<I: SliceIndex<[T]>>(&mut self, index: I) -> &mut I::Output {
        self.contents.get_mut(index).unwrap()
    }
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}

impl<T: Real, I: SliceIndex<[T]>> Index<I> for Vector<T> {
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.index(index)
    }
}

impl<T: Real, I: SliceIndex<[T]>> IndexMut<I> for Vector<T> {
    #[inline]    
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.index_mut(index)
    }
}