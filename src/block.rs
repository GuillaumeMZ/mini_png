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
    fn from_bytes(bytes: &[u8]) -> Result<Block, ()> {
        if bytes.len() <= 5 {
            return Err(()); //not enough bytes
        }

        let block_type = bytes[0];
        let block_length = u32::from_ne_bytes(bytes[1..=4].try_into().unwrap());
        let content_bytes = bytes.get(5..5+block_length as usize).ok_or(())?;
        
        if !block_type.is_ascii() {
            return Err(());
        }

        let content = match block_type {
            b'H' => BlockContent::Header(HeaderBlock::from_bytes(content_bytes)?),
            b'C' => BlockContent::Comment(CommentBlock::from_bytes(content_bytes)?),
            b'D' => BlockContent::Data(DataBlock::from_bytes(content_bytes)?),
            _ => { return Err(()); }
        };

        Ok(Block {
            block_length,
            content
        })
    }

    fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        let mut result = Vec::new();
        
        let block_length_bytes = self.block_length.to_ne_bytes();
        let (content_type, content_bytes) = match &self.content {
            BlockContent::Header(header) => (b'H', header.to_bytes().unwrap()), //TODO: return an error
            BlockContent::Comment(comment) => (b'C', comment.to_bytes().unwrap()),
            BlockContent::Data(data) => (b'D', data.to_bytes().unwrap()),
        };

        result.insert(0, content_type);
        result.extend_from_slice(&block_length_bytes);
        result.extend_from_slice(&content_bytes);

        Ok(result)
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