use std::io::Write;

use berg::style_text;
use epub::doc::EpubDoc;
use mobi::{Mobi, MobiError};

fn main() {
    let mut doc = EpubDoc::new("rust.epub").unwrap();

    let mut content = String::from(doc.get_current_str().unwrap());
    while let Ok(()) = doc.go_next() {
        content.push_str(&doc.get_current_str().unwrap());
    }

    let styled_content = style_html_content(content);

    let mut out = std::fs::File::create("rust.out").unwrap();
    out.write_all(styled_content.as_bytes()).unwrap();
}

fn style_mobi(input_file: &str, output_file: &str) -> Result<(), MobiError> {
    let m = Mobi::from_path(input_file)?;

    // Access content
    let content = m.content_as_string_lossy();
    let styled_content = style_html_content(content);

    let mut out = std::fs::File::create(output_file).unwrap();
    out.write_all(styled_content.as_bytes())?;

    Ok(())
}

fn style_html_content(content: String) -> String {
    let mut styled_content = String::new();
    let mut in_tag = false;
    let mut text = String::new();
    // apply style_text on everything but tags
    for c in content.chars() {
        if c == '<' {
            in_tag = true;
            styled_content.push_str(&style_text(&text));
            text = String::new();
        } else if c == '>' {
            in_tag = false;
        }
        if in_tag || c == '>' {
            styled_content.push(c);
        } else {
            text.push(c);
        }
    }

    styled_content
}
