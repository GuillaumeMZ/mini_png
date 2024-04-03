use anyhow::{anyhow, Result};

use crate::{binary_data::BinaryData, comment_block::CommentBlock, header_block::HeaderBlock, data_block::DataBlock};

pub enum BlockContent {
    Header(HeaderBlock),
    Comment(CommentBlock),
    Data(DataBlock)
}

#[derive(PartialEq)]
pub enum BlockType {
    Header,
    Comment,
    Data
}

pub struct Block {
    pub block_length: u32,
    pub content: BlockContent
}

impl BinaryData<Block> for Block {
    fn from_bytes(bytes: &[u8]) -> Result<Block> {
        if bytes.len() <= 5 {
            return Err(anyhow!("Unable to parse a block: not enough bytes to store type + length.")); //not enough bytes
        }

        let block_type = bytes[0];
        let block_length = u32::from_ne_bytes(bytes[1..=4].try_into().unwrap()); //safe unwrap because we have 4 bytes
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

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        let block_length_bytes = self.block_length.to_ne_bytes();
        let (content_type, content_bytes) = match &self.content {
            BlockContent::Header(header) => (b'H', header.to_bytes()),
            BlockContent::Comment(comment) => (b'C', comment.to_bytes()),
            BlockContent::Data(data) => (b'D', data.to_bytes()),
        };

        result.insert(0, content_type);
        result.extend_from_slice(&block_length_bytes);
        result.extend_from_slice(&content_bytes);

        result
    }
}

impl Block {
    pub fn get_type(&self) -> BlockType {
        match self.content {
            BlockContent::Header(_) => BlockType::Header,
            BlockContent::Comment(_) => BlockType::Comment,
            BlockContent::Data(_) => BlockType::Data
        }
    }
}