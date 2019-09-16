pub fn get_a_or_an(word: &str) -> &str
{
    let mut is_an = is_naively_an(word);

    if is_exception(word)
    {
        is_an = !is_an;
    }

    if is_an
    {
        return "an"
    }
    else 
    {
     return "a";
    }
}

fn is_naively_an(word: &str) -> bool
{
    let first_letter = word.to_lowercase().chars().next().unwrap();

    if "aeiou".contains(first_letter) 
    {
        return true;
    }

    false
}

fn is_exception(word: &str) -> bool
{
    // ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/irregular-words.js

    let exceptions = [
        // Nouns: eu like y
        "eunuch", "eucalyptus", "eugenics", "eulogy", "euphemism", "euphony", "euphoria", "eureka",

        // Adjectives: eu like y
        "european", "euphemistic", "euphonic", "euphoric",

        // Adverbs: eu like y
        "euphemistically", "euphonically", "euphorically",

        // Nouns: silent h
        "heir", "heiress", "herb", "homage", "honesty", "honor", "honour", "hour",

        // Adjectives: silent h
        "honest", "honorous",

        // Adverbs: silent h
        "honestly", "hourly",

        // Nouns: o like w
        "one", "ouija",

        // Adjectives: o like w
        "once",

        // Adverbs: o like w

        // Nouns: u like y
        "ubiquity", "udometer", "ufo", "uke", "ukelele", "ululate", "unicorn", "unicycle", "uniform",
        "unify", "union", "unison", "unit", "unity", "universe", "university", "upas", "ural", "uranium",
        "urea", "ureter", "urethra", "urine", "urology", "urus", "usage", "use", "user", "usual", "usurp",
        "usury", "utensil", "uterus", "utility", "utopia", "utricle", "uvarovite", "uvea", "uvula",

        // Adjectives: u like y
        "ubiquitous", "ugandan", "ukrainian", "unanimous", "unicameral", "unified", "unique", "unisex",
        "universal", "urinal", "urological", "useful", "useless", "usurious", "usurped", "utilitarian",
        "utopic",

        // Adverbs: u like y
        "ubiquitously", "unanimously", "unicamerally", "uniquely", "universally", "urologically", "usefully", "uselessly", "usuriously",

        // Nouns: y like i
        "yttria", "yggdrasil", "ylem", "yperite", "ytterbia", "ytterbium", "yttrium",

        // Adjectives: y like i
        "ytterbous", "ytterbic", "yttric"
        ];

    return exceptions.contains(&word);
}