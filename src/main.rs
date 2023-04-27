use berg::{BionicTransformer, EpubDocument};
use std::{error::Error, fs, io::Write, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut out = fs::File::create("rust_.epub").unwrap();

    let mut epub = EpubDocument::open("rust.epub")?;

    epub.transform(BionicTransformer::new(), &mut out)?;

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    out.flush()?;
    Ok(())
}
