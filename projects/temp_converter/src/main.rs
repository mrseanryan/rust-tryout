use std::io;

fn main() {
    println!("Convert temperature");

    println!("From [C]elsius or [F]ahrenheit?");
    let is_from_celsius = get_is_from_celsius();

    let message = if is_from_celsius {
        "Celsius to Fahrenheit"
    } else {
        "Fahrenheit to Celsius"
    };

    loop {
        println!("Converting {}", message);
        println!("Type a number and hit ENTER, or press CTRL + C to quit");

        let (input_temperature, is_input_ok) = get_number_from_keyboard();

        if !is_input_ok {
            println!("Could not read that number!");
            return;
        }

        let output_temperature = if is_from_celsius {
            convert_celsius(input_temperature)
        } else {
            convert_fahrenheit(input_temperature)
        };

        let input_unit = if is_from_celsius { "C" } else { "F" };
        let output_unit = if is_from_celsius { "F" } else { "C" };

        println!(
            "{}[{}] = {}[{}]",
            input_temperature, input_unit, output_temperature, output_unit
        );
    }
}

fn convert_celsius(degrees_fahrenheit: i32) -> i32 {
    (degrees_fahrenheit * 9 / 5) + 32
}

fn convert_fahrenheit(degrees_celsius: i32) -> i32 {
    (degrees_celsius - 32) * 5 / 9
}

fn get_is_from_celsius() -> bool {
    loop {
        // mutable, on the heap (not 's'tring)
        let mut input = String::new();

        // passing mutable ref
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // using match instead of expect, to handle the error (not just throw)
        // let input: bool =
        match input.trim() {
            "C" => return true,
            "F" => return false,
            _ => continue,
        };

        // return input;
    }
}

fn get_number_from_keyboard() -> (i32, bool) {
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
