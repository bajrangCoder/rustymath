use rustymath::{create_pratt_parser, eval, Calculator, Rule};
use pest::Parser;

#[cfg(test)]
// Helper function to evaluate expressions and compare results
fn eval_and_assert(expr: &str, expected_result: f64) {
    let parse_result = Calculator::parse(Rule::calculation, expr);
    assert!(parse_result.is_ok(), "Parsing failed: {:?}", parse_result);

    let pairs = parse_result.unwrap().next().unwrap().into_inner();
    let result = eval(pairs, &create_pratt_parser());

    assert_eq!(result, expected_result, "Expression: {}", expr);
}

#[test]
fn test_simple_addition() {
    eval_and_assert("2 + 3", 5.0);
}

#[test]
fn test_simple_subtraction() {
    eval_and_assert("5 - 3", 2.0);
}

#[test]
fn test_simple_multiplication() {
    eval_and_assert("2 * 3", 6.0);
}

#[test]
fn test_simple_division() {
    eval_and_assert("6 / 2", 3.0);
}

#[test]
fn test_complex_expression() {
    eval_and_assert("2 + 3 * 4", 14.0);
}

#[test]
fn test_expression_with_brackets() {
    eval_and_assert("(2 + 3) * 4", 20.0);
}

#[test]
fn test_exponential_expression() {
    eval_and_assert("2 ^ 3", 8.0);
}

#[test]
fn test_expression_with_multiple_brackets() {
    eval_and_assert("((2 + 3) * (4 - 1)) / 2", 7.5);
}
