use std::io; // input/output standard lib
use rand::Rng; // crate => trait
use std::cmp::Ordering;

fn main() {
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");

    loop {
        println!("Please input your guess");
        let mut guess = String::new(); // mutable variable, immutable by default
        io::stdin().read_line(&mut guess)
        .expect("cannot read the line");
        //let guess:u32 = guess.trim().parse().expect("Please type a number"); 
        let guess: u32 = match guess.trim().parse() { // shadow, convert type
            Ok(num) => num,
            Err(_) => continue
        }; // better handling of invalid input
        println!("the number you guess is : {}", guess);
    
        match guess.cmp(&secret_number){ // match expression
            Ordering::Less => println!("Too small"), // arm
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    };
}

// cargo new xxx_xxx, cargo build, cargo run, cargo check
