mod error;
mod image;
mod kernel;

use std::{iter, cmp::min, cmp::max, collections::VecDeque};
use std::io::{Read, Write};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::{current_num_threads, slice::ParallelSliceMut};

use error::Error;
use image::{Image, Channel};
use kernel::Kernel;

const KERNEL_RADIUS: usize = 3;
const CHANNEL_OFFSET: f64 = 1.0 / 3.0;

pub fn fix_color<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), Error> {
    let input_image = Image::load(input)?;
    let output_image = process_image(&input_image);
    output_image.save(output)?;
    return Ok(());
}

fn process_image(input_image: &Image) -> Image {
    let mut output_image = input_image.clone();
    let (width, height) = output_image.dimensions();
    
    let red_kernel = Kernel::translation_lanczos(KERNEL_RADIUS, -CHANNEL_OFFSET);
    let blue_kernel = Kernel::translation_lanczos(KERNEL_RADIUS, CHANNEL_OFFSET);
    
    // slice the output image into `chunk_count` chunks by rows and process them in parallel
    let chunk_count = current_num_threads();
    let rows_per_chunk = (height - 1) / chunk_count + 1; // ceil(height / chunk_count)
    let chunk_size = rows_per_chunk * width * 3;
    
    output_image.pixel_data.par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(index, chunk)| {
            let first_row = rows_per_chunk * index;
            let last_row = min(height, rows_per_chunk * (index + 1));
            offset_channel(&input_image.pixel_data, width, height, first_row, last_row, chunk, Channel::Red, &red_kernel);
            offset_channel(&input_image.pixel_data, width, height, first_row, last_row, chunk, Channel::Blue, &blue_kernel);
        });
    
    output_image
}

fn offset_channel(input_pixel_data: &[u8], width: usize, height: usize, first_row: usize, last_row: usize, output_chunk: &mut[u8], channel: Channel, kernel: &Kernel) {
    let left_radius = kernel.left_radius();
    let right_radius = kernel.right_radius();
    let stride = width * 3;
    let bottom = height - 1;
    
    for x in 0..width {
        let index_offset = x * 3 + channel as usize;
        
        let trailing = max(first_row, left_radius) - first_row;
        let first = first_row - min(first_row, left_radius);
        let last = first_row + right_radius;
        
        let mut window: VecDeque<u8> = iter::repeat(input_pixel_data[index_offset]).take(trailing)
            .chain((first..last).map(|y| input_pixel_data[min(bottom, y) * stride + index_offset]))
            .collect();
        
        for y in first_row..last_row {
            let value = kernel.apply(window.iter());
            output_chunk[(y - first_row) * stride + index_offset] = value;
            
            window.pop_front();
            window.push_back(input_pixel_data[min(bottom, y + right_radius) * stride + index_offset]);
        }
    }
}
