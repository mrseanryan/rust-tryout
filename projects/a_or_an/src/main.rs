use a_or_an_lib;

fn main() {
    println!("a or an");

    test("antelope");
    test("user");
}

fn test(word: &str) {
    println!("{} {}", a_or_an_lib::get_a_or_an(word), word)
}
