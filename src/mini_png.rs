use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::binary_data::BinaryData;
use crate::block::{Block, BlockContent, BlockType};
use crate::data_block::DataBlock;
use crate::header_block::{HeaderBlock, PixelType};
use crate::comment_block::CommentBlock;

pub struct MiniPNG {
    header_block: HeaderBlock,
    comment_blocks: Vec<CommentBlock>,
    data_blocks: Vec<DataBlock>, //one or more
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
        
        //parse header block (fail if it does not exist)
        let new_bytes = &bytes.as_slice()[8..];
        let (header_block, mut bytes) = MiniPNG::try_parse_block(new_bytes)?;
        if header_block.get_type() != BlockType::Header {
            return Err(anyhow!("No header block found."));
        }

        let header_block = match header_block.content {
            BlockContent::Header(header) => header,
            _ => unreachable!()
        };
        
        //parse comment blocks (if any)
        //TODO: refactor as a function
        let mut comment_blocks = Vec::<CommentBlock>::new();
        
        loop {
            let comment_parse_result = MiniPNG::try_parse_block(bytes);
            
            if comment_parse_result.is_err() {
                break;
            }

            let (block, next_bytes) = comment_parse_result.unwrap(); //safe unwrap
            match block.content {
                BlockContent::Comment(it) => { 
                    bytes = next_bytes;
                    comment_blocks.push(it); 
                },
                _ => { 
                    break; 
                }
            }

        }

        //parse data blocks (at least one, fail if 0)
        //TODO: refactor as a function
        let mut data_blocks = Vec::<DataBlock>::new();

        let (first_data_block, mut bytes) = MiniPNG::try_parse_block(bytes)?;
        
        match first_data_block.content {
            BlockContent::Data(data) => { data_blocks.push(data); },
            _ => { return Err(anyhow!("There should be at least one data block.")); }
        }

        loop {
            if bytes.len() == 0 {
                break;
                //EOF
            }

            let data_parse_result = MiniPNG::try_parse_block(bytes);

            if data_parse_result.is_err() {
                return Err(anyhow!("Unable to parse a block")); //TODO: use the parent error
            }

            let (block, next_bytes) = data_parse_result.unwrap(); //safe unwrap
            match block.content {
                BlockContent::Data(it) => {
                    bytes = next_bytes;
                    data_blocks.push(it);
                },
                _ => {
                    return Err(anyhow!("Unexpected block type in the data blocks section."));
                }
            }
        }

        Ok(MiniPNG {
            header_block,
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
}