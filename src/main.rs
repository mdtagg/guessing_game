use std::io;

fn main() {
    println!("Guess the number!");
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
