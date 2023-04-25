use std::{error::Error, fs};

use berg::style_text;
use epub::doc::EpubDoc;
use epub_builder::{EpubBuilder, ZipLibrary, EpubContent};

fn main() -> Result<(), Box<dyn Error>> {
    let mut doc = EpubDoc::new("rust.epub").unwrap();

    let creator = doc.mdata("creator").unwrap_or("".to_owned());
    let title = doc.mdata("title").unwrap_or("".to_owned());
    let zip = ZipLibrary::new()?;
    // Create a new EpubBuilder using the zip library
    let mut builder = EpubBuilder::new(zip)?;
    // Set some metadata
    builder.metadata("author", creator)?;
    builder.metadata("title", title)?;

    let resources = doc.resources.clone();
    let spine = doc.spine.clone();

    // Put all resources except chapters
    for (resource_id, (resource_path, resource_mime)) in &resources {
        if resource_mime != "application/xhtml+xml" {
            let content = &mut doc.get_resource(&resource_id)?;
            builder.add_resource(
                resource_path.to_str().unwrap(), &content[..], resource_mime)?;
        }
    }

    for content_id in spine {
        let (path, mime) = &resources.get(&content_id).unwrap();
        let content = doc.get_resource_str(&content_id).unwrap();
        let styled_content = style_html_content(content);
        dbg!(&path);
        builder.add_content(EpubContent::new(path.to_str().unwrap(), styled_content.as_bytes()))?;
    }

    builder.generate(fs::File::create("rust_.epub").unwrap())?;
        
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
