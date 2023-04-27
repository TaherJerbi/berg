# BERG

berg is a Rust library that provides a way to transform the contents of Epub documents **while keeping the structure intact**.

## Usage

```rust
use berg::{EpubDocument, Transformer};

// Define a transformer
struct CapitalizeTransformer;
impl Transformer for CapitalizeTransformer {
    fn transform(&self, content: &str) -> String {
        content.to_uppercase()
    }
}

// Open an epub document
let mut epub = EpubDocument::open("rust.epub")?;

// Create the output file
let mut out = fs::File::create("rust_transformed.epub").unwrap();

// Transform the epub document using the transformer
epub.transform(CapitalizeTransformer {}, &mut out)?;
```

## Bionic Transformer

`BionicTransformer` is a `Transformer` that makes the text in the Bionic Reading format.

```rust
use berg::{EpubDocument, BionicTransformer};

let result = BionicTransformer::bionic("Hello world!");

assert_eq!(result, "<b>He</b>llo <b>wor</b>ld!");
```

### Note

The implementation is not perfect and it doesn't work well with words that have special characters like `Hello, world!` or `Hello-world!`

### TODO

- [ ] Fix the implementation to work with special characters
- [ ] Add parameters to control the behavior of the bionic transformer (fixation, saccade, etc.)

## Motivation

- Apply newly acquired Rust knowledge after completing the official book
- Wanted to create a tool that could transform an EPUB file into _Bionic Reading_ format while keeping the original structure of the file intact

## Features

- [x] Transform the contents of an EPUB file
- [x] Keep the structure of the EPUB file intact
- [x] Bionic Transformer
- [ ] Make CLI tool to transform EPUB files using the Bionic Transformer
