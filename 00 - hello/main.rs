// Import env package to access command-line arguments
use std::env;

fn main() {
    // Get command-line arguments and store them in a variable
    // Note:
    // - we skip the first argument (filename)
    // - the variable must be mutable because println will try to borrow it
    let mut args = env::args().skip(1);

    // Check if the user provided a name to greet as the first command-line argument
    if args.len() >= 1 {
        // Print the first argument.
        // Note:
        // - we need to unwrap it because it is a Some(String) and we want self (inner String)
        // - "{}" allows to format using a String (%s)
        // - if unwrap panics, it uses "world" (&str) as a fallback
        // - "world" needs to be casted to a String for unwrap_or to return a String
        println!("Hello, {}!", args.nth(0).unwrap_or("world".to_string()));
    } else {
        println!("Hello, world!");
    }
}