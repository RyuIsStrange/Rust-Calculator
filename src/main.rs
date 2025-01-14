use std::io;

fn main() {
    let mth: Vec<(i128, String, i128)> = Vec::new();
    
    println!("Insert your first number");
    let mut val1 = String::new();

    io::stdin().read_line(&mut val1);

    println!("Insert your operator");
    let mut opr = String::new();

    io::stdin().read_line(&mut opr);

    println!("Insert your second number");
    let mut val2 = String::new();
    
    io::stdin().read_line(&mut val2);

    
}