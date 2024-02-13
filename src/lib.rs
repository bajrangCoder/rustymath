pub mod unit_converter;

use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Calculator;

pub fn create_pratt_parser() -> PrattParser<Rule> {
    use Assoc::*;
    use Rule::*;

    PrattParser::new()
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
        .op(Op::infix(power, Right))
}

pub fn eval(expression: pest::iterators::Pairs<Rule>, pratt_parser: &PrattParser<Rule>) -> f64 {
    pratt_parser
        .map_primary(|primary| match primary.as_rule() {
            Rule::num => primary.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(primary.into_inner(), pratt_parser),
            Rule::bracketed_expr => {
                let mut inner_pairs = primary.into_inner();
                let inner_expr = inner_pairs.next().unwrap(); // Get the expression inside the brackets
                eval(inner_expr.into_inner(), pratt_parser)
            }
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            _ => unreachable!(),
        })
        .parse(expression)
}

#[wasm_bindgen]
pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let pratt_parser = create_pratt_parser();
    let parse_result = Calculator::parse(Rule::calculation, expression);
    match parse_result {
        Ok(mut calc) => {
            let result = eval(calc.next().unwrap().into_inner(), &pratt_parser);
            Ok(result)
        }
        Err(_) => Err("Syntax error".to_string()),
    }
}