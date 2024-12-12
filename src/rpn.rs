use super::error::{Error, Result};
enum Operation {
    Add,
    Multiply,
    // ??
    // ??
}

impl TryFrom<&str> for Operation {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            // "-" => ??
            // "/" => ??
            _ => Err(Error::UnsupportedOperation(value.into())),
        }
    }
}

pub fn rpn_calculator(input: &str) -> Result<Option<f32>> {
    let output = input
        .split_whitespace()
        .try_fold(Vec::new(), |mut stack, x| {
            if let Ok(number) = x.parse::<f32>() {
                stack.push(number)
            } else {
                let operation = Operation::try_from(x)?;
                let (b, a) = (stack.pop(), stack.pop());
                let result = try_mathematical_operation(a, b, operation)?;
                stack.push(result)
            }
            Ok::<Vec<f32>, Error>(stack)
        })?
        .into_iter()
        .next();
    Ok(output)
}

fn try_mathematical_operation(
    rhs: Option<f32>,
    lhs: Option<f32>,
    operation: Operation,
) -> Result<f32> {
    let result = match (rhs, lhs, operation) {
        (Some(a), Some(b), Operation::Add) => Ok(a + b),
        // (??, ?? ??) = a / b,
        (Some(a), Some(b), Operation::Multiply) => Ok(a * b),
        // (??, ??, ??) = a - b,
        _ => Err(Error::MalforedInput),
    };
    result
}

#[cfg(test)]
mod tests {
    use super::rpn_calculator;
    use crate::error::Result;

    #[test]
    fn simple_add() -> Result<()> {
        let input = "10 4 +";
        let out = rpn_calculator(input)?;
        let number = out.unwrap();

        assert_eq!(number, 14.0);
        Ok(())
    }

    #[test]
    fn test_simple_multiply() -> Result<()> {
        let input = "10 4 *";

        let out = rpn_calculator(input)?;
        let result = out.unwrap();

        assert_eq!(result, 40.0);
        Ok(())
    }

    #[test]
    fn simpe_subtract() -> Result<()> {
        let input = "10 4 -";

        let out = rpn_calculator(input)?;

        let result = out.unwrap();
        assert_eq!(result, 6.0);
        Ok(())
    }

    #[test]
    fn base_divide() -> Result<()> {
        let input = "14 2 /";
        let out = rpn_calculator(input)?;

        let result = out.unwrap();
        assert_eq!(result, 7.0);
        Ok(())
    }

    #[test]
    fn invalid_input() {
        let input = "10 +";
        let out = rpn_calculator(input);
        assert!(out.is_err());
    }

    #[test]
    fn test_empty_string() {
        let input = "";

        let out = rpn_calculator(input).unwrap();
        assert!(out.is_none());
    }
}
