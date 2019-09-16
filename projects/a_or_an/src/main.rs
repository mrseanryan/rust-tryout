use a_or_an_lib;

fn main() {
    println!("= a or an =");
    println!("===========");

    test("alien");
    test("antelope");
    test("EU");
    test("FIFA");
    test("herb");
    test("hotel");
    test("MIA");
    test("UFO");
    test("UN");
    test("umbrella");
    test("user");
}

fn test(word: &str) {
    println!("{} {}", a_or_an_lib::get_a_or_an(word), word)
}
