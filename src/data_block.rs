use crate::binary_data::BinaryData;

pub struct DataBlock(Vec<u8>);

impl BinaryData<DataBlock> for DataBlock {
    fn from_bytes(bytes: &[u8]) -> Result<DataBlock, ()> {
        //TODO: what should we do when bytes.len() == 0 ?
        Ok(DataBlock(Vec::from(bytes)))
    }

    fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        Ok(self.0.clone())
    }
}