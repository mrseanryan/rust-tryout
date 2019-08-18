fn main() {
    println!("! Twelve Days !");

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Chrismas, my true love sent to me:\n  {}",
            days[day],
            generate_gifts(day)
        );
    }
}

// TODO review is there a better API?
fn generate_gifts(day: usize) -> String {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let mut verse = Vec::new();

    for gift in (0..day + 1).rev() {
        verse.push(gifts[gift]);
    }

    verse.join(", ")
}
