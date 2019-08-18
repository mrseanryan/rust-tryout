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
        "(unused)",
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

    for gift in (1..day + 1).rev() {
        verse.push(gifts[gift]);
    }

    let is_first_day_only = day == 0;

    let first_day = if is_first_day_only {
        "a partridge in a pear tree"
    } else {
        " and a partridge in a pear tree"
    };
    let first_day = String::from(first_day);

    let mut verse = verse.join(", ");

    verse.push_str(&first_day);

    verse
}
