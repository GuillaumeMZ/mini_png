use anyhow::{anyhow, Result};

pub struct CommentBlock(String);

impl TryFrom<&[u8]> for CommentBlock {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<CommentBlock> {
        let are_all_chars_representable = bytes.iter().all(|byte| *byte >= 32 && *byte <= 127);
        
        if !are_all_chars_representable {
            return Err(anyhow!("Unable to parse a comment: the text is not valid ASCII."));
        }
        
        Ok(CommentBlock(String::from_utf8(bytes.to_vec()).unwrap())) //safe unwrap because all bytes are valid ASCII
    }
}

impl CommentBlock {
    pub fn get_comment(&self) -> String {
        self.0.clone()
    }
}