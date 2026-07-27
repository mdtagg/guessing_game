use rand::Rng;
use std::io;
// for documentation on crates Cargo doc --open

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        //&mut is used to first define the guess string as being mutable
        //its used again to say "let .readline change guess now but only now, main still retains
        //ownership"
        .read_line(&mut guess)
        //.expect -> value, Result = enum, a type that can be in one of
        //multiple different states
        .expect("Failed to read line");
    println!("You guessed:{guess}");
}
