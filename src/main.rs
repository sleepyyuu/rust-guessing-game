use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop  {
        println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() //can use std::io::stdin if no use std::io
        .read_line(&mut guess)  //mutable reference to guess
                                //.read_line returns a value, io::Result, that is a enum
                                //either "ok" or "err"
        .expect("Failed to read line");  //.expect handles "err" enum

    let guess: u32 = match guess.trim().parse()  {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);  //{} is placeholder, guess variable goes in there

    match guess.cmp(&secret_number)   {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    }

    
}
