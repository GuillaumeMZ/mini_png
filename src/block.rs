use crate::{binary_data::BinaryData, comment_block::CommentBlock, header_block::HeaderBlock};

enum BlockContent {
    Header(HeaderBlock),
    Comment(CommentBlock),
    //Data()
}

struct Block {
    block_type: u8,
    block_length: u32,
    content: BlockContent
}

impl BinaryData<Block> for Block {
    fn from_bytes(bytes: &[u8]) -> Result<Block, ()> {
        if bytes.len() <= 5 {
            return Err(()); //not enough bytes
        }

        let block_type = bytes[0];
        let block_length = u32::from_ne_bytes(bytes[1..=4].try_into().unwrap());
        let content_bytes = &bytes[5..];
        
        if !block_type.is_ascii() {
            return Err(());
        }

        let block_type_char = block_type as char;
        let content = match block_type_char {
            'H' => BlockContent::Header(HeaderBlock::from_bytes(content_bytes).unwrap()), //TODO: safer unwraps ?
            'C' => BlockContent::Comment(CommentBlock::from_bytes(content_bytes).unwrap()),
            //'D' => B
            _ => { return Err(()); }
        };

        Ok(Block {
            block_type,
            block_length,
            content
        })
    }

    fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        let mut result = Vec::new();
        
        let block_length_bytes = self.block_length.to_ne_bytes();
        let content_bytes = match &self.content {
            BlockContent::Header(header) => header.to_bytes().unwrap(), //TODO: return an error
            BlockContent::Comment(comment) => comment.to_bytes().unwrap(),
        };

        result.insert(0, self.block_type);
        result.extend_from_slice(&block_length_bytes);
        result.extend_from_slice(&content_bytes);

        Ok(result)
    }
}