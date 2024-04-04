use std::fmt;

use anyhow::{anyhow, Result};

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