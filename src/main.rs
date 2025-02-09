use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

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

    /// ページを指定したファイルに書き込む
    fn write_to_file(&self, file_name: &str, block_num: u64) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)?;
        file.seek(SeekFrom::Start(block_num * BLOCK_SIZE as u64))?;
        file.write_all(&self.data);
        Ok(())
    }

    /// ファイルから指定したブロックをページに読み込む
    fn read_from_file(&mut self, file_name: &str, block_num: u64) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open(file_name)?;
        file.seek(SeekFrom::Start(block_num * BLOCK_SIZE as u64))?;
        file.read_exact(&mut self.data)?;
        Ok(())
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

fn main() -> io::Result<()> {
    let mut page = Page::new();

    // ページにデータを書き込み
    page.set_int(0, 12345);
    page.set_string(4, "Hello, Rust!");

    // ページをファイルに保存
    page.write_to_file("testfile.db", 0)?;

    // 別のページオブジェクトにファイルから読み込む
    let mut read_page = Page::new();
    read_page.read_from_file("testfile.db", 0)?;

    // データを読み取り
    let read_value = read_page.get_int(0);
    let read_string = read_page.get_string(4, 12);

    println!("Read Int: {}", read_value);
    println!("Read String: {}", read_string);

    Ok(())
}
