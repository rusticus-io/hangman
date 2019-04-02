
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

pub fn find_char_in_string(ch: &str, s: &str) -> bool {

    let (head, rest) = get_first_unicode_char(s);
    if head.eq(ch) {
        return true
    }
    else if rest.ne("") {
        return find_char_in_string(ch, rest)
    }

    false
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_char_in_string() {
        assert_eq!(false, find_char_in_string("", "hallo"));
        assert_eq!(false, find_char_in_string("c", "hallo"));
        assert_eq!(true, find_char_in_string("a", "hallo"));
        assert_eq!(true, find_char_in_string("h", "hallo"));
        assert_eq!(true, find_char_in_string("o", "hallo"));
    }
}