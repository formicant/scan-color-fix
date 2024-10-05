mod error;
mod kernel;
mod timing;

use std::{collections::VecDeque, fs::File, io::BufWriter, iter, path::Path, cmp::min};
use error::Error;
use kernel::Kernel;

const KERNEL_RADIUS: usize = 3;

const RED: usize = 0;
const BLUE: usize = 2;

struct Image {
    info: png::Info<'static>,
    pixel_data: Vec<u8>,
}

fn main() -> Result<(), Error> {
    let input_path = Path::new(r"img/text300.png");
    let output_path = Path::new(r"img/out.png");
    
    let mut timing = timing::Timing::new();
    println!("Start");
    
    let mut image = load_image(input_path)?;
    timing.mark("Decoding");
    fix_color(&mut image);
    timing.mark("Processing");
    save_image(output_path, image)?;
    timing.mark("Encoding");
    
    println!("{timing}");
    return Ok(());
}

fn load_image(path: &Path) -> Result<Image, Error> {
    let file = File::open(path)?;
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info()?;
    
    let (color_type, bit_depth) = reader.output_color_type();
    if reader.info().is_animated()
        || color_type != png::ColorType::Rgb
        || bit_depth != png::BitDepth::Eight
    {
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
    let width = image.info.width as usize;
    let height = image.info.height as usize;
    offset_channel(&mut image.pixel_data, width, height, RED, -1.0 / 3.0);
    offset_channel(&mut image.pixel_data, width, height, BLUE, 1.0 / 3.0);
}

fn offset_channel(pixel_data: &mut[u8], width: usize, height: usize, channel: usize, offset: f64) {
    let stride = width * 3;
    let bottom = height - 1;
    let kernel = Kernel::translation_lanczos(KERNEL_RADIUS, offset);
    let kernel_rest = kernel.values.len() - kernel.center_index;
    
    for x in 0..width {
        let index_offset = x * 3 + channel;
        
        let mut source: VecDeque<f64> = iter::repeat(0).take(kernel.center_index).chain(0..kernel_rest)
            .map(|y| pixel_data[min(bottom, y) * stride + index_offset] as f64)
            .collect();
        
        for y in 0..height {
            let value: f64 = iter::zip(kernel.values.iter(), source.iter()).map(|(k, s)| k * s).sum();
            pixel_data[y * stride + index_offset] = value.clamp(0.0, 255.0).round() as u8;
            
            source.pop_front();
            source.push_back(pixel_data[min(bottom, y + kernel_rest) * stride + index_offset] as f64);
        }
    }
}
