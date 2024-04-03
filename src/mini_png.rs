use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::binary_data::BinaryData;
use crate::block::{Block, BlockContent};
use crate::data_block::DataBlock;
use crate::header_block::{HeaderBlock, PixelType};
use crate::comment_block::CommentBlock;

pub struct MiniPNG {
    header_block: HeaderBlock,
    comment_blocks: Vec<CommentBlock>,
    data_blocks: Vec<DataBlock>, //one or more | TODO: only store the concatenation
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

        if data_blocks.len() == 0{
            return Err(anyhow!("Unable to parse the file: no data block has been found."))
        }

        //TODO: check that the number of pixels matches the specified dimensions of the image

        Ok(MiniPNG {
            header_block: header_blocks[0], //safe access since we checked the size earlier
            comment_blocks,
            data_blocks
        })
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

    }
}