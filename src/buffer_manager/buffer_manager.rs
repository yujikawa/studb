use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::buffer_manager::buffer_frame::BufferFrame;
use crate::constants::MAX_BUFFER_SIZE;
use crate::file_manager::block_id::BlockId;
use crate::file_manager::file_manager::FileManager;
use crate::file_manager::page::Page;

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

    fn evicted_page(&self) -> Option(BlockId) {
        let mut pool = self.pool.lock().unwrap();

        let evicted_block = pool
            .iter()
            .find(|(_, frame)| frame.lock().unwrap().pin_count == 0)
            .map(|(block_id, _)| block_id.clone());

        if let Some(block_id) = &evicted_block {
            let frame = pool.remove(block_id).unwrap();
            let frame = frame.lock().unwrap();

            if frame.is_dirty {
                self.file_manager.write(block_id, &frame.page).unwrap();
            }
        }

        evicted_block
    }

    pub fn pin_page(&self, block_id: &BlockId) -> Arc<Mutex<BufferFrame>> {
        // Buffer poolの中身を探す
        let mut pool = self.pool.lock().unwrap();

        // 目的のデータがある場合は使用中に設定してそのBuffer frameを返すだけ
        if let Some(frame) = pool.get(block_id) {
            frame.lock.unwrap().pin();
            return Arc::clone(frame);
        }

        if pool.len() >= MAX_BUFFER_SIZE {
            if let Some(evicted_block) = self.evicted_page() {
                println!("Evicted page {:?}", evicted_block);
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "No pages available for eviction",
                ));
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::sync::Arc;

    #[test]
    fn test_buffer_manager() -> io::Result<()> {
        let file_manager = Arc::new(FileManager::new());
        let buffer_manager = BufferManager::new(file_manager.clone());

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

        // BufferManager にページをピンする
        let buffer_frame = buffer_manager.pin_page(&block);

        // Mutex のロックを取得
        let buffer_frame = buffer_frame.lock().unwrap();

        // テスト項目
        assert_eq!(buffer_frame.pin_count, 1, "pinカウントが一致しません");
        assert_eq!(buffer_frame.is_dirty, false, "is_dirty一致しません");
        assert_eq!(
            buffer_frame.page.get_int(0),
            int_value,
            "整数データが一致しません。"
        );
        assert_eq!(
            buffer_frame.page.get_string(4, string_value.len()),
            string_value,
            "文字列データが一致しません。"
        );

        // BufferManager にページをアンピンする
        let buffer_frame = buffer_manager.unpin_page(&block);

        // Mutex のロックを取得
        let buffer_frame = buffer_frame.lock().unwrap();
        assert_eq!(buffer_frame.pin_count, 0, "pinカウントが一致しません");

        Ok(())
    }

    #[test]
    fn test_eviction() -> io::Result<()> {
        // ページの置換テスト
        let file_manager = Arc::new(FileManager::new());
        let buffer_manager = BufferManager::new(file_manager.clone());

        let file_name = "testfile.studb";

        for i in 0..MAX_BUFFER_SIZE {
            let block = BlockId::new(file_name, i);
            let _ = buffer_manager.pin_page(&block)?;
        }

        let new_block = BlockId::new(file_name, MAX_BUFFER_SIZE);
        let result = buffer_manager.pin_page(&new_block);

        assert_eq!(result.is_ok());

        Ok(())
    }
}
