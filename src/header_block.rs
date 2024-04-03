use crate::binary_data::BinaryData;

pub struct HeaderBlock {
    image_width: u32, //must be greater than 0
    image_height: u32, //ditto
    pixel_type: u8
}

impl BinaryData<HeaderBlock> for HeaderBlock {
    fn from_bytes(bytes: &[u8]) -> Result<HeaderBlock, ()> {
        if bytes.len() != 9 {
            Err(()) //TODO: clearer error type
        } else {
            //these will never fail because we know that bytes.len() == 9
            let first_four_bytes: [u8; 4] = bytes[0..=3].try_into().unwrap();
            let next_four_bytes: [u8; 4] = bytes[4..=8].try_into().unwrap();
            let last_byte = bytes[bytes.len() - 1];
            
            let image_width = u32::from_ne_bytes(first_four_bytes); //TODO: endianess
            let image_height = u32::from_ne_bytes(next_four_bytes);

            if image_width == 0 || image_height == 0 {
                return Err(()); //TODO: clearer error type
            }

            if last_byte > 3 { //change according to the supported pixel formats
                return Err(()); //TODO: clearer error type
            }

            Ok(HeaderBlock {
                image_width,
                image_height,
                pixel_type: last_byte
            })
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        let mut result = Vec::with_capacity(9);

        let image_width_as_bytes = self.image_width.to_ne_bytes();
        let image_height_as_bytes = self.image_height.to_ne_bytes();

        for i in 0usize..4 {
            result.insert(i, image_width_as_bytes[i]);
            result.insert(i + 4, image_height_as_bytes[i]);
        }

        result.insert(8, self.pixel_type);

        Ok(result)
    }
}