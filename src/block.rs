struct Block<'a> {
    block_type: u8,
    block_length: u32,
    content: &'a [u8]
}

impl<'a> Block<'a> {

}