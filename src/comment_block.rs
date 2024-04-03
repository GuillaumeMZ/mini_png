use anyhow::{anyhow, Result};

use crate::binary_data::BinaryData;

pub struct CommentBlock(String);

impl BinaryData<CommentBlock> for CommentBlock {
    fn from_bytes(bytes: &[u8]) -> Result<CommentBlock> {
        //TODO: what should we do when bytes.len() == 0 ?
        let are_all_chars_representable = bytes.iter().all(|byte| *byte >= 32 && *byte <= 127);
        
        if are_all_chars_representable {
            Ok(CommentBlock(String::from_utf8(bytes.to_vec()).unwrap())) //safe unwrap because all bytes are valid ASCII
        } else {
            Err(anyhow!("Unable to parse a comment: the text is not valid ASCII."))
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }
}