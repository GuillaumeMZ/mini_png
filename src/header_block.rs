use core::fmt;

use anyhow::{anyhow, Result};

use crate::binary_data::BinaryData;

#[derive(Clone, Copy)]
pub enum PixelType {
    BlackAndWhite,
    GrayLevels,
    Palette,
    TwentyFourBitsColors
}

impl TryFrom<u8> for PixelType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            return Err(());
        }

        return Ok(match value {
            0 => PixelType::BlackAndWhite,
            1 => PixelType::GrayLevels,
            2 => PixelType::Palette,
            3 => PixelType::TwentyFourBitsColors,
            _ => unreachable!()
        });
    }
}

impl From<PixelType> for u8 {
    fn from(value: PixelType) -> Self {
        match value {
            PixelType::BlackAndWhite => 0,
            PixelType::GrayLevels => 1,
            PixelType::Palette => 2,
            PixelType::TwentyFourBitsColors => 3
        }
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
            Err(anyhow!("Unable to parse a header block: not enough bytes to store width + height + pixel type."))
        } else {
            //these will never fail because we know that bytes.len() == 9
            let first_four_bytes: [u8; 4] = bytes[0..=3].try_into().unwrap();
            let next_four_bytes: [u8; 4] = bytes[4..=7].try_into().unwrap();
            let last_byte = bytes[8];
            
            let image_width = u32::from_ne_bytes(first_four_bytes); //TODO: endianess
            let image_height = u32::from_ne_bytes(next_four_bytes);

            if image_width == 0 || image_height == 0 {
                return Err(anyhow!("Unable to parse a header block: one (or both) of the image's dimension is (are) 0."));
            }

            if last_byte > 3 { //change according to the supported pixel formats
                return Err(anyhow!("Unable to parse a header block: {} is not a valid pixel format type.", last_byte));
            }

            Ok(HeaderBlock {
                image_width,
                image_height,
                pixel_type: last_byte.try_into().unwrap()
            })
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(9);

        let image_width_as_bytes = self.image_width.to_ne_bytes();
        let image_height_as_bytes = self.image_height.to_ne_bytes();

        for i in 0usize..4 {
            result[i] = image_width_as_bytes[i];
            result[i + 4] = image_height_as_bytes[i];
        }

        result[8] = self.pixel_type.into();

        result
    }
}