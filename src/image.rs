use std::io::{BufWriter, Read, Write};
use crate::error::Error;

pub const RED: usize = 0;
pub const BLUE: usize = 2;

pub struct Image {
    pub pixel_data: Vec<u8>,
    info: png::Info<'static>,
}

impl Image {
    pub fn load<R: Read>(stream: R) -> Result<Image, Error> {
        let decoder = png::Decoder::new(stream);
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

    pub fn save<W: Write>(&self, stream: W) -> Result<(), Error> {
        let encoder = png::Encoder::with_info(BufWriter::new(stream), self.info.clone())?;
        
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixel_data)?;
        
        return Ok(());
    }
    
    pub fn width(&self) -> usize {
        self.info.width as usize
    }
    
    pub fn height(&self) -> usize {
        self.info.height as usize
    }
}
