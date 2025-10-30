use std::{
    time::{self, Duration},
    thread::sleep, 
    io,
    process
};

use eframe::egui;

// Math functions
mod math;

#[derive(Default)]
struct Content {
    display: String,
    prev_value: f64,
    cur_value: f64
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        ctx.request_repaint_after(Duration::from_secs(1));   
    }
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
                print!("\nBye bye\n");
                process::exit(0); // Exits the operation successfully
            }
        }

        if parts.len() != 3 && parts[1] != "sqrt" {
            eprintln!("Invalid input format. Use: <number> <operator> <number>");
        }

        let val1 = parts[0];
        let opr = parts[1];

        // Sets default value to 0 so it passes parsing
        let mut val2= "0";
        // Checks if there is actually a 3rd value
        // Usually not if operator is sqrt
        if parts.len() > 2 { val2 = parts[2]; }

        let ints = math::parse::intparse(val1, val2);
        
        if let (Ok(int1), Ok(int2)) = ints {
            match math::main::calc(int1, opr.trim(), int2) {
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
        }
    }
}