#[derive(Clone, Debug)]
pub struct BlockId {
    pub file_name: String,
    pub block_num: u64,
}

impl BlockId {
    /// 新しいブロックIDを作成する
    pub fn new(file_name: &str, block_num: u64) -> Self {
        BlockId {
            file_name: file_name.to_string(),
            block_num: block_num,
        }
    }
}
