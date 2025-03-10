#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockId {
    pub filename: String,
    pub block_num: usize,
}

impl BlockId {
    /// 新しいブロックIDを作成する
    pub fn new(filename: &str, block_num: usize) -> Self {
        BlockId {
            filename: filename.to_string(),
            block_num: block_num,
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn block_num(&self) -> usize {
        self.block_num
    }
}
