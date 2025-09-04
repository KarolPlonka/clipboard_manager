// use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use gtk::ListBoxRow;
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

#[derive(PartialEq)]
pub enum DetailsVisibility {
    Hidden,
    Normal,
    Big,
}

pub struct AppState {
    pub rows: RefCell<Vec<ListBoxRow>>,
    pub row_to_entry_map: RefCell<HashMap<ListBoxRow, Box<dyn ClipboardEntry>>>,
    pub details_visibility: RefCell<DetailsVisibility>,
    pub all_entries_loaded: RefCell<bool>,
    pub search_query: RefCell<Option<String>>,
    pub filtered_rows: RefCell<Option<Vec<ListBoxRow>>>,
    pub search_cache: RefCell<HashMap<String, Vec<ListBoxRow>>>,
    pub last_selected_row: RefCell<Option<ListBoxRow>>,
}

