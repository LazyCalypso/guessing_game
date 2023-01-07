// includes
use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 20!");

    // Generates a random u32 number between 1 and 20
    let secret_number = rand::thread_rng()
                                    .gen_range(1..=20);

    loop {
        print!("INPUT: ");
        // Flush output stream to ensure all buffered contents reach their destination
        io::stdout()
            .flush()
            .expect("Flush failed");

        // Creates a new mutable variable of type string
        let mut guess = String::new();

        // Reads a line of input and stores it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converts the string guess variable to a u32
        let guess: u32 = match guess
                                .trim()
                                .parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compares the guess variable to the secret_number variable
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!"); 
                break; 
            }
        }
    }    
}
