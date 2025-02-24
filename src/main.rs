mod constants;
mod file_manager;
use file_manager::block_id::BlockId;
use file_manager::file_manager::FileManager;
use file_manager::page::Page;

fn main() {
    println!("Hello studb");
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
