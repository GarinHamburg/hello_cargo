use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); 
    
    /*

    Adding 'mut' to the variable deffinition means it can be motified.
    Variables are immutable by default

    so 
    let apples = 5;
    can't be edited later

    but
    let mut apples = 5;
    can be

    */
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line")
    
    println!("You guessed: {guess}");
}