
use regex::Regex;

pub fn get_first_unicode_char(s: &str) -> (&str, &str) {

    for i in 1..5 {
        if let Some(head) = s.get(0..i) { return (head, &s[i..]) }
    }

    (&s[0..0], s)
}

pub fn get_search_string() -> String {
    String::from("HANGMAN")
}

pub fn prepare(s: &str) -> String {
    let re = Regex::new(r".").expect("Can't create regular expression");
    re.replace_all(s, "_").to_string()
}