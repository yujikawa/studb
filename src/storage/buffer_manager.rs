use std::collections::HashMap;
use super::block::BlockId;
use super::file_manager::FileManager;
use super::page::Page;

const MAX_BUFFERS: usize = 3; // シンプルな制限

pub struct BufferManager {
    file_manager: FileManager,
    cache: HashMap<BlockId, Page>,
    pinned_blocks: HashMap<BlockId, usize>, // ピンカウント管理
}

impl BufferManager {
    pub fn new(file_manager: FileManager) -> Self {
        Self {
            file_manager,
            cache: HashMap::new(),
            pinned_blocks: HashMap::new(),
        }
    }

    pub fn pin_page(&mut self, block_id: &BlockId) -> std::io::Result<&Page> {
        if !self.cache.contains_key(block_id) {
            if self.cache.len() >= MAX_BUFFERS {
                self.evict_page();
            }
            let mut page = Page::new();
            self.file_manager.read_block(block_id, &mut page)?;
            self.cache.insert(block_id.clone(), page);
        }
        let count = self.pinned_blocks.entry(block_id.clone()).or_insert(0);
        *count += 1;
        Ok(self.cache.get(block_id).unwrap())
    }

    pub fn unpin_page(&mut self, block_id: &BlockId) {
        if let Some(count) = self.pinned_blocks.get_mut(block_id) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.pinned_blocks.remove(block_id);
            }
        }
    }

    pub fn is_pinned(&self, block_id: &BlockId) -> bool {
        self.pinned_blocks.get(block_id).map_or(false, |&count| count > 0)
    }

    fn evict_page(&mut self) {
        let mut evicted = None;
        for block_id in self.cache.keys() {
            if !self.is_pinned(block_id) {
                evicted = Some(block_id.clone());
                break;
            }
        }
        if let Some(block_id) = evicted {
            self.cache.remove(&block_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_buffer_manager_pin_unpin() {
        let db_path = "test_db.studb";
        let _ = remove_file(db_path);

        let file_manager = FileManager::new(db_path).expect("Failed to create FileManager");
        let mut buffer_manager = BufferManager::new(file_manager);

        let block = BlockId::new(db_path, 0);
        let _page = buffer_manager.pin_page(&block).expect("Failed to pin page");
        assert!(buffer_manager.is_pinned(&block));

        buffer_manager.unpin_page(&block);
        assert!(!buffer_manager.is_pinned(&block));
    }
}
