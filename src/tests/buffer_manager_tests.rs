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
}
