mod error;
mod image;
mod kernel;
mod timing;

use std::{iter, cmp::min, collections::VecDeque, path::Path, fs::File};
use error::Error;
use image::{Image, Channel};
use kernel::Kernel;

const KERNEL_RADIUS: usize = 3;

fn main() -> Result<(), Error> {
    let input_path = Path::new(r"img/text600.png");
    let output_path = Path::new(r"img/out.png");
    
    let mut timing = timing::Timing::new();
    println!("Start");
    
    let input_file = File::open(input_path)?;
    let mut image = Image::load(input_file)?;
    timing.mark("Decoding");
    
    fix_color(&mut image);
    timing.mark("Processing");
    
    let output_file = File::create(output_path)?;
    image.save(output_file)?;
    timing.mark("Encoding");
    
    println!("{timing}");
    return Ok(());
}

fn fix_color(image: &mut Image) {
    let width = image.width();
    let height = image.height();
    let offset = 1.0 / 3.0;
    offset_channel(&mut image.pixel_data, width, height, Channel::Red, -offset);
    offset_channel(&mut image.pixel_data, width, height, Channel::Blue, offset);
}

fn offset_channel(pixel_data: &mut[u8], width: usize, height: usize, channel: Channel, offset: f64) {
    let kernel = Kernel::translation_lanczos(KERNEL_RADIUS, offset);
    let right_radius = kernel.right_radius();
    let stride = width * 3;
    let bottom = height - 1;
    
    for x in 0..width {
        let index_offset = x * 3 + channel as usize;
        
        let mut window: VecDeque<u8> = iter::repeat(0).take(kernel.left_radius()).chain(0..right_radius)
            .map(|y| pixel_data[min(bottom, y) * stride + index_offset])
            .collect();
        
        for y in 0..height {
            let value = kernel.apply(window.iter());
            pixel_data[y * stride + index_offset] = value;
            
            window.pop_front();
            window.push_back(pixel_data[min(bottom, y + right_radius) * stride + index_offset]);
        }
    }
}
