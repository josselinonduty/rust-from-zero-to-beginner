/**
 * Program: less or more
 * Description:
 * A number between 0 and 100 is chosen at random.
 * The player will try to guess the number by providing its guesses.
 */

use rand::{Rng, thread_rng};
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("===== Initialisation =====");

    // Create a number between 1 and 100 included.
    // Note:
    // - we import Rng because `gen_range` needs this struct to be in scope (we don't see it but we need it)
    let secret = thread_rng().gen_range(1..=100);
    println!("Secret: OK");
    println!("==========================");

    let mut guess_count = 0;
    
    loop {
        print!("guess> ");
        // Print to stdout directly without waiting for a newline character
        stdout().flush().unwrap();

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Error: could not read stdin");

        // Exit if the user writes "q" or "quit"
        // Note:
        //  - guess is a String. "q" and "quit" are &str.
        //    To compare them using match, they must have the same type so we convert guess to a &str
        //    We could also use an if a==b statement directly, since the PartialEq trait is implemented for all pairs of str | &str | String
        match guess.as_str().trim() {
            "q" | "quit" => break,
            // Default arm: do nothing
            _ => (),
        }

        // Convert the guess String to a guess integer.
        // The same label (i.e. variable name) is used so we must explicitly convert the type.
        let guess: u32 = match guess.trim().parse() {
            // Ignore invalid inputs
            Err(_) => continue,
            Ok(input) => input,
        };

        guess_count += 1;

        // Match case using std::cmp and Ordering.
        match guess.cmp(&secret) {
            Ordering::Less => println!("$ bigger"),
            Ordering::Greater => println!("$ smaller"),
            Ordering::Equal => {
                println!("$ correct");
                break;
            }
        }
    }

    println!("Guesses: {guess_count}");
}
