use std::io;

fn main() {
    println!("Guess the secret number");

    println!("Please input your guess");

    // mutable, on the heap (not 's'tring)
    let mut guess = String::new();

    // passing mutable ref
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
