use std::io;

fn main() {
    println!("Guess the number!")

    println!("Please input your guess.");

    let mut guess = String::new();

    to::stdin().read_line(&mut guess).expect("Failed to read_line")

    println!("You quessed: {}", guess);
}
