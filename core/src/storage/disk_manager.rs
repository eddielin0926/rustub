use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::constants::{DEFAULT_DB_PAGES, PAGE_SIZE};
use crate::errors::DiskError;
use crate::types::{PageId, PageIdRef};

use super::page::Page;

pub struct DiskManager {
    db_file: Arc<Mutex<File>>,
    page_table: Mutex<HashMap<PageId, usize>>,
    num_pages: Mutex<usize>,
}

impl DiskManager {
    pub fn new<P: AsRef<Path>>(db_file_path: P) -> io::Result<Self> {
        let db_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(db_file_path)?;
        Ok(Self {
            db_file: Arc::new(Mutex::new(db_file)),
            page_table: Mutex::new(HashMap::new()),
            num_pages: Mutex::new(DEFAULT_DB_PAGES),
        })
    }

    pub fn read_page(&self, page_id: PageIdRef, page: &mut Page) -> Result<(), DiskError> {
        let page_id = page_id.ok_or(DiskError::InvalidPageId)?;
        let page_table = self.page_table.lock().unwrap();
        let offset = *page_table.get(&page_id).ok_or(DiskError::PageNotFound)?;

        let file = self.db_file.lock().unwrap();
        file.read_at(page.as_mut_slice(), offset as u64)?;
        Ok(())
    }

    pub fn write_page(&self, page_id: PageIdRef, page: &Page) -> Result<(), DiskError> {
        let page_id = page_id.ok_or(DiskError::InvalidPageId)?;

        let offset = {
            let mut page_table = self.page_table.lock().unwrap();
            match page_table.get(&page_id) {
                Some(&offset) => offset,
                None => {
                    let offset = self.allocate_page();
                    page_table.insert(page_id, offset);
                    offset
                }
            }
        };

        let file = self.db_file.lock().unwrap();
        file.write_at(page.as_slice(), offset as u64)?;
        Ok(())
    }

    fn allocate_page(&self) -> usize {
        let mut pages = self.num_pages.lock().unwrap();
        let offset = *pages * PAGE_SIZE;
        *pages += 1;
        offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_then_read_page() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        drop(file); // close tempfile to let DiskManager open it

        let dm = DiskManager::new(&path).unwrap();
        let page_id = Some(1);

        // Construct dummy page
        let mut write_page = Page::new();
        write_page.as_mut_slice()[0..4].copy_from_slice(&123u32.to_le_bytes());

        dm.write_page(page_id, &write_page).unwrap();

        let mut read_page = Page::new();
        dm.read_page(page_id, &mut read_page).unwrap();

        let read_val = u32::from_le_bytes(read_page.as_slice()[0..4].try_into().unwrap());
        assert_eq!(read_val, 123);

        remove_file(path).unwrap();
    }
}
