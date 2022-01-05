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

                for op in op::Operator::into_enum_iter() {
                    match trimmed.find(|c: char| c.to_string() == op.to_string()) {
                        Some(i) => {
                            let operand1 = trimmed[0..i].trim().parse().unwrap();
                            let operand2 = trimmed[i + 1..].trim().parse().unwrap();
                            operation.operands.push(operand1);
                            operation.operands.push(operand2);
                            operation.operator = op;
                            break;
                        }
                        None => println!("got nothing"),
                    };
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
