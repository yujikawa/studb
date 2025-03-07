use crate::constants::BLOCK_SIZE;
#[derive(Debug)]
pub struct Page {
    pub data: [u8; BLOCK_SIZE],
}

impl Page {
    /// 新しいページを作成する
    pub fn new() -> Self {
        Page {
            data: [0; BLOCK_SIZE],
        }
    }

    /// 指定したオフセットに整数を書き込む
    pub fn set_int(&mut self, offset: usize, value: i32) {
        let bytes = value.to_be_bytes();
        self.data[offset..offset + 4].copy_from_slice(&bytes);
    }

    /// 指定したオフセットから整数を読み取る
    pub fn get_int(&mut self, offset: usize) -> i32 {
        let bytes = &self.data[offset..offset + 4];
        i32::from_be_bytes(bytes.try_into().unwrap())
    }

    /// 指定したオフセットに文字列を書き込む
    pub fn set_string(&mut self, offset: usize, value: &str) {
        let bytes = value.as_bytes();
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    /// 指定したオフセットから文字列を読み取る(長さを指定する)
    pub fn get_string(&mut self, offset: usize, length: usize) -> String {
        let bytes = &self.data[offset..offset + length];
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}
