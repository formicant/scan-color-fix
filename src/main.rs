mod error;

use std::{fs::File, path::Path, io::BufWriter};
use error::Error;

struct Image {
    info: png::Info<'static>,
    pixel_data: Vec<u8>,
}

fn main() -> Result<(), Error> {
    let input_path = Path::new(r"img/sample.png");
    let output_path = Path::new(r"img/out.png");
    
    let mut image = load_image(input_path)?;
    fix_color(&mut image);
    save_image(output_path, image)?;
    
    return Ok(());
}

fn load_image(path: &Path) -> Result<Image, Error> {
    let file = File::open(path)?;
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info()?;
    
    let (color_type, bit_depth) = reader.output_color_type();
    if color_type != png::ColorType::Rgb
        || bit_depth != png::BitDepth::Eight
        || reader.info().is_animated() {
        return Err(Error::UnsupportedImageType);
    }
    
    let size = reader.output_buffer_size();
    let mut pixel_data = vec![0; size];
    reader.next_frame(&mut pixel_data)?;
    
    reader.finish()?;
    let info = reader.info().clone();
    
    return Ok(Image { info, pixel_data });
}

fn save_image(path: &Path, image: Image) -> Result<(), Error> {
    let file = File::create(path)?;
    let encoder = png::Encoder::with_info(BufWriter::new(file), image.info)?;
    
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image.pixel_data)?;
    
    return Ok(());
}

fn fix_color(image: &mut Image) {
    let size = image.pixel_data.len();
    for i in (0..size).step_by(3) {
        image.pixel_data.swap(i, i + 2);
    }
}