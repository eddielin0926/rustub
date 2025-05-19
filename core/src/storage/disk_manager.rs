use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::sync::Mutex;

use crate::constants::{DEFAULT_DB_PAGES, PAGE_SIZE};
use crate::errors::DiskError;
use crate::types::{PageId, PageIdRef};

use super::page::Page;

/// `DiskManager` handles low-level operations:
/// - Reading and writing pages from/to disk
/// - Page allocation
/// Note: it does not handle buffer management or transactional consistency.
pub struct DiskManager {
    db_file: Mutex<File>,
    page_table: Mutex<HashMap<PageId, usize>>,
    num_pages: Mutex<usize>,
    // TODO: Maintain a reusable free list for deleted pages.
    // free_pages: Mutex<Vec<usize>>,
}

impl DiskManager {
    /// Create or open the underlying database file.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        Ok(Self {
            db_file: Mutex::new(file),
            page_table: Mutex::new(HashMap::new()),
            num_pages: Mutex::new(DEFAULT_DB_PAGES),
        })
    }

    /// Reads a page from disk into the provided buffer.
    pub fn fetch_page(&self, page_id: PageIdRef, page: &mut Page) -> Result<(), DiskError> {
        let page_id = page_id.ok_or_else(|| DiskError::InvalidPageId)?;
        let offset = {
            let page_table = self.page_table.lock().unwrap();
            *page_table
                .get(&page_id)
                .ok_or_else(|| DiskError::PageNotFound)?
        };

        let file = self.db_file.lock().unwrap();
        file.read_at(page.as_mut_slice(), offset as u64)
            .map_err(DiskError::Io)?;
        Ok(())
    }

    /// Writes a page to disk. Allocates a new page slot if this is the first time the page is flushed.
    pub fn flush_page(&self, page_id: PageIdRef, page: &Page) -> Result<(), DiskError> {
        let page_id = page_id.ok_or_else(|| DiskError::InvalidPageId)?;

        let offset = {
            let mut table = self.page_table.lock().unwrap();
            *table.entry(page_id).or_insert_with(|| self.allocate_page())
        };

        let file = self.db_file.lock().unwrap();
        file.write_at(page.as_slice(), offset as u64)
            .map_err(DiskError::Io)?;
        Ok(())
    }

    /// Marks a page as deleted. Future implementation may support page reuse.
    pub fn delete_page(&self, _page_id: PageIdRef) -> Result<(), DiskError> {
        todo!("delete_page will support free page reuse");
    }

    /// Allocates the next available page slot and returns its byte offset.
    fn allocate_page(&self) -> usize {
        let mut np = self.num_pages.lock().unwrap();
        let offset = *np * PAGE_SIZE;
        *np += 1;
        offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use tempfile::NamedTempFile;

    #[test]
    fn test_flush_then_fetch_page() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        drop(file); // Let DiskManager take ownership

        let dm = DiskManager::new(&path).unwrap();
        let page_id = Some(1);

        let mut write_page = Page::new();
        write_page.as_mut_slice()[0..4].copy_from_slice(&123u32.to_le_bytes());
        dm.flush_page(page_id, &write_page).unwrap();

        let mut read_page = Page::new();
        dm.fetch_page(page_id, &mut read_page).unwrap();
        let read_val = u32::from_le_bytes(read_page.as_slice()[0..4].try_into().unwrap());

        assert_eq!(read_val, 123);
        debug_assert_eq!(read_page.as_slice()[4..], [0; PAGE_SIZE - 4]);

        remove_file(path).unwrap();
    }
}
