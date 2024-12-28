use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use super::error::{Error, Result};
enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl TryFrom<&str> for Operation {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            "-" => Ok(Self::Subtract),
            "/" => Ok(Self::Divide),
            _ => Err(Error::UnsupportedOperation(value.into())),
        }
    }
}

pub fn rpn_calculator<T>(input: &str) -> Result<Option<T>>
where
    T: FromStr + Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    let output = input
        .split_whitespace()
        .try_fold(Vec::new(), |mut stack, x| {
            if let Ok(number) = x.parse::<T>() {
                stack.push(number)
            } else {
                let operation = Operation::try_from(x)?;
                let (b, a) = (stack.pop(), stack.pop());
                let result = try_mathematical_operation(a, b, operation)?;
                stack.push(result)
            }
            Ok::<Vec<T>, Error>(stack)
        })?
        .into_iter()
        .next();
    Ok(output)
}

fn try_mathematical_operation<T>(rhs: Option<T>, lhs: Option<T>, operation: Operation) -> Result<T>
where
    T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    let result = match (rhs, lhs, operation) {
        (Some(a), Some(b), Operation::Add) => Ok(a + b),
        (Some(a), Some(b), Operation::Subtract) => Ok(a - b),
        (Some(a), Some(b), Operation::Multiply) => Ok(a * b),
        (Some(a), Some(b), Operation::Divide) => Ok(a / b),
        _ => Err(Error::MalforedInput),
    };
    result
}

#[cfg(test)]
mod tests {

    use super::rpn_calculator;
    use crate::error::Result;
    use std::fs;

    #[test]
    fn simple_add() -> Result<()> {
        let input = "10 4 +";
        let out = rpn_calculator::<f64>(input)?;
        let number = out.unwrap();

        assert_eq!(number, 14.0);
        Ok(())
    }

    #[test]
    fn test_simple_multiply() -> Result<()> {
        let input = "10 4 *";

        let out = rpn_calculator::<f64>(input)?;
        let result = out.unwrap();

        assert_eq!(result, 40.0);
        Ok(())
    }

    #[test]
    fn simpe_subtract() -> Result<()> {
        let input = "10 4 -";

        let out = rpn_calculator::<f64>(input)?;

        let result = out.unwrap();
        assert_eq!(result, 6.0);
        Ok(())
    }

    #[test]
    fn base_divide() -> Result<()> {
        let input = "14 2 /";
        let out = rpn_calculator::<f64>(input)?;

        let result = out.unwrap();
        assert_eq!(result, 7.0);
        Ok(())
    }

    #[test]
    fn invalid_input() {
        let input = "10 +";
        let out = rpn_calculator::<f64>(input);
        assert!(out.is_err());
    }

    #[test]
    fn test_empty_string() {
        let input = "";

        let out = rpn_calculator::<f64>(input).unwrap();
        assert!(out.is_none());
    }

    #[test]
    fn test_ode5_1() {
        let input = "9 9 - 9 + 9 * 9 /";
        let out = rpn_calculator::<f64>(input).unwrap();

        let result = out.unwrap();

        assert_eq!(result, 9.);
    }

    #[test]
    fn test_ode5_2() {
        let input = "3 4 + 5 6 + *";
        let out = rpn_calculator::<f64>(input).unwrap();

        let result = out.unwrap();

        assert_eq!(result, 77.);
    }

    #[test]
    fn test_ode5_3() {
        let input = "2 3 + 5 3 - 4 2 / * -";
        let out = rpn_calculator::<f64>(input).unwrap();

        let result = out.unwrap();

        assert_eq!(result, 1.);
    }

    #[test]
    fn test_ode5_large_file() -> Result<()> {
        let file_path = std::path::Path::new("./test/rpn_expression-5869168714501555882.txt");

        let input = fs::read_to_string(file_path)?;

        let out = rpn_calculator::<f64>(&input).unwrap();

        let result = out.unwrap();

        dbg!(result);

        assert!(false);

        Ok(())
    }
}
