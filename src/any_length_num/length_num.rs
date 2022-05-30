use std::{cmp::max, iter, ops::Add, result, str::Chars};

pub struct AnyLengthNum<const U: usize> {
    pub num: [u8; U],
}

impl<const U: usize> AnyLengthNum<U> {
    pub fn new() -> Self {
        Self { num: [0; U] }
    }
    pub fn to_string(&self) -> String {
        let mut num = format!("{:b}", self.num[0]);
        for x in 1..self.num.len() {
            num = Self::add_string_u8(num, self.num[x]);
        }
        num
    }
    pub(crate) fn add_string_u8(a: String, b: u8) -> String {
        // Credit to https://leetcode.com/problems/add-binary/discuss/1324370/Rust-solution
        let b = format!("{:b}", b);
        let mut carry = 0;
        let mut cur_sum = 0;
        let mut char_vec = a
            .as_bytes()
            .iter()
            .rev()
            .chain(iter::repeat(&b'0'))
            .zip(b.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
            .take(a.len().max(b.len()))
            .map(|(ac, bc)| {
                cur_sum = (*ac - b'0') + (*bc - b'0') + carry;
                carry = cur_sum / 2;
                match cur_sum % 2 {
                    1 => '1',
                    _ => '0',
                }
            })
            .collect::<Vec<_>>();

        if carry == 1 {
            char_vec.push('1');
        }

        char_vec.iter().rev().collect()
    }
}

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

#[test]
fn length_num_math() {
    let x = AnyLengthNum::<128>::new();
    let y = 255;
    let z = x + y + 255;
    println!("{:?}", z.num);
}
