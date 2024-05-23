use rand::prelude::*;
use std::io;

fn main() {
    let rand_number = thread_rng().gen_range(1..101);
    let mut count = 0;
    println!(
        "You must guess a number between 1 and 100! 
        \nIf the number is wrong, you will have to guess until you get the right answer...
        \nYou only have 10 guesses...
        \nWhat is your guess: "
    );

    loop {
        let mut buffer = String::new();

        let guess_number = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value, //success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again:");
                    continue; //calling this will terminate the loop here, and will not execute the if else statements.  It will restart the loop
                }
            },
            Err(_) => {
                println!("\nFailed to read input. Guess again:");
                continue;
            }
        };
        count += 1;
        if count == 10 {
            println!("You ran out of guesses!  Try again next time.  Thanks for playing!");
            println!("The number was {}", rand_number);
            break;
        } else if rand_number < guess_number {
            println!("\nYour number was too high, try again!");
            println!("Current count of guesses: {}", count);
        } else if rand_number > guess_number {
            println!("\nYour number was too low, try again!");
            println!("Current count of guesses: {}", count);
        } else {
            println!(
                "\nCongrats!  You found the number, and it is: {}",
                rand_number
            );
            break;
        }
    }
}
