use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=5);
    /*
    Generates a number between the values in 'gen_range'
    */
   
    loop {
        println!("Please input your guess between 1 and 5."); 
        let mut guess = String::new();
        /*
        Adding 'mut' to the variable deffinition means it can be motified.
        Variables are immutable by default
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /*
        This code here reads what the user inputs, and screams if somehow it breaks
        */
        
        let guess: u32 = guess.trim().parse().expect("You know was a numer is right?");
        /*
        Making 'variablename: u32' makes it so guess is expected to be a
        32 bit integer. 'variablename.trim().parse().expect("")' attempts
        to parse the variable, if it isnt right, log the exception.
        */

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            /*
            break exits the loop
            */
            Ordering::Less => println!("Just too low"),
            Ordering::Greater => println!("Barley too high!"),
        }
        /*
        This runs through and compares one variable to another using Ordering.
        */
    }
}