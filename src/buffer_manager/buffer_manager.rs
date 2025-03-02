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
        // 使用者が増えたらインクリメント
        self.pin_count += 1;
    }

    pub fn unpin(&mut self) {
        // 使用者が減ればデクリメント
        if self.pin_count > 0 {
            self.pin_count -= 1;
        }
    }

    pub fn is_pinned(&self) -> book {
        self.pin_count > 0
    }
}

pub struct BufferManager {
    // Buffer poolを定義
    pool: Mutex<HashMap<BlockId, Arc<Mutex<BufferFrame>>>>,
    file_manager: Arc<FileManager>,
}

impl BufferManager {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        BufferManager {
            pool: Mutex::new(HashMap::new()),
            file_manager,
        }
    }

    pub fn pin_page(&self, block_id: &BlockId) -> Arc<Mutex<BufferFrame>> {
        // Buffer poolの中身を探す
        let mut pool = self.pool.lock().unwrap();

        // 目的のデータがある場合は使用中に設定してそのBuffer frameを返すだけ
        if let Some(frame) = pool.get(block_id) {
            frame.lock.unwrap().pin();
            return Arc::clone(frame);
        }

        // 以下目的のデータがない場合
        // ディスクから読み出しをして内容をページに保存
        let mut page = Page::new();
        self.file_manager.read(block_id, &mut page).unwrap();

        // 取得したページをBuffer frame化、使用中、Frameをpoolに入れる
        let frame = Arc::new(Mutex::new(BufferFrame::new(page)));
        frame.lock().unwrap().pin();
        pool.insert(block_id.clone(), Arc::clone(&frame));

        frame
    }

    pub fn unpin_page(&self, block_id: &BlockId, is_dirty: bool) {
        // ピン留めを解除する
        let mut pool = self.pool.lock().unwrap();

        // 対象があればピン留めを解除
        if let Some(frame) = pool.get(block_id) {
            let mut frame = frame.lock().unwrap();
            frame.unpin();
            if is_dirty {
                frame.is_dirty = true;
            }

            // 参照がなく更新対象ならディスクに書き込み
            if frame.pin_count == 0 && frame.is_dirty {
                self.flush_page(block_id);
            }
        }
    }

    pub fn flush_page(&self, block_id: &BlockId) {
        // ディスクへの書き込み処理
        let mut pool = self.pool.lock().unwrap();
        if let Some(frame) = pool.get(block_id) {
            let mut frame = frame.lock().unwrap();
            if frame.is_dirty {
                self.file_manager.write(block_id, &frame.page).unwrap();
                frame.is_dirty = false;
            }
        }
    }
}
