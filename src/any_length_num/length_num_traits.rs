use std::ops::Add;

use super::length_num::AnyLengthNum;

trait ZFill {
    fn zfill(&mut self, len: usize);
}

impl ZFill for String {
    fn zfill(&mut self, len: usize) {
        for _ in self.len()..len {
            self.insert(0, '0');
        }
    }
}

impl<const U: usize> Add<u8> for AnyLengthNum<U> {
    type Output = AnyLengthNum<U>;

    fn add(self, rhs: u8) -> Self::Output {
        Self::add_string_u8(self.to_string(), rhs).into()
    }
}

impl<const U: usize> From<String> for AnyLengthNum<U> {
    fn from(from: String) -> Self {
        let mut res = AnyLengthNum::<U>::new();
        let mut from = from;
        from.zfill(U * 8);
        println!("From: {from}");
        for x in 0..U {
            res.num[x] = u8::from_str_radix(&from.as_str()[x * 8..x * 8 + 8], 2).unwrap();
        }

        res
    }
}
