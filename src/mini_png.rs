use crate::header_block::HeaderBlock;
use crate::comment_block::CommentBlock;

struct MiniPNG {
    header_block: HeaderBlock,
    comment_blocks: Vec<CommentBlock>,
    //data_blocks: Vec<> //one or more
}

impl MiniPNG {
    fn from_file(file_path: &str) -> Result<MiniPNG, ()> {
        todo!()
        //check magic
        //parse header block (fail if it does not exist)
        //parse comment blocks (if any)
        //parse data blocks (at least one, fail if 0)
    }
}