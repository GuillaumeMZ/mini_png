use crate::binary_data::BinaryData;

pub struct CommentBlock(String);

impl BinaryData<CommentBlock> for CommentBlock {
    fn from_bytes(bytes: &[u8]) -> Result<CommentBlock, ()> {
        //TODO: what should we do when bytes.len() == 0 ?
        let are_all_chars_representable = bytes.iter().all(|byte| *byte >= 33 && *byte <= 126);
        
        match are_all_chars_representable {
            true => Ok(CommentBlock(String::from_utf8(bytes.to_vec()).unwrap())), //safe unwrap because all bytes are valid ASCII
            false => Err(()) //TODO: explicit error
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        Ok(self.0.clone().into_bytes())
    }
}