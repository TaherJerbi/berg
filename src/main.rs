use std::{
    error::Error,
    fs,
    io::{Read, Write},
    str::from_utf8,
    time::Instant,
};

use berg::{bionic, transform_html};
use zip;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let reader = fs::File::open("rust.epub").unwrap();
    let out = fs::File::create("rust_.epub").unwrap();
    let mut debug_out = fs::File::create("debug.html").unwrap();
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut out_zip = zip::ZipWriter::new(out);
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        let options = zip::write::FileOptions::default();

        out_zip.start_file(file.name(), options)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if file.name().ends_with(".xhtml") {
            println!("Filename: {}", file.name());
            let content = from_utf8(&buf).unwrap();

            let styled_content = transform_html(content, bionic);

            debug_out.write(styled_content.as_bytes())?;

            out_zip.write(styled_content.as_bytes())?;
        } else {
            out_zip.write(&buf)?;
        }
    }

    out_zip.finish()?;
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    Ok(())
}
