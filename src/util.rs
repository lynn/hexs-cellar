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
