use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*,　SeekFrom};
use std::path::Path;

// Linuxで使われているext4ファイルシステムのデフォルトブロックサイズ
pub const PAGE_SIZE: usize = 4096;

// ページIDを表す構造体：専用の構造体を定義して他の整数値と区別
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64)

impl PageId {
    pub fn to_u64(self) -> u64 {
        self.0
    }
}


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
    pub fn new(heap_file: File) -> io::Result<Self> {
        // ファイルサイズを取得
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    // ファイルパスを指定して開く
    pub fn open(data_file_path: impl AsRef<Path>)
            -> io::Result<Self> {
        let heap_file = OpenOptions::new()
                .read(true) //　ファイルの読み書き可能なモードに指定
                .write(true)
                .create(true) // ファイルがなかったら新規作成する
                // ファイルディスクリプタまたはI/Oエラー。
                // ?はI/0エラーの場合早期リターンする記法
                .open(heap_file_path)?;
        Self::new(heap_file)
    }

    // 新しいページIDを採番する
    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    // ページのデータを読み出す
    pub fb read_page_data(&mut self, page_id: PageId,
            data: &mut [u8]) -> io::Result<()> {
        // オフセットを計算
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // ページ先頭へシーク
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを読み出す（data引数に書き込み）
        self.heap_file.read_exact(data)
    }

    // データをページに書き出す
    pub fn write_page_date(&mut self, page_id: PageId,
            data: &[u8]) -> io::Result<()> {
        // オフセットを計算
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // ページ先頭へシーク
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを書き込む
        self.heap_file.write_all(data)
    }

    // TODO
}