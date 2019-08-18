use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the secret number");

    let secret_number = rand::thread_rng().gen_range(1, 100 + 1);

    loop {
        println!("Please input your guess [1..100]");

        let guess = get_next_guess();

        println!("You guessed: {}", guess);

        let is_done = is_guess_correct(guess, secret_number);

        if is_done {
            break;
        }
    }
}

fn get_next_guess() -> i32 {
    // mutable, on the heap (not 's'tring)
    let mut guess = String::new();

    // passing mutable ref
    io::stdin()
        .read_line(&mut guess)
        // arg to expect() is the error message to show:
        .expect("Failed to read line");

    // using match instead of expect, to handle the error (not just throw)
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        // _ is a catchall pattern
        Err(_) => -1,
    };

    return guess;
}

fn is_guess_correct(guess: i32, secret_number: i32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
