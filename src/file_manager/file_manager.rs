use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

use crate::constants::BLOCK_SIZE;
use crate::file_manager::block_id::BlockId;
use crate::file_manager::page::Page;

pub struct FileManager {
    files: Mutex<HashMap<String, File>>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            files: Mutex::new(HashMap::new()),
        }
    }

    pub fn read(&self, block: &BlockId, page: &mut Page) -> io::Result<()> {
        let mut files = self.files.lock().unwrap();
        let file = files.entry(block.file_name.clone()).or_insert_with(|| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&block.file_name)
                .unwrap()
        });
        file.seek(SeekFrom::Start(block.block_num * BLOCK_SIZE as u64))?;
        file.read_exact(&mut page.data)?;

        Ok(())
    }

    pub fn write(&self, block: &BlockId, page: &Page) -> io::Result<()> {
        let mut files = self.files.lock().unwrap();
        let file = files.entry(block.file_name.clone()).or_insert_with(|| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&block.file_name)
                .unwrap()
        });
        file.seek(SeekFrom::Start(block.block_num * BLOCK_SIZE as u64))?;
        file.write_all(&page.data)?;

        Ok(())
    }

    pub fn append(&self, file_name: &str, page: &Page) -> io::Result<BlockId> {
        let mut files = self.files.lock().unwrap();
        let file = files.entry(file_name.to_string()).or_insert_with(|| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&file_name)
                .unwrap()
        });

        let end_pos = file.seek(SeekFrom::End(0))?;
        let new_block_num = end_pos / BLOCK_SIZE as u64;

        file.write_all(&page.data)?;

        Ok(BlockId::new(file_name, new_block_num))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_file_write_and_read() -> io::Result<()> {
        let file_manager = FileManager::new();
        let file_name = "testfile.studb";

        // ブロックとページの初期化
        let mut page = Page::new();
        let block = BlockId::new(file_name, 0);

        // データを書き込む
        let int_value = 12345;
        let string_value = "Hello, SimpleDB!";
        page.set_int(0, int_value);
        page.set_string(4, string_value);

        // ファイルに書き込む
        file_manager.write(&block, &page)?;

        // 読み込んで検証
        let mut read_page = Page::new();
        file_manager.read(&block, &mut read_page)?;

        let read_int = read_page.get_int(0);
        let read_string = read_page.get_string(4, string_value.len());

        assert_eq!(read_int, int_value, "整数データが一致しません。");
        assert_eq!(read_string, string_value, "文字列データが一致しません。");

        Ok(())
    }

    #[test]
    fn test_append_block() -> io::Result<()> {
        let file_manager = FileManager::new();
        let file_name = "testfile.studb";

        // 新しいページを作成してデータをセット
        let mut new_page = Page::new();
        new_page.set_int(0, 54321);

        // 新しいブロックを追加
        let new_block = file_manager.append(file_name, &new_page)?;

        // 読み込んで確認
        let mut verify_page = Page::new();
        file_manager.read(&new_block, &mut verify_page)?;

        let verify_int = verify_page.get_int(0);
        assert_eq!(verify_int, 54321, "追加ブロックのデータが一致しません。");

        Ok(())
    }
}
