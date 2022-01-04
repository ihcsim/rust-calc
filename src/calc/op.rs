use std::fmt;
use std::str::FromStr;

use crate::calc::error as err;

#[derive(Default)]
pub struct Operation {
    pub operands: Vec<f64>,
    pub operator: Operator,
}

impl Operation {
    pub fn exec(&self) -> Result<f64, err::FailedOperationError> {
        match self.operator {
            Operator::Add => self.operator.add(self.operands[0], self.operands[1]),
            Operator::Subtract => self.operator.subtract(self.operands[0], self.operands[1]),
            Operator::Multiply => self.operator.multiply(self.operands[0], self.operands[1]),
            Operator::Divide => self.operator.divide(self.operands[0], self.operands[1]),
            Operator::None => Err(err::FailedOperationError {
                reason: String::from("No supported operator provided"),
            }),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
}

impl Operator {
    fn add(&self, operand1: f64, operand2: f64) -> Result<f64, err::FailedOperationError> {
        if *self != Operator::Add {
            let reason = format!("wrong operator {}", self);
            return Err(err::FailedOperationError { reason });
        }
        Ok(operand1 + operand2)
    }

    fn subtract(&self, operand1: f64, operand2: f64) -> Result<f64, err::FailedOperationError> {
        if *self != Operator::Subtract {
            let reason = format!("wrong operator {}", self);
            return Err(err::FailedOperationError { reason });
        }
        Ok(operand1 - operand2)
    }

    fn multiply(&self, operand1: f64, operand2: f64) -> Result<f64, err::FailedOperationError> {
        if *self != Operator::Multiply {
            let reason = format!("wrong operator {}", self);
            return Err(err::FailedOperationError { reason });
        }
        Ok(operand1 * operand2)
    }

    fn divide(&self, operand1: f64, operand2: f64) -> Result<f64, err::FailedOperationError> {
        if *self != Operator::Divide {
            let reason = format!("wrong operator {}", self);
            return Err(err::FailedOperationError { reason });
        }
        if operand2 == 0.0_f64 {
            let reason = String::from("zero divisor");
            return Err(err::FailedOperationError { reason });
        }
        Ok(operand1 / operand2)
    }
}

impl Default for Operator {
    fn default() -> Self {
        Operator::None
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::None => "<none>",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Operator {
    type Err = err::UnsupportedOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(err::new(String::from(s))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn exec_add() {
            let operation = Operation {
                operands: vec![10.0, 8.0],
                operator: Operator::Add,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(18.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-10.0, 8.0],
                operator: Operator::Add,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-2.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-10.0, -8.0],
                operator: Operator::Add,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-18.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-10.0, -8.0],
                operator: Operator::Add,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-18.0, actual);
            } else {
                panic!("unexpected error")
            }
        }

        #[test]
        fn exec_subtract() {
            let operation = Operation {
                operands: vec![12.0, 5.0],
                operator: Operator::Subtract,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(7.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-12.0, 5.0],
                operator: Operator::Subtract,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-17.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![12.0, -5.0],
                operator: Operator::Subtract,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(17.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-12.0, -5.0],
                operator: Operator::Subtract,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-7.0, actual);
            } else {
                panic!("unexpected error")
            }
        }

        #[test]
        fn exec_multiply() {
            let operation = Operation {
                operands: vec![9.0, 3.0],
                operator: Operator::Multiply,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(27.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-9.0, 3.0],
                operator: Operator::Multiply,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-27.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-9.0, -3.0],
                operator: Operator::Multiply,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(27.0, actual);
            } else {
                panic!("unexpected error")
            }
        }

        #[test]
        fn exec_divide() {
            let operation = Operation {
                operands: vec![9.0, 3.0],
                operator: Operator::Divide,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(3.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-9.0, 3.0],
                operator: Operator::Divide,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(-3.0, actual);
            } else {
                panic!("unexpected error")
            }

            let operation = Operation {
                operands: vec![-9.0, -3.0],
                operator: Operator::Divide,
            };

            if let Ok(actual) = operation.exec() {
                assert_eq!(3.0, actual);
            } else {
                panic!("unexpected error")
            }
        }

