use rand::{self, Rng, thread_rng};
use rand::distributions::range::SampleRange;
use std::ops::Range;

// Flip a coin using the thread RNG.
pub fn coin_flip() -> bool {
    thread_rng().gen()
}

// Sample `amount` values from an iterable using the thread RNG.
// Order of sampled values will not be random.
pub fn sample<T, I>(iterable: I, amount: usize) -> Vec<T>
    where I: IntoIterator<Item=T>
{
    rand::sample(&mut thread_rng(), iterable, amount)
}

// Sample `amount` values from an iterable using the thread RNG.
// Order of sampled values will be random.
pub fn shuffle_sample<T, I>(iterable: I, amount: usize) -> Vec<T>
    where I: IntoIterator<Item=T>
{
    let mut sample_vec = sample(iterable, amount);
    thread_rng().shuffle(&mut sample_vec[..]);
    sample_vec
}

// Pick a random value from an iterable using the thread RNG.
pub fn pick<T, I>(iterable: I) -> T
    where I: IntoIterator<Item=T>
{
    sample(iterable, 1).remove(0)
}

// Sample a value from a range using the thread RNG.
pub fn random_range<T: PartialOrd + SampleRange>(r: Range<T>) -> T {
    thread_rng().gen_range(r.start, r.end)
}

// Sample two distinct values from a range using the thread RNG.
pub fn random_range_two<T: PartialOrd + SampleRange>(r: Range<T>) -> (T, T)
    where Range<T>: IntoIterator<Item = T>
{
    let mut s = shuffle_sample(r, 2);
    let b = s.remove(1);
    let a = s.remove(0);
    (a, b)
}

pub fn punctuation_name(c: char) -> &'static str {
    match c {
        '!' => "exclamation mark",
        '"' => "quotation mark",
        '#' => "number sign",
        '$' => "dollar sign",
        '%' => "percent sign",
        '\'' => "apostrophe",
        '(' => "left parenthesis",
        ')' => "right parenthesis",
        '*' => "asterisk",
        '+' => "plus sign",
        ',' => "comma",
        '-' => "hyphen",
        '.' => "period",
        '/' => "slash",
        '0' => "zero",
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        ':' => "colon",
        ';' => "semicolon",
        '<' => "less-than sign",
        '=' => "equals sign",
        '>' => "greater-than sign",
        '?' => "question mark",
        '@' => "at sign",
        _ => "strange character"
    }
}

pub fn ordinal(n: i32) -> String {
    let i = if 10 <= n && n <= 19 {0} else {n % 10};
    n.to_string() + ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"][i as usize]
}

pub fn a_or_an(s: &str) -> String {
    match s.chars().nth(0) {
        None => String::new(),
        Some(c) => {
            let article = match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' =>
                    // special case: "one" can be returned by punctuation_name
                    if s == "one" {
                        "a"
                    } else {
                        "an"
                    },
                _ => "a"
            };
            format!("{} {}", article, s)
        }
    }
}
