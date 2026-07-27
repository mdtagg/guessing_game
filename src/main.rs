use rand::Rng;
use std::cmp::Ordering;
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
    //Rust allows "shadowing" creating a variable with the same name of different type
    //prevents having to create variables with new names only to change a type
    //The : u32 tells rust which type to convert to in the parse function
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed:{guess}");

    //match is an expression made up of "arms" similar to switch cases
    //here it compares guess to secret_number, can be used on anything comparable
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
