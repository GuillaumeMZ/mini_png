use anyhow::{anyhow, Result};

use crate::{binary_data::BinaryData, comment_block::CommentBlock, header_block::HeaderBlock, data_block::DataBlock};

pub enum BlockContent {
    Header(HeaderBlock),
    Comment(CommentBlock),
    Data(DataBlock)
}

pub struct Block {
    pub block_length: u32,
    pub content: BlockContent
}

impl BinaryData<Block> for Block {
    fn from_bytes(bytes: &[u8]) -> Result<Block> {
        if bytes.len() <= 5 {
            return Err(anyhow!("Unable to parse a block: not enough bytes to store type + length."));
        }

        let block_type = bytes[0];
        let block_length = u32::from_be_bytes(bytes[1..=4].try_into().unwrap()); //safe unwrap because we have 4 bytes
        let content_bytes = bytes.get(5..5+block_length as usize).ok_or(anyhow!("Unable to parse a block: there is a mismatch between block length and the actual number of bytes."))?;
        
        if !block_type.is_ascii() {
            return Err(anyhow!("Unable to parse a block: its type is not a valid ASCII character (so it cannot be H, C or D)"));
        }

        let content = match block_type {
            b'H' => BlockContent::Header(HeaderBlock::from_bytes(content_bytes)?),
            b'C' => BlockContent::Comment(CommentBlock::from_bytes(content_bytes)?),
            b'D' => BlockContent::Data(DataBlock::from_bytes(content_bytes)?),
            _ => { return Err(anyhow!("Unable to parse a block: its type is not one of H, C or D.")); }
        };

        Ok(Block {
            block_length,
            content
        })
    }
}