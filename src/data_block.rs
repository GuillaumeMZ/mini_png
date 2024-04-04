use anyhow::Result;

use crate::binary_data::BinaryData;

pub struct DataBlock(Vec<u8>);

impl BinaryData<DataBlock> for DataBlock {
    fn from_bytes(bytes: &[u8]) -> Result<DataBlock> {
        //TODO: what should we do when bytes.len() == 0 ?
        Ok(DataBlock(Vec::from(bytes)))
    }
}

impl DataBlock {
    pub fn contents(&self) -> Vec<u8> {
        let DataBlock(contents) = self;
        contents.clone()
    }
}