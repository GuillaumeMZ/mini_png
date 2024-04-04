use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{anyhow, Result};

use super::block::{Block, BlockContent};
use super::data_block::DataBlock;
use super::header_block::HeaderBlock;
use super::comment_block::CommentBlock;
use super::palette_block::PaletteBlock;
use super::pixel::{Pixel, PixelType};

pub struct MiniPNG {
    header_block: HeaderBlock,
    comment_blocks: Vec<CommentBlock>,
    palette_block: Option<PaletteBlock>,
    pixels: Vec<Pixel>,
}

impl MiniPNG {
    fn try_parse_block(bytes: &[u8]) -> Result<(Block, &[u8])> {
        let block = Block::try_from(bytes)?;

        let remaining_bytes = &bytes[5 + block.block_length as usize..]; //safe slicing (TODO: why ?)
        Ok((block, remaining_bytes))
    }

    pub fn from_file(file_path: &Path) -> Result<MiniPNG> {
        let mut bytes = Vec::<u8>::new();
        
        {
            let file = File::open(file_path)?;
            let mut reader = BufReader::new(file);
            
            reader.read_to_end(&mut bytes)?;
        }

        //TODO: check if there are enough bytes to store the magic string
        
        //check magic
        let magic_bytes = &bytes.as_slice()[0..=7];
        if magic_bytes != [b'M', b'i', b'n', b'i', b'-', b'P', b'N', b'G'] {
            return Err(anyhow!("This file is not a valid MiniPNG file (magic mismatch)."));
        }
        
        let mut bytes = &bytes.as_slice()[8..];
        
        //parse blocks
        //TODO: refactor as a function
        let mut header_blocks = Vec::<HeaderBlock>::new();
        let mut comment_blocks = Vec::<CommentBlock>::new();
        let mut data_blocks = Vec::<DataBlock>::new();
        let mut palette_blocks = Vec::<PaletteBlock>::new();

        loop {
            let (block, next_bytes) = MiniPNG::try_parse_block(bytes)?;

            match block.content {
                BlockContent::Comment(it) => comment_blocks.push(it),
                BlockContent::Data(it) => data_blocks.push(it),
                BlockContent::Header(it) => header_blocks.push(it),
                BlockContent::Palette(it) => palette_blocks.push(it),
            }

            if next_bytes.len() == 0 {
                break;
            }

            bytes = next_bytes;
        }

        let headers_count = header_blocks.len();
        if headers_count != 1 {
            return Err(anyhow!("Unable to parse the file: 1 header is expected, but {} were found.", headers_count));
        }

        let header_block = header_blocks[0]; //safe access since we checked the size earlier

        if data_blocks.len() == 0{
            return Err(anyhow!("Unable to parse the file: no data block has been found."))
        }

        if palette_blocks.len() >= 2 {
            return Err(anyhow!("Unable to parse the file: there cannot be more than one palette block, but {} were found.", palette_blocks.len()));
        }

        let data_bytes: Vec<u8> = data_blocks.iter()
                                             .map(|data_block| data_block.get_bytes())
                                             .flatten()
                                             .collect();

        let pixel_type = header_block.get_pixel_type();
        //check that the number of pixels matches the specified dimensions of the image
        if !MiniPNG::data_size_matches_image_size(header_block.get_image_width(), header_block.get_image_height(), data_bytes.len(), pixel_type) {
            return Err(anyhow!("Error detected after parsing the file: the file size does not match the number of pixels parsed."));
        }

        //TODO: do something if header says its a palette image but no palette block is found
        //TODO: check all pixels exist in the palette

        let pixels = MiniPNG::process_pixels(pixel_type, data_bytes);

        Ok(MiniPNG {
            header_block,
            comment_blocks,
            palette_block: if palette_blocks.len() == 1 { Some(palette_blocks[0].clone()) } else { None },
            pixels
        })
    }

    fn data_size_matches_image_size(image_width: u32, image_height: u32, bytes_count: usize, pixel_type: PixelType) -> bool {
        match pixel_type {
            PixelType::BlackAndWhite => ((image_width * image_height) as f32 / 8f32).ceil() as usize == bytes_count,
            PixelType::GrayLevels | PixelType::Palette => (image_width * image_height) as usize == bytes_count,
            PixelType::TwentyFourBitsColors => (image_width * image_height * 3) as usize == bytes_count
        }
    }

    fn process_pixels(pixel_type: PixelType, pixels_bytes: Vec<u8>) -> Vec<Pixel> {
        let mut result = Vec::new();

        match pixel_type {
            PixelType::BlackAndWhite => {
                for pixel_byte in pixels_bytes {
                    for i in (0..=7).rev() {
                        if (pixel_byte >> i) & 1 == 1 {
                            result.push(Pixel::White);
                        } else {
                            result.push(Pixel::Black);
                        }
                    }
                }
            },
            PixelType::GrayLevels => {
                for pixel_byte in pixels_bytes {
                    result.push(Pixel::Gray(pixel_byte));
                }
            },
            PixelType::Palette => {
                for pixel_byte in pixels_bytes {
                    result.push(Pixel::Palette(pixel_byte));
                }
            },
            PixelType::TwentyFourBitsColors => {
                for rgb in pixels_bytes.chunks(3) {
                    result.push(Pixel::TwentyFourBitsColors(rgb[0], rgb[1], rgb[2]));
                }
            }
        }

        result
    }

    pub fn get_image_width(&self) -> u32 {
        self.header_block.get_image_width()
    }

    pub fn get_image_height(&self) -> u32 {
        self.header_block.get_image_height()
    }

    pub fn get_pixel_type(&self) -> PixelType {
        self.header_block.get_pixel_type()
    }

    pub fn get_comments(&self) -> Vec<String> {
        self.comment_blocks.iter()
                           .map(|comment| comment.get_comment())
                           .collect()
    }

    pub fn get_pixel_at(&self, x: u32, y: u32) -> Option<Pixel> {
        let image_width = self.get_image_width();
        let image_height = self.get_image_height();

        if y >= image_width || x >= image_height {
            return None;
        }

        Some(self.pixels[(image_width * x + y) as usize])
    }

    pub fn get_palette(&self) -> Option<PaletteBlock> {
        self.palette_block.clone()
    }
}