        #[test]
        #[should_panic(expected = "zero divisor")]
        fn exec_divide_by_zero() {
            let operation = Operation {
                operands: vec![13.0, 0.0],
                operator: Operator::Divide,
            };

            operation.exec().unwrap();
        }
    }

    mod operator {
        use super::*;

        #[test]
        fn default_is_none() {
            let operator = Operator::default();
            assert_eq!(operator, Operator::None);
        }

        #[test]
        fn from_str() {
            if let Ok(operator) = "+".parse::<Operator>() {
                assert_eq!(operator, Operator::Add);
            } else {
                panic!("failed to parse operator");
            }

            if let Ok(operator) = "-".parse::<Operator>() {
                assert_eq!(operator, Operator::Subtract);
            } else {
                panic!("failed to parse operator");
            }

            if let Ok(operator) = "*".parse::<Operator>() {
                assert_eq!(operator, Operator::Multiply);
            } else {
                panic!("failed to parse operator");
            }

            if let Ok(operator) = "/".parse::<Operator>() {
                assert_eq!(operator, Operator::Divide);
            } else {
                panic!("failed to parse operator");
            }
        }

        #[test]
        fn add() {
            let operator = Operator::Add;
            match operator.add(1.0, 1.0) {
                Ok(actual) => assert_eq!(actual, 2.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.add(-1.0, 1.0) {
                Ok(actual) => assert_eq!(actual, 0.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.add(-1.0, -1.0) {
                Ok(actual) => assert_eq!(actual, -2.0),
                Err(e) => panic!("unexpected error: {}", e),
            };
        }

        #[test]
        fn subtract() {
            let operator = Operator::Subtract;
            match operator.subtract(5.0, 2.0) {
                Ok(actual) => assert_eq!(actual, 3.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.subtract(-5.0, 2.0) {
                Ok(actual) => assert_eq!(actual, -7.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.subtract(-5.0, -2.0) {
                Ok(actual) => assert_eq!(actual, -3.0),
                Err(e) => panic!("unexpected error: {}", e),
            };
        }

        #[test]
        fn multiply() {
            let operator = Operator::Multiply;
            match operator.multiply(4.0, 7.0) {
                Ok(actual) => assert_eq!(actual, 28.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.multiply(-4.0, 7.0) {
                Ok(actual) => assert_eq!(actual, -28.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.multiply(-4.0, -7.0) {
                Ok(actual) => assert_eq!(actual, 28.0),
                Err(e) => panic!("unexpected error: {}", e),
            };
        }

        #[test]
        fn divide() {
            let operator = Operator::Divide;
            match operator.divide(10.0, 2.0) {
                Ok(actual) => assert_eq!(actual, 5.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.divide(-10.0, 2.0) {
                Ok(actual) => assert_eq!(actual, -5.0),
                Err(e) => panic!("unexpected error: {}", e),
            };

            match operator.divide(-10.0, -2.0) {
                Ok(actual) => assert_eq!(actual, 5.0),
                Err(e) => panic!("unexpected error: {}", e),
            };
        }

        #[test]
        #[should_panic(expected = "zero divisor")]
        fn divide_by_zero() {
            let operator = Operator::Divide;
            operator.divide(10.0, 0.0).unwrap();
        }

        #[test]
        #[should_panic(expected = "wrong operator")]
        fn using_wrong_operators() {
            let operators = vec![Operator::Subtract, Operator::Multiply, Operator::Divide];
            for operator in operators.iter() {
                operator.add(1.0, 1.0).unwrap();
            }

            let operators = vec![Operator::Add, Operator::Multiply, Operator::Divide];
            for operator in operators.iter() {
                operator.subtract(1.0, 1.0).unwrap();
            }

            let operators = vec![Operator::Add, Operator::Subtract, Operator::Divide];
            for operator in operators.iter() {
                operator.multiply(1.0, 1.0).unwrap();
            }

            let operators = vec![Operator::Add, Operator::Subtract, Operator::Multiply];
            for operator in operators.iter() {
                operator.divide(1.0, 1.0).unwrap();
            }
        }
    }
}
