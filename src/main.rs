mod constants;
mod file_manager;
use file_manager::block_id::BlockId;
use file_manager::file_manager::FileManager;
use file_manager::page::Page;

fn main() -> std::io::Result<()> {
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
    println!("{}", read_int);
    println!("{}", read_string);

    Ok(())
}
