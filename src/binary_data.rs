use std::vec::Vec;

use anyhow::Result;

pub trait BinaryData<T> {
    fn from_bytes(bytes: &[u8]) -> Result<T>;
    fn to_bytes(&self) -> Vec<u8>;
}