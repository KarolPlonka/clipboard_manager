use gtk::{prelude::*, ApplicationWindow, Inhibit, ListBox};

use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

use crate::ui::AppState;

fn get_current_entry<'a>(
    list_box: &ListBox,
    app_state: &'a AppState,
) -> Option<std::cell::Ref<'a, Box<dyn ClipboardEntry>>> {
    let selected_row = list_box.selected_row()?;
    let map = app_state.row_to_entry_map.borrow();
    
    if map.contains_key(&selected_row) {
        Some(std::cell::Ref::map(map, |m| m.get(&selected_row).unwrap()))
    } else {
        None
    }
}

pub fn handle_copy_and_close(
    window: &ApplicationWindow,
    list_box: &ListBox,
    app_state: &AppState,
    copy_path: bool,
) -> Inhibit {
    if let Some(entry) = get_current_entry(list_box, app_state) {
        if let Err(e) = entry.copy_to_clipboard(copy_path) {
            eprintln!("Error copying to clipboard: {}", e);
        }
    }
    window.close();
    Inhibit(true)
}

pub fn handle_open_in_external_app(
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    if let Some(entry) = get_current_entry(list_box, app_state) {
        if let Err(e) = entry.open_in_external_app() {
            eprintln!("Error opening in external app: {}", e);
        }
    }
    Inhibit(true)
}