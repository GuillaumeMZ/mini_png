use anyhow::{anyhow, Result};

type PaletteEntry = (u8, u8, u8);

#[derive(Clone)]
pub struct PaletteBlock {
    entries: Vec<PaletteEntry>
}

impl TryFrom<&[u8]> for PaletteBlock {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<PaletteBlock> {
        if bytes.len() % 3 != 0 { //no need to check for 0 since it's done in Block::try_from
            return Err(anyhow!("Unable to parse a palette block: there should be 3n bytes, but {} is not a multiple of 3.", bytes.len()));
        }

        Ok(PaletteBlock {
            entries: bytes.chunks(3)
                          .map(|chunk| (chunk[0], chunk[1], chunk[2])) //safe access since we checked that there are 3n bytes
                          .collect()
        })
    }
}

impl PaletteBlock {
    pub fn entries(&self) -> Vec<PaletteEntry> {
        self.entries.clone()
    }
}
