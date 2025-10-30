use::std::num::ParseFloatError;

/// Check if the inputed values are actually numbers.
/// 
/// # Parameters
/// - v1 [`&str`]: First input.
/// - v2 [`&str`]: Second input.
/// 
/// # Result
/// - [`bool`]
///     - [`True`]: Both are numbers.
///     - [`False`]: Either one or both values are NOT numbers.
fn preparse(v1: &str, v2: &str) -> bool {
    let contains_number = |s: &str| s.chars().any(|c| c.is_numeric());

    contains_number(v1) && contains_number(v2)
}

/// Parses the inputs and turns them into usable numbers.
/// 
/// # Parameters
/// - v1 [`&str`]: First input.
/// - v2 [`&str`]: Second input.
/// 
/// # Result
/// - ([`Result<f64, ParseFloatError>`], [`Result<f64, ParseFloatError>`])
///     - [`f64`]: The input after being parsed.
///     - [`ParseFloatError`]: If this is returned there was an error in the preparse step.
pub fn intparse(v1: &str, v2: &str) -> (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>) {
    if preparse(&v1, &v2) {

        (v1.trim().parse::<f64>(), v2.trim().parse::<f64>())
    } else {
        let err = Err("Pre-parse validation failed".parse::<f64>().unwrap_err());

        (err.clone(), err)
    }
}