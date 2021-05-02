
// データをファイルに永続化する構造体
pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file: File,

    // 採番するページIDを決めるカウンタ
    next_page_id: u64,
}

// メソッドを定義する
impl DiskManager {
    // コンストラクタ
    pub fn new(data_file: File) -> io::Result<Self> {
        // TODO
    }

    // ファイルパスを指定して開く
    pub fn open(data_file_path: impl AsRef<Path>)
            -> io::Result<Self> {
        // TODO
    }

    // 新しいページIDを採番する
    pub fn allocate_page(&mut self) -> PageId {
        // TODO
    }

    // ページのデータを読み出す
    pub fb read_page_data(&mut self, page_id: PageId,
            data: &mut [u8]) -> io::Result<()> {
        // TODO
    }

    // データをページに書き出す
    pub fn write_page_date(&mut self, page_id: PageId,
            data: &[u8]) -> io::Result<()> {
        // TODO
    }

    // TODO
}