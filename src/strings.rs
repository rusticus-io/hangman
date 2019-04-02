
pub fn get_first_unicode_char(s: &str) -> (&str, &str) {

    for i in 1..5 {
        if let Some(head) = s.get(0..i) { return (head, &s[i..]) }
    }

    (&s[0..0], s)
}