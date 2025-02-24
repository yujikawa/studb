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
