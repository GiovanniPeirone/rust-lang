use std::io;

fn main() {
}

fn input() -> String{
    let mut line = String::new();
    println!("Enter your name:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello, {}", line);
    println!("No of bytes read: {}", b1);
}
