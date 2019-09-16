pub fn get_a_or_an(word: &str) -> &str
{
    let first_letter = word.to_lowercase().chars().next().unwrap();

    if "aeiou".contains(first_letter) 
    {
        return "an";
    }

    "a"
}
