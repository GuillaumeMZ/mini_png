use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::binary_data::BinaryData;
use crate::block::{Block, BlockContent};
use crate::data_block::DataBlock;
use crate::header_block::{HeaderBlock, Pixel, PixelType};
use crate::comment_block::CommentBlock;

pub struct MiniPNG {
    header_block: HeaderBlock,
    comment_blocks: Vec<CommentBlock>,
    pixels: Vec<Pixel>,
}

impl MiniPNG {
    fn try_parse_block(bytes: &[u8]) -> Result<(Block, &[u8])> {
        let block = Block::from_bytes(bytes)?;

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
        
        loop {
            let (block, next_bytes) = MiniPNG::try_parse_block(bytes)?;

            match block.content {
                BlockContent::Comment(it) => comment_blocks.push(it),
                BlockContent::Data(it) => data_blocks.push(it),
                BlockContent::Header(it) => header_blocks.push(it)
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

        let data_bytes: Vec<u8> = data_blocks.iter()
                                             .map(|data_block| data_block.contents())
                                             .flatten()
                                             .collect();

        let pixel_type = header_block.get_pixel_type();
        //let pixel_size_in_bytes = pixel_type.size_in_bytes();
        //check that the number of pixels matches the specified dimensions of the image
        // if data_bytes.len() != header_block.get_image_width() as usize * header_block.get_image_height() as usize * pixel_size_in_bytes {
        //     return Err(anyhow!("Error detected after parsing the file: the file size does not match the number of pixels parsed."));
        // }

        let pixels = MiniPNG::process_pixels(pixel_type, data_bytes);

        Ok(MiniPNG {
            header_block,
            comment_blocks,
            pixels
        })
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
        self.comment_blocks.iter().map(|comment| {
            let CommentBlock(comment) = comment;
            comment.clone()
        }).collect()
    }

    pub fn get_pixel_at(&self, x: u32, y: u32) -> Option<Pixel> {
        let image_width = self.get_image_width();
        let image_height = self.get_image_height();

        if y >= image_width || x >= image_height {
            return None;
        }

        Some(self.pixels[(image_width * x + y) as usize])
    }
}