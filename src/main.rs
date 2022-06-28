use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() //can use std::io::stdin if no use std::io
        .read_line(&mut guess)  //mutable reference to guess
                                //.read_line returns a value, io::Result, that is a enum
                                //either "ok" or "err"
        .expect("Failed to read line");  //.expect handles "err" enum

    println!("You guessed: {}", guess);  //{} is placeholder, guess variable goes in there
}
