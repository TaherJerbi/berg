pub fn style_text(text: &str) -> String {
    let words = text.split_ascii_whitespace();

    words
        .map(|word| {
            let mid_point = word.chars().count() / 2;

            let chars: Vec<char> = word.chars().collect();
            let split_chars = chars.split_at(mid_point);

            let new_word = String::from(
                "<b>".to_owned()
                    + &String::from_iter(split_chars.0)
                    + "</b>"
                    + &String::from_iter(split_chars.1),
            );

            new_word
        })
        .reduce(|acc, e| String::from(acc + " " + &e))
        .unwrap_or(String::new())
}

#[cfg(test)]
mod test {
    use crate::style_text;

    #[test]
    fn example_1() {
        let result = style_text("Hello world!");

        assert_eq!(result, "<b>He</b>llo <b>wor</b>ld!");
    }
}
