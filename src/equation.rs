pub fn interpret_equation(mut equation: String) -> String {
    let mut last_char = equation.chars().nth(0).unwrap();
    for (i, c) in equation.clone().chars().into_iter().enumerate() {
        print!("{c} {i} {}", c.is_alphabetic() && last_char.is_numeric() && (c != '+' || c != '*' || c != '/' || c != '-'));
        // if a variable is next to a number add a * inbetween the number and the variable
        if c.is_alphabetic() && last_char.is_numeric() && (c != '+' || c != '*' || c != '/' || c != '-') {
            equation.insert(i, '*');
            print!(" *");
        }
        println!("");
        last_char = c;
    }
    return equation;
}

macro_rules! equation {
    ($($x:tt)*) => {
        interpret_equation(String::new() $( + stringify!($x))* + " ")
    };
}

#[test]
fn equation_macro_test() {
    let x = equation!(10x + 3y + 3z - 2a);
    println!("{x}");
}