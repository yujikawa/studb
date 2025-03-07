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

    pub fn is_pinned(&self) -> bool {
        self.pin_count > 0
    }
}
