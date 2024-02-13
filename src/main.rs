use rustymath::evaluate_expression;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap().trim().to_string();
        match evaluate_expression(&line) {
            Ok(result) => println!(" = {}", result),
            Err(_) => println!(" Syntax error"),
        }
    }
}
