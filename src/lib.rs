use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    str::from_utf8,
};
use zip::ZipArchive;

/// EpubDocument represents an epub file
/// It can be opened and transformed.
/// The transformation is done by a Transformer.
/// The Transformer is a trait that can be implemented to transform the content of the epub file in any way the user wants **while keeping the epub structure intact**
pub struct EpubDocument {
    zip: ZipArchive<File>,
}

/// EpubDocument is the main struct that represents an epub file
impl EpubDocument {
    /// open an epub file
    /// # Example
    /// ```no_run
    /// use berg::EpubDocument;
    ///
    /// let epub = EpubDocument::open("rust.epub");
    /// ```
    /// # Errors
    /// This function will return an error if the file does not exist or if it is not a valid epub file
    pub fn open(path: &str) -> Result<EpubDocument, Box<dyn Error>> {
        let reader = File::open(path)?;
        let zip = ZipArchive::new(reader)?;

        Ok(EpubDocument { zip })
    }
    /// Transform the epub file using a Transformer and write the result to a file.
    /// # Example
    /// ```no_run
    /// use berg::{BionicTransformer, EpubDocument};
    /// use std::{error::Error, fs, time::Instant};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///    let mut out = fs::File::create("rust_.epub").unwrap();
    ///
    ///    let mut epub = EpubDocument::open("rust.epub")?;
    ///
    ///   epub.transform(BionicTransformer::new(), &mut out)?;
    ///   Ok(())
    ///
    /// }
    /// ```
    pub fn transform<T: Transformer>(
        &mut self,
        transformer: T,
        out: &mut File,
    ) -> Result<(), Box<dyn Error>> {
        let mut out_zip = zip::ZipWriter::new(out);
        for i in 0..self.zip.len() {
            let mut file = self.zip.by_index(i)?;

            let options = zip::write::FileOptions::default();

            out_zip.start_file(file.name(), options)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            if file.name().ends_with(".xhtml") {
                let content = from_utf8(&buf).unwrap();

                let styled_content = Self::transform_html(content, &transformer);

                out_zip.write(styled_content.as_bytes())?;
            } else {
                out_zip.write(&buf)?;
            }
        }

        Ok(())
    }

    /// Transform the html content of the epub file using a Transformer
    /// # Example
    /// ```
    /// use berg::{EpubDocument, Transformer};
    ///    fn capitalize() {
    ///        struct CapitalizeTransformer;
    ///
    ///        impl Transformer for CapitalizeTransformer {
    ///            fn transform(&self, content: &str) -> String {
    ///                content.to_uppercase()
    ///            }
    ///        }
    ///
    ///        let result = EpubDocument::transform_html(
    ///            "<div><h1>hello <span>world</span><h1> <p>my name is taher</p></div>",
    ///            &CapitalizeTransformer {},
    ///        );
    ///
    ///        assert_eq!(
    ///            result,
    ///            "<div><h1>HELLO <span>WORLD</span><h1> <p>MY NAME IS TAHER</p></div>"
    ///        );
    ///    }
    /// ```
    pub fn transform_html<T: Transformer>(content: &str, transformer: &T) -> String {
        let mut styled_content = String::new();
        let mut in_tag = false;
        let mut ignore_block = false;
        let mut text = String::new();

        for c in content.chars() {
            if c == '<' {
                in_tag = true;
                if text.len() > 0 {
                    styled_content.push_str(&transformer.transform(&text));
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
}

/// Transformer is a trait that can be implemented to transform a string into another string. It is used to transform the content of the epub file in any way the user wants **while keeping the epub structure intact**
/// # Example
/// ```
/// use berg::Transformer;
/// struct CapitalizeTransformer;
/// impl Transformer for CapitalizeTransformer {
///    fn transform(&self, content: &str) -> String {
///       content.to_uppercase()
///   }
/// }
/// ```
pub trait Transformer {
    fn transform(&self, content: &str) -> String;
}

/// BionicTransformer is a Transformer that makes the text in the Bionic Reading format
pub struct BionicTransformer;
impl BionicTransformer {
    pub fn new() -> BionicTransformer {
        BionicTransformer {}
    }

    fn bionic_word(word: &str) -> String {
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
    /// Transform the text in the Bionic Reading format
    /// # Example
    /// ```
    /// use berg::BionicTransformer;
    /// let result = BionicTransformer::bionic("Hello world!");
    /// assert_eq!(result, "<b>He</b>llo <b>wor</b>ld!");
    /// ```
    /// # Note
    /// The implementation is not perfect and it doesn't work well with words that have special characters like `Hello, world!` or `Hello-world!`
    /// # TODO
    /// - [ ] Fix the implementation to work with special characters
    /// - [ ] Add parameters to control the behavior of the bionic transformer (fixation, saccade, etc.)
    pub fn bionic(text: &str) -> String {
        // keep the whitespace between words like the original
        let words = text.split(' ');

        words
            .map(Self::bionic_word)
            .reduce(|acc, e| String::from(acc + " " + &e))
            .unwrap_or(String::new())
    }
}

impl Transformer for BionicTransformer {
    fn transform(&self, content: &str) -> String {
        Self::bionic(content)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bionic_transform() {
        let result = BionicTransformer::bionic("Hello world!");

        assert_eq!(result, "<b>He</b>llo <b>wor</b>ld!");
    }

    #[test]
    fn capitalize() {
        struct CapitalizeTransformer;

        impl Transformer for CapitalizeTransformer {
            fn transform(&self, content: &str) -> String {
                content.to_uppercase()
            }
        }

        let result = EpubDocument::transform_html(
            "<div><h1>hello <span>world</span><h1> <p>my name is taher</p></div>",
            &CapitalizeTransformer {},
        );

        assert_eq!(
            result,
            "<div><h1>HELLO <span>WORLD</span><h1> <p>MY NAME IS TAHER</p></div>"
        );
    }

    #[test]
    fn bionic_html() {
        let result = EpubDocument::transform_html(
            "<div><h1>hello <span>world</span><h1> <p>my name is taher</p></div>",
            &BionicTransformer::new(),
        );

        assert_eq!(result, "<div><h1><b>he</b>llo <span><b>wo</b>rld</span><h1> <p><b>m</b>y <b>na</b>me <b>i</b>s <b>ta</b>her</p></div>");
    }
}
