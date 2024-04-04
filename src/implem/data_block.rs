use anyhow::Result;

pub struct DataBlock(Vec<u8>);

impl TryFrom<&[u8]> for DataBlock {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<DataBlock> {
        Ok(DataBlock(Vec::from(bytes)))
    }
}

impl DataBlock {
    pub fn get_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}