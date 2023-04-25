use std::{error::Error, fs, io::{Read, Write}, str::from_utf8};

use berg::style_text;
use zip;

fn main() -> Result<(), Box<dyn Error>>{
    let reader = fs::File::open("rust.epub").unwrap();
    let out = fs::File::create("rust_.epub").unwrap();
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut out_zip = zip::ZipWriter::new(out);
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        
        println!("Filename: {}", file.name());

        let options = zip::write::FileOptions::default()
            .compression_method(file.compression());
        
        out_zip.start_file(file.name(), options)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        
        if file.name().ends_with(".xhtml") {
            let content = from_utf8(&buf)
                .unwrap()
                .to_owned();

            let styled_content = style_html_content(content);
            
            out_zip.write(styled_content.as_bytes())?;
        } else {
            out_zip.write(&buf)?;
        }
    }

    out_zip.finish()?;

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
            if text.len() > 0 {
                text.push(' ');
            }
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
