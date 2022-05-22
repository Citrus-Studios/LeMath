pub use function_from_equation::equation;

#[test]
fn equation_macro_test() {
    use function_from_equation::equation;

    let x = equation!(10 * x + 3 * y + 3 * z - 2 * a);
    assert_eq!(170, x(10, 20, 30, 40));
}
