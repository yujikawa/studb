use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::file_manager::block_id::BlockId;
use crate::file_manager::file_manager::FileManager;
use crate::file_manager::page::Page;

#[derive(Debug)]
pub struct BufferFrame {
    pub page: Page,
    pub pin_count: usize,
    pub is_dirty: bool,
}

impl BufferFrame {
    pub fn new(page: Page) -> Self {
        BufferFrame {
            page,
            pin_count: 0,
            is_dirty: false,
        }
    }

    pub fn pin(&mut self) {
        self.pin_count += 1;
    }

    pub fn unpin(&mut self) {
        if self.pin_count > 0 {
            self.pin_count -= 1;
        }
    }

    pub fn is_pinned(&self) -> book {
        self.pin_count > 0
    }
}

pub struct BufferPool {
    pool: Mutex<HashMap<BlockId, Arc<Mutex<BufferFrame>>>>,
    file_manager: Arc<FileManager>,
}

impl BufferPool {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        BufferPool {
            pool: Mutex::new(HashMap::new()),
            file_manager,
        }
    }

    pub fn pin_page(&self, block_id: &BlockId) -> Arc<Mutex<BufferFrame>> {
        let mut pool = self.pool.lock().unwrap();

        if let Some(frame) = pool.get(block_id) {
            frame.lock.unwrap().pin();
            return Arc::clone(frame);
        }

        let mut page = Page::new();
        self.file_manager.read(block_id, &mut page).unwrap();

        let frame = Arc::new(Mutex::new(BufferFrame::new(page)));
        frame.lock().unwrap().pin();
        pool.insert(block_id.clone(), Arc::clone(&frame));

        frame
    }

    pub fn unpin_page(&self, block_id: &BlockId, is_dirty: bool) {
        let mut pool = self.pool.lock().unwrap();

        if let Some(frame) = pool.get(block_id) {
            let mut frame = frame.lock().unwrap();
            frame.unpin();
            if is_dirty {
                frame.is_dirty = true;
            }
        }
    }
}
