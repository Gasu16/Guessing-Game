use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");

    // Generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // Variable "guess" with mutable value

        let mut guess = String::new();

        // Handling invalid input

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // We "shadow" the previous value of "guess" with a new value
        // We "convert" guess to unsigned 32-bit number type

        let guess: u32 = match guess.trim().parse() {
            // "guess" refers to the original "guess" variable
            // trim() removes any whitespace at the beginning and at the end
            // parse() parses a string into some kind of number
            // we have to tell Rust the exact number type we want by using let guess: u32
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed: {}", guess);

    }

}
