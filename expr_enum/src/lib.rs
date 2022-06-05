#[derive(Debug, Clone)]
pub enum Equation<T: EquationTrait, U: EquationTrait> {
    Add(T, U),
    Sub(T, U),
    Mul(T, U),
    Div(T, U),
    AddVariable(T, String),
    SubVariable(T, String),
    MulVariable(T, String),
    DivVariable(T, String),
    Parenthesis(T),
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

        todo!()
    }
}

#[test]
fn equation_enum_test() {
    // 10x + 3 -> 10*x + 3 -> MulVariable(10, "x") + 3 -> Add(MulVariable(10, "x"), 3)
    let equation = Equation::<i32, i32>::new("10x + 3");
}
