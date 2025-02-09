
#[derive(Clone, Debug)]
struct BlockId {
    file_name: String,
    block_num: u64,
}

impl BlockId {
    fn new(file_name: &str, block_num: u64) -> Self {
        BlockId {
            file_name: file_name.to_string(),
            block_num: block_num,
        }
    }
}

fn main() {
    let block = BlockId::new("testfile.db", 42);
    println!("Block ID: {:?}", block);
}
