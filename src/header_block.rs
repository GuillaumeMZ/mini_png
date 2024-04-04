use core::fmt;

use anyhow::{anyhow, Result};

use crate::binary_data::BinaryData;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pixel {
    Black,
    White,
    Gray(u8),
    Palette(u8),
    TwentyFourBitsColors(u8, u8, u8)
}

#[derive(Clone, Copy, PartialEq)]
pub enum PixelType {
    BlackAndWhite,
    GrayLevels,
    Palette,
    TwentyFourBitsColors
}

impl TryFrom<u8> for PixelType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => PixelType::BlackAndWhite,
            1 => PixelType::GrayLevels,
            2 => PixelType::Palette,
            3 => PixelType::TwentyFourBitsColors,
            _ => return Err(anyhow!("Unable to parse the pixel type: {} is not a valid pixel type.", value))
        })
    }
}

impl fmt::Display for PixelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelType::BlackAndWhite => write!(f, "0 (Black and white)"),
            PixelType::GrayLevels => write!(f, "1 (Gray levels)"),
            PixelType::Palette => write!(f, "2 (Palette)"),
            PixelType::TwentyFourBitsColors => write!(f, "3 (24 bits colors)")
        }
    }
}

#[derive(Clone, Copy)]
pub struct HeaderBlock {
    image_width: u32, //must be greater than 0
    image_height: u32, //ditto
    pixel_type: PixelType
}

impl HeaderBlock {
    pub fn get_image_width(&self) -> u32 {
        self.image_width
    }

    pub fn get_image_height(&self) -> u32 {
        self.image_height
    }

    pub fn get_pixel_type(&self) -> PixelType {
        self.pixel_type
    }
}

impl BinaryData<HeaderBlock> for HeaderBlock {
    fn from_bytes(bytes: &[u8]) -> Result<HeaderBlock> {
        if bytes.len() != 9 {
            return Err(anyhow!("Unable to parse a header block: not enough bytes to store width + height + pixel type."));
        }
            
        //these will never fail because we know that bytes.len() == 9
        let first_four_bytes: [u8; 4] = bytes[0..=3].try_into().unwrap();
        let next_four_bytes: [u8; 4] = bytes[4..=7].try_into().unwrap();
        let last_byte = bytes[8];
        
        let image_width = u32::from_be_bytes(first_four_bytes);
        let image_height = u32::from_be_bytes(next_four_bytes);

        if image_width == 0 || image_height == 0 {
            return Err(anyhow!("Unable to parse a header block: one (or both) of the image's dimension is (are) 0."));
        }

        if last_byte > 3 { //change according to the supported pixel formats
            return Err(anyhow!("Unable to parse a header block: {} is not a valid pixel format type.", last_byte));
        }

        Ok(HeaderBlock {
            image_width,
            image_height,
            pixel_type: last_byte.try_into().unwrap() //safe unwrap because we checked the value earlier
        })
    }
}