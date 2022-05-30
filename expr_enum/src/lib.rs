#[derive(Debug, Clone, Copy)]
pub enum Equation<T: EquationTrait, U: EquationTrait> {
    Add(T, U),
    Sub(T, U),
    Mul(T, U),
    Div(T, U),
    AddVariable(T),
    SubVariable(T),
    MulVariable(T),
    DivVariable(T),
}

pub trait EquationTrait {}
impl<T: EquationTrait, U: EquationTrait> EquationTrait for Equation<T, U> {}
macro_rules! impl_equation_trait {
    ($($x:ty)*) => {
        $(
            impl EquationTrait for $x {}
        )*
    };
}
impl_equation_trait!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl<T: EquationTrait, U: EquationTrait> Equation<T, U> {
    pub fn new(equation: &str) -> Self {
        let mut equation = equation.to_string();
        let mut index_offset = 0;
        let mut last_char = '|';
        for (i, c) in equation.clone().chars().enumerate() {
            if c == 'x' && last_char.is_numeric() {
                equation.insert(i + index_offset, '*');
                index_offset += 1;
            }
            last_char = c;
        }
        println!("First Run Through: {equation}");

        for (i, c) in equation.clone().chars().enumerate() {
            if c == '*' {}
        }

        todo!()
    }
}

#[test]
fn equation_enum_test() {
    let equation: Equation<i32, i32> = Equation::new("10x + 3");
}
