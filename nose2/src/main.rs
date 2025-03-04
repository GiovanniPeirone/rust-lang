use std::io;

fn main() {
    print!("cual es el numero:");
    let n1: String = input("cual es el numero:".to_string());
    println!("{}", n1);
}

fn input(ask:String) -> String{
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    return b1.to_string();
}
