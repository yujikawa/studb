// mydb/src/storage/file_manager.rs
use super::block::BlockId;
use super::page::{Page, PAGE_SIZE};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct FileManager {
    db_file: File,
}

impl FileManager {
    pub fn new(db_path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(db_path)?;
        Ok(Self { db_file: file })
    }

    pub fn read_block(&mut self, block_id: &BlockId, page: &mut Page) -> std::io::Result<()> {
        let offset = (block_id.block_num() * PAGE_SIZE) as u64;
        self.db_file.seek(SeekFrom::Start(offset))?;
        self.db_file.read_exact(&mut page.data)?;
        Ok(())
    }

    pub fn write_block(&mut self, block_id: &BlockId, page: &Page) -> std::io::Result<()> {
        let offset = (block_id.block_num() * PAGE_SIZE) as u64;
        self.db_file.seek(SeekFrom::Start(offset))?;
        self.db_file.write_all(&page.data)?;
        self.db_file.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_file_manager_read_write() {
        let db_path = "test_db.studb";
        let _ = remove_file(db_path); // 既存のテスト用ファイルを削除

        let mut file_manager = FileManager::new(db_path).expect("Failed to create FileManager");

        // 書き込むデータを作成
        let block = BlockId::new(db_path, 0);
        let mut write_page = Page::new();
        let test_data = b"Hello, FileManager!";
        write_page.write_bytes(0, test_data);

        // ディスクに書き込み
        file_manager
            .write_block(&block, &write_page)
            .expect("Failed to write block");

        // 読み込み用の Page を作成
        let mut read_page = Page::new();
        file_manager
            .read_block(&block, &mut read_page)
            .expect("Failed to read block");

        // 読み込んだデータを確認
        let read_data = read_page.read_bytes(0, test_data.len());
        assert_eq!(read_data, test_data);
    }
}
