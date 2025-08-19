use gtk::prelude::*;
use gtk::{ListBox, ListBoxRow, ScrolledWindow};
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
use std::collections::HashMap;


pub fn populate_list_view(
    list_box: &ListBox,
    entries: Vec<Box<dyn ClipboardEntry>>,
    width: i32,
) -> HashMap<ListBoxRow, Box<dyn ClipboardEntry>> {
    list_box.foreach(|child| {
        list_box.remove(child);
    });
    
    let mut row_to_entry_map = HashMap::new();
    
    for entry in entries {
        let row = entry.create_entry_row(width);
        list_box.add(&row);
        row_to_entry_map.insert(row, entry);
    }
    
    if !row_to_entry_map.is_empty() {
        list_box.select_row(list_box.row_at_index(0).as_ref());
    }
    
    row_to_entry_map
}

pub fn create_list_view(
    entries: Vec<Box<dyn ClipboardEntry>>,
    width: i32,
) -> (ScrolledWindow, ListBox, HashMap<ListBoxRow, Box<dyn ClipboardEntry>>) {
    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(width)
        .build();
    
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);
    
    let row_to_entry_map = populate_list_view(&list_box, entries, width);
    
    list_scrolled_window.add(&list_box);
    
    (list_scrolled_window, list_box, row_to_entry_map)
}
