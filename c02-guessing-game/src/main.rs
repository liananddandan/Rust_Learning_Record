use std::io; // Bring the standard input/output library into scope
use rand::Rng; // Trait needed to use random number generation
use std::cmp::Ordering; // Enum for comparing values

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Uncomment this line to debug the secret number
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Create a mutable String to hold user input

        // Read the input from stdin and store it in 'guess'
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Try to parse the input into a number; continue if invalid
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop if correct
            }
        }
    }
}
