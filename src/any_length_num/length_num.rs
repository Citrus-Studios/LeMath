use std::{cmp::max, ops::Add, result};

pub struct AnyLengthNum<const U: usize> {
    num: [u8; U],
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
        println!("num: {num}");
        num
    }
    pub(crate) fn add_string_u8(x: String, y: u8) -> String {
        let x = x;
        let y = format!("{:b}", y);
        println!("({x}, {y})");
        let max_len = max(x.len(), y.len());
        let mut res = String::new();
        let mut carry = 0;
        for i in 0..max_len {
            let i = max_len - 1 - i;
            let mut r = carry;
            r += {
                if x.chars().nth(i).unwrap() == '1' {
                    1
                } else {
                    0
                }
            };
            r += {
                if y.chars().nth(i).unwrap() == '1' {
                    1
                } else {
                    0
                }
            };
            if r % 2 == 1 {
                res.push('1')
            }
            carry = {
                if r < 2 {
                    0;
                }
                1
            }
        }

        if carry != 0 {
            res.push('1');
        }
        res
    }
}

trait ZFill {
    fn zfill(&mut self, len: usize);
}

impl ZFill for String {
    fn zfill(&mut self, len: usize) {
        for _ in len..=self.len() {
            self.insert(0, '0');
        }
    }
}

impl<const U: usize> Add<u8> for AnyLengthNum<U> {
    type Output = String;

    fn add(self, rhs: u8) -> Self::Output {
        Self::add_string_u8(self.to_string(), rhs)
    }
}

#[test]
fn length_num_math() {
    let x = AnyLengthNum::<2>::new();
    let y = 10;
    let z = x + y;
    println!("{z}");
}
