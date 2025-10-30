/// Error types.
/// 
/// - [Error]
///     - [`Error::InvalidOperation`]: The operation in the equation is either not implemented or doesn't exist.
///     - [`Error::DivideByZero`]: You can't divide by zero.
#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    DivideByZero
}

/// Returns the calculated value of two numbers
/// 
/// # Parameters
/// - v1 [`f64`]: First number in the operation
/// - op [`&str`]: The operator
/// - v2 [`f64`]: Second number in the operation
/// 
/// # Result
/// - [`Result<f64, Error>`]
///     - [`f64`]: The result.
///     - [`Error`]
///         - The primary error(s) that will be returned are either [`Error::DivideByZero`] or [`Error::InvalidOperation`]
///         - There may be other errors but no clue if there are actually any others.
pub fn calc(v1: f64, op: &str, v2: f64) -> Result<f64, Error> {
    Ok(match op {
        "+" => v1 + v2,
        "-" => v1 - v2,
        "*" => v1 * v2,
        "/" => {
            if v2 == 0.0 {
                return Err(Error::DivideByZero)
            } else {
                v1 / v2
            }
        },
        "^" => {v1.powf((v2).try_into().unwrap())},
        "sqrt" => {v1.sqrt()},
        _ => return Err(Error::InvalidOperation)
    })
}