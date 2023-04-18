use image::ImageFormat;
use std::fs::{read, write};
use std::io::Result;
use std::ops::Deref;
use std::path::PathBuf;
use webp::Encoder;

pub fn convert(path: PathBuf, output: String) -> Result<String> {
    println!("path: {:?}", path);

    let image_data = read(path)?;

    let image = image::load_from_memory_with_format(&image_data, ImageFormat::Png)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let encoder = Encoder::from_image(&image)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let encoded = encoder.encode(100.00);

    let data = encoded.deref();

    let output_path = "/home/riri/dev/rust/webpConvert/output/".to_owned() + &output + ".webp";

    write(&output_path, data)?;

    Ok(output_path)
}
