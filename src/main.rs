use core::time;
use std::{io, num::ParseIntError, process, thread::sleep};

#[derive(Debug)]
enum Error {
    InvalidOperation,
    DivideByZero
}

fn preparse(v1: &str, v2: &str) -> bool {
    let contains_number = |s: &str| s.chars().any(|c| c.is_numeric());

    contains_number(v1) && contains_number(v2)
}

fn intmaxxing(v1: &str, v2: &str) -> (Result<i64, ParseIntError>, Result<i64, ParseIntError>) {
    if preparse(&v1, &v2) {
        (v1.trim().parse::<i64>(), v2.trim().parse::<i64>())
    } else {
        let err = Err("Pre-parse validation failed".parse::<i64>().unwrap_err());

        (err.clone(), err)
    }
}

fn calc(v1: i64, op: &str, v2: i64) -> Result<i64, Error> {
    Ok(match op {
        "+" => v1 + v2,
        "-" => v1 - v2,
        "*" => v1 * v2,
        "/" => {
            if v2 == 0 {
                return Err(Error::DivideByZero)
            } else {
                v1 / v2
            }
        }
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

        if parts.len() != 3 {
            eprintln!("Invalid input format. Use: <number> <operator> <number>");
            process::exit(1); // Punish the user
        }

        let val1 = parts[0];
        let opr = parts[1];
        let val2 = parts[2];

        let ints = intmaxxing(val1, val2);
        
        if let (Ok(int1), Ok(int2)) = ints {
            match calc(int1, opr.trim(), int2) {
                Ok(res) => {
                    println!("Result: {}\n", res);
                    sleep(time::Duration::from_secs(1));
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
            /*
            Ok maybe dont punish them for now

            process::exit(1); // Punish the user
            */
        }
    }
}