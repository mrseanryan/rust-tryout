// TODO xxx
/*
- p79

- unit test
- space at start, end
- tabs
- punctuation
- Nth word
- all words, as vector
*/

fn main() {
    println!("words by slices");

    let sentence = String::from("This is a medium length sentence.");

    println!(
        "The first word of '{}' is '{}'",
        sentence,
        first_word(&sentence)
    );
}

fn first_word(s: &String) -> &str {
    // assumption: utf8 (1 char = 1 byte)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
