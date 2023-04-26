pub fn bionic_word(word: &str) -> String {
    if word.split_whitespace().count() == 0 {
        return String::from(word);
    }

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
}
pub fn bionic(text: &str) -> String {
    // keep the whitespace between words like the original
    let words = text.split(' ');

    words
        .map(bionic_word)
        .reduce(|acc, e| String::from(acc + " " + &e))
        .unwrap_or(String::new())
}

pub fn transform_html(content: &str, transform: fn(&str) -> String) -> String {
    let mut styled_content = String::new();
    let mut in_tag = false;
    let mut ignore_block = false;
    let mut text = String::new();

    for c in content.chars() {
        if c == '<' {
            in_tag = true;
            if text.len() > 0 {
                styled_content.push_str(&transform(&text));
                text = String::new();
            }
        } else if c == '>' {
            in_tag = false;
        }

        if in_tag || c == '>' || ignore_block {
            styled_content.push(c);

            if styled_content.ends_with("<style") {
                ignore_block = true;
            } else if styled_content.ends_with("</style") {
                ignore_block = false;
            }

            if styled_content.ends_with("<code") {
                ignore_block = true;
            } else if styled_content.ends_with("</code") {
                ignore_block = false;
            }
        } else {
            text.push(c);
        }
    }

    styled_content
}

#[cfg(test)]
mod test {
    use crate::{bionic, transform_html};

    #[test]
    fn bionic_transform() {
        let result = bionic("Hello world!");

        assert_eq!(result, "<b>He</b>llo <b>wor</b>ld!");
    }

    #[test]
    fn capitalize() {
        let result = transform_html(
            "<div><h1>hello <span>world</span><h1> <p>my name is taher</p></div>",
            |s| s.to_uppercase(),
        );

        assert_eq!(
            result,
            "<div><h1>HELLO <span>WORLD</span><h1> <p>MY NAME IS TAHER</p></div>"
        );
    }

    #[test]
    fn bionic_html() {
        let result = transform_html(
            "<div><h1>hello <span>world</span><h1> <p>my name is taher</p></div>",
            bionic,
        );

        assert_eq!(result, "<div><h1><b>he</b>llo <span><b>wo</b>rld</span><h1> <p><b>m</b>y <b>na</b>me <b>i</b>s <b>ta</b>her</p></div>");
    }
}
