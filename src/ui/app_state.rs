// use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

#[derive(PartialEq)]
pub enum DetailsVisibility {
    Hidden,
    Normal,
    Big,
}

pub struct AppState {
    // pub entries: RefCell<Vec<Box<dyn ClipboardEntry>>>,
    pub row_to_entry_map: RefCell<HashMap<gtk::ListBoxRow, Box<dyn ClipboardEntry>>>,
    pub details_visibility: RefCell<DetailsVisibility>,
    pub all_entries_loaded: RefCell<bool>,
    // pub current_index: RefCell<usize>,
}

// impl AppState {
//     // pub fn max_index(&self) -> usize {
//     //     if self.entries.is_empty() { 0 } else { self.entries.len() - 1 }
//     // }
//     // pub fn max_index(&self) -> usize {
//     //
//     // }
// }
