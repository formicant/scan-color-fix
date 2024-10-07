mod error;
mod image;
mod kernel;

use std::{iter, cmp::min, collections::VecDeque};
use std::io::{Read, Write};

use error::Error;
use image::{Image, Channel};
use kernel::Kernel;

const KERNEL_RADIUS: usize = 3;
const CHANNEL_OFFSET: f64 = 1.0 / 3.0;

pub fn fix_color<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), Error> {
    let mut image = Image::load(input)?;
    let (width, height) = image.dimensions();
    
    offset_channel(&mut image.pixel_data, width, height, Channel::Red, -CHANNEL_OFFSET);
    offset_channel(&mut image.pixel_data, width, height, Channel::Blue, CHANNEL_OFFSET);
    
    image.save(output)?;
    return Ok(());
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
