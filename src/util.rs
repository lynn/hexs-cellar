use rand::{self, Rng, thread_rng};
use rand::distributions::range::SampleRange;
use sprite::Color;
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

pub fn address_name(address: u8) -> &'static str {
    match address {
        0x00 => "player appearance",
        0x01 => "player name[0]",
        0x02 => "player name[1]",
        0x03 => "player name[2]",
        0x04 => "player name[3]",
        0x05 => "player name[4]",
        0x06 => "player name[5]",
        0x07 => "player name[6]",
        0x08 => "player name[7]",
        0x09 => "player name[8]",
        0x0a => "player name[9]",
        0x0b => "player name[10]",
        0x0c => "player name[11]",
        0x0d => "player name[12]",
        0x0e => "player name[13]",
        0x0f => "player name[14]",
        0x10 => "monster 1 flags",
        0x11 => "monster 1 position",
        0x12 => "monster 1 hp",
        0x13 => "monster 2 flags",
        0x14 => "monster 2 position",
        0x15 => "monster 2 hp",
        0x16 => "monster 3 flags",
        0x17 => "monster 3 position",
        0x18 => "monster 3 hp",
        0x19 => "monster 4 flags",
        0x1a => "monster 4 position",
        0x1b => "monster 4 hp",
        0x1c => "monster 5 flags",
        0x1d => "monster 5 position",
        0x1e => "monster 5 hp",
        0x1f => "spell memory",
        0x20 => "identification[0]",
        0x21 => "identification[1]",
        0x22 => "identification[2]",
        0x23 => "identification[3]",
        0x24 => "poison",
        0x25 => "haste",
        0x26 => "charge",
        0x27 => "protect",
        0x28 => "inventory[0]",
        0x29 => "inventory[1]",
        0x2a => "inventory[2]",
        0x2b => "inventory[3]",
        0x2c => "inventory[4]",
        0x2d => "inventory[5]",
        0x2e => "inventory[6]",
        0x2f => "inventory[7]",
        0x30 => "door appearance",
        0x31 => "wall appearance",
        0x32 => "floor color",
        0x33 => "stairs delta",
        0x34 => "timer delta",
        0x35 => "damage offset",
        0x36 => "0x36",
        0x37 => "0x37",
        0x38 => "0x38",
        0x39 => "player hp",
        0x3a => "player tp",
        0x3b => "player xl/def",
        0x3c => "player position",
        0x3d => "player depth",
        0x3e => "player metal/acid",
        0x3f => "player fire/elec",
        _    => "invalid address",
    }
}

pub fn color_name(color: Color) -> &'static str {
    match color {
        Color::Navy => "navy",
        Color::Green => "green",
        Color::Teal => "teal",
        Color::Maroon => "maroon",
        Color::Purple => "purple",
        Color::Brown => "brown",
        Color::Gray => "gray",
        Color::Dark => "dark gray",
        Color::Blue => "blue",
        Color::Lime => "lime",
        Color::Aqua => "cyan",
        Color::Red => "red",
        Color::Pink => "pink",
        Color::Yellow => "yellow",
        Color::White => "white",
    }
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
