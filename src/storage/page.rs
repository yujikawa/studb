pub const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: [0; PAGE_SIZE],
        }
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        let end = offset + bytes.len();
        if end <= PAGE_SIZE {
            self.data[offset..end].copy_from_slice(bytes);
        }
    }

    pub fn read_bytes(&self, offset: usize, length: usize) -> &[u8] {
        &self.data[offset..offset + length]
    }
}
