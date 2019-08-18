use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the secret number");

    let secret_number = rand::thread_rng().gen_range(1, 100 + 1);

    // TODO refactor into smaller fns
    loop {
        println!("Please input your guess");

        // mutable, on the heap (not 's'tring)
        let mut guess = String::new();

        // passing mutable ref
        io::stdin()
            .read_line(&mut guess)
            // arg to expect() is the error message to show:
            .expect("Failed to read line");

        // using match instead of expect, to handle the error (not just throw)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catchall pattern
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
