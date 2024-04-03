use std::vec::Vec;

pub trait BinaryData<T> {
    fn from_bytes(bytes: &[u8]) -> Result<T, ()>;
    fn to_bytes(&self) -> Result<Vec<u8>, ()>; //TODO: looks like it never fails. if so, remove result 
}