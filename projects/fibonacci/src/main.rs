use std::io;

fn main() {
    loop {
        println!("Calc the Nth Fibonacci number");
        println!("Type a number and hit ENTER, or press CTRL + C to quit");

        let (input_number, is_input_ok) = get_number_from_keyboard();

        if !is_input_ok {
            println!("Could not read that number - Bye!");
            return;
        }

        let fibo_result = get_nth_fibonacci(input_number);

        println!("The {}th Fibonacci is {}", input_number, fibo_result);
    }
}

fn get_nth_fibonacci(desired_index: u32) -> u128 {
    let mut current_index = 0;

    let mut preceding = 0;
    let mut previous_preceding = 1;
    let mut next;

    loop {
        next = preceding + previous_preceding;
        previous_preceding = preceding;
        preceding = next;

        print!("{} ", next);

        current_index += 1;

        if current_index == desired_index {
            break;
        }
    }

    next
}

fn get_number_from_keyboard() -> (u32, bool) {
    // mutable, on the heap (not 's'tring)
    let mut number = String::new();

    // passing mutable ref
    io::stdin()
        .read_line(&mut number)
        // arg to expect() is the error message to show:
        .expect("Failed to read line");
    match number.trim().parse() {
        Ok(num) => (num, true),
        // _ is a catchall pattern
        Err(_) => (666, false),
    }
}
