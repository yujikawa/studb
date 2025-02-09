const BLOCK_SIZE: usize = 4096;

struct Page {
    data: [u8; BLOCK_SIZE],
}

impl Page {
    /// 新しいページを作成する
    fn new() -> Self {
        Page {
            data: [0; BLOCK_SIZE],
        }
    }

    /// 指定したオフセットに整数を書き込む
    fn set_int(&mut self, offset: usize, value: i32) {
        let bytes = value.to_be_bytes();
        self.data[offset..offset + 4].copy_from_slice(&bytes);
    }

    /// 指定したオフセットから整数を読み取る
    fn get_int(&mut self, offset: usize) -> i32 {
        let bytes = &self.data[offset..offset + 4];
        i32::from_be_bytes(bytes.try_into().unwrap())
    }

    /// 指定したオフセットに文字列を書き込む
    fn set_string(&mut self, offset: usize, value: &str) {
        let bytes = value.as_bytes();
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    /// 指定したオフセットから文字列を読み取る(長さを指定する)
    fn get_string(&mut self, offset: usize, length: usize) -> String {
        let bytes = &self.data[offset..offset + length];
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

#[derive(Clone, Debug)]
struct BlockId {
    file_name: String,
    block_num: u64,
}

impl BlockId {
    /// 新しいブロックIDを作成する
    fn new(file_name: &str, block_num: u64) -> Self {
        BlockId {
            file_name: file_name.to_string(),
            block_num: block_num,
        }
    }
}

fn main() {
    let block = BlockId::new("testfile.db", 42);
    let mut page = Page::new();
    page.set_int(0, 12345);
    page.set_string(4, "Hello world");

    let read_value = page.get_int(0);
    let read_string = page.get_string(4, 12);
    println!("Block ID: {:?}", block);
    println!("Block ID: {:?}", read_value);
    println!("Block ID: {:?}", read_string);
    
}
