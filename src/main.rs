use std::time;
use std::{io, num::ParseIntError, process, thread::sleep};

/// Error types.
/// 
/// - [Error]
///     - [`Error::InvalidOperation`]: The operation in the equation is either not implemented or doesn't exist.
///     - [`Error::DivideByZero`]: You can't divide by zero.
#[derive(Debug)]
enum Error {
    InvalidOperation,
    DivideByZero
}

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
/// - ([`Result`]<[`i64`], [`ParseIntError`]>, [`Result`]<[`i64`], [`ParseIntError`]>)
///     - [`i64`]: The input after being parsed.
///     - [`ParseIntError`]: If this is returned there was an error in the preparse step.
fn intparse(v1: &str, v2: &str) -> (Result<i64, ParseIntError>, Result<i64, ParseIntError>) {
    if preparse(&v1, &v2) {
        (v1.trim().parse::<i64>(), v2.trim().parse::<i64>())
    } else {
        let err = Err("Pre-parse validation failed".parse::<i64>().unwrap_err());

        (err.clone(), err)
    }
}

/// Returns the calculated value of two numbers
/// 
/// # Parameters
/// - v1 [`i64`]: First number in the operation
/// - op [`&str`]: The operator
/// - v2 [`i64`]: Second number in the operation
/// 
/// # Result
/// - [`Result<(i64, i64), Error>`]
///     - [`(i64, i64)`]
///         - res.0: The primary calculated output
///         - res.1: The remainder of any output.
///             - Default: 0 no remainder.
///             - Anything other than 0 is a valid remainder.
///     - [`Error`]
///         - The primary error(s) that will be returned are either [`Error::DivideByZero`] or [`Error::InvalidOperation`]
///         - There may be other errors but no clue if there are actually any others.
fn calc(v1: i64, op: &str, v2: i64) -> Result<(i64, i64), Error> {
    Ok(match op {
        "+" => (v1 + v2, 0),
        "-" => (v1 - v2, 0),
        "*" => (v1 * v2, 0),
        "/" => {
            if v2 == 0 {
                return Err(Error::DivideByZero)
            } else {
                if (v1 % v2) == 0 {
                    (v1 / v2, 0)
                } else {
                    (v1 / v2, v1 % v2)
                }
            }
        },
        "^" => {(v1.pow((v2).try_into().unwrap()), 0)},
        "sqrt" => {(v1.isqrt(),0)},
        _ => return Err(Error::InvalidOperation)
    })
}

fn main() {    
    loop {
        println!("============================================================");
        println!("Enter your input in the format: <number> <operator> <number>");
        println!("============================================================");
        println!("    Or you may input 'exit' to stop using the calculator    ");
        println!("============================================================");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() == 1 {
            if parts[0] == "exit" {
                print!("Bye bye\n");
                process::exit(1); // Grant the user the sweet mercy of death
            }
        }

        if parts.len() != 3 && parts[1] != "sqrt" {
            eprintln!("Invalid input format. Use: <number> <operator> <number>");
            process::exit(1); // Punish the user
        }

        let val1 = parts[0];
        let opr = parts[1];

        // Sets default value to 0 so it passes parsing
        let mut val2= "0";
        // Checks if there is actually a 3rd value
        // Usually not if operator is sqrt
        if parts.len() > 2 { val2 = parts[2]; }

        let ints = intparse(val1, val2);
        
        if let (Ok(int1), Ok(int2)) = ints {
            match calc(int1, opr.trim(), int2) {
                Ok(res) => {
                    if res.1 != 0 {
                        println!("Result: {} R{}", res.0, res.1);
                    } else {
                        println!("Result: {}\n", res.0);
                        sleep(time::Duration::from_secs(1));
                    }
                }
                Err(e) => {
                    eprintln!("Error in calculation {:?}\n", e);
                    sleep(time::Duration::from_secs(1))
                }
            }
        } else {
            if let Err(e1) = ints.0 {
                eprintln!("Error parsing first value: {}\n", e1);
                sleep(time::Duration::from_secs(1));
            }
            if let Err(e2) = ints.1 {
                eprintln!("Error parsing second value: {}\n", e2);
                sleep(time::Duration::from_secs(1));
            }
        }
    }
}