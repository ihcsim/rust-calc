mod calc;

use calc::op;
use enum_iterator::IntoEnumIterator;
use std::io::{self, Write};

fn usage() {
    let mut operators = String::new();
    for operator in op::Operator::into_enum_iter() {
        if operator == op::Operator::None {
            continue;
        }

        operators.push_str(operator.to_string().as_str());
        operators.push_str(",")
    }

    let usage = format!("Supported operators: [{}]", operators);
    println!("{}", usage);
}

fn main() {
    println!("Type '?' to see supported operators, 'q' to quit");
    loop {
        print!("# ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => println!("can't read from stdin: {}", e),
        };

        let trimmed = input.trim();
        let mut result = String::new();
        match trimmed {
            "q" => break,
            "?" => {
                usage();
                continue;
            }
            _ => {
                let mut operation = op::Operation::default();
                for c in trimmed.chars() {
                    if let Ok(v) = c.to_string().parse::<f64>() {
                        operation.operands.push(v);
                        continue;
                    }

                    if let Ok(o) = c.to_string().parse::<op::Operator>() {
                        operation.operator = o;
                    }
                }

                match operation.exec() {
                    Ok(v) => result.push_str(v.to_string().as_str()),
                    Err(e) => result.push_str(e.to_string().as_str()),
                };
            }
        }

        println!("> {}", result);
    }
}
