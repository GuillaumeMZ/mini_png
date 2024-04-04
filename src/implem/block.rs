use anyhow::{anyhow, Result};

use super::{comment_block::CommentBlock, data_block::DataBlock, header_block::HeaderBlock, palette_block::PaletteBlock};

pub enum BlockContent {
    Header(HeaderBlock),
    Comment(CommentBlock),
    Data(DataBlock),
    Palette(PaletteBlock)
}

pub struct Block {
    pub block_length: u32,
    pub content: BlockContent
}

impl TryFrom<&[u8]> for Block {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Block> {
        if bytes.len() <= 5 {
            return Err(anyhow!("Unable to parse a block: not enough bytes to store type + length."));
        }

        let block_type = bytes[0];
        let block_length = u32::from_be_bytes(bytes[1..=4].try_into().unwrap()); //safe unwrap because we have 4 bytes
        
        if block_length == 0 {
            return Err(anyhow!("Unable to parse a block: according to its metadata, its length is 0."));
        }

        let content_bytes = bytes.get(5..5+block_length as usize).ok_or(anyhow!("Unable to parse a block: there is a mismatch between block length and the actual number of bytes."))?;
        
        if !block_type.is_ascii() {
            return Err(anyhow!("Unable to parse a block: its type is not a valid ASCII character (so it cannot be H, C or D)"));
        }

        let content = match block_type {
            b'H' => BlockContent::Header(HeaderBlock::try_from(content_bytes)?),
            b'C' => BlockContent::Comment(CommentBlock::try_from(content_bytes)?),
            b'D' => BlockContent::Data(DataBlock::try_from(content_bytes)?),
            b'P' => BlockContent::Palette(PaletteBlock::try_from(content_bytes)?),
            _ => { return Err(anyhow!("Unable to parse a block: its type is not one of H, C or D.")); }
        };

        Ok(Block {
            block_length,
            content
        })
    }
}