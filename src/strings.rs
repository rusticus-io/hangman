
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

pub fn create_hint(search_string: &str, already_matched: &str) -> String {
    if already_matched.eq("") {
        prepare(search_string)
    }
    else {
        let re = Regex::new( ("[^".to_string() + already_matched + "]").as_str())
            .expect("Can't create regular expression");
        re.replace_all(search_string, "_").to_string()
    }
}

pub fn len_of_string(s: &str) -> i32 {
    let (head, rest) = get_first_unicode_char(s);
    if head.eq("") && rest.eq("") { 0 }
    else if rest.eq("") { 1 }
    else {1 + len_of_string(rest) }
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

    #[test]
    fn test_create_hint() {
        assert_eq!("_____", create_hint("hallo", "").as_str());
        assert_eq!("_all_", create_hint("hallo", "la").as_str());
        assert_eq!("hallo", create_hint("hallo", "hola").as_str());
    }

    #[test]
    fn test_len_of_string() {
        assert_eq!(0, len_of_string(""));
        assert_eq!(5, len_of_string("hallo"));
        assert_eq!(3, len_of_string("olá"));
    }

}