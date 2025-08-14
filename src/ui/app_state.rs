// use std::rc::Rc;
use std::cell::RefCell;
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

pub struct AppState {
    pub entries: RefCell<Vec<Box<dyn ClipboardEntry>>>,
    pub details_visible: RefCell<bool>,
    pub current_index: RefCell<usize>,
}

impl AppState {
    // pub fn max_index(&self) -> usize {
    //     if self.entries.is_empty() { 0 } else { self.entries.len() - 1 }
    // }
    pub fn max_index(&self) -> usize {
        self.entries.borrow().len().saturating_sub(1)
    }
}
