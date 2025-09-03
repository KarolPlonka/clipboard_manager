use gtk::prelude::*;
use gtk::{ListBox, ListBoxRow, ScrolledWindow};
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
use std::collections::HashMap;


pub fn append_to_list_view(
    list_box: &ListBox,
    entries: Vec<Box<dyn ClipboardEntry>>,
    width: i32,
) -> (Vec<ListBoxRow>, HashMap<ListBoxRow, Box<dyn ClipboardEntry>>) {
    // list_box.foreach(|child| {
    //     list_box.remove(child);
    // });
    
    let rows: Vec<ListBoxRow> = entries
        .iter()
        .map(|entry| entry.create_entry_row(width))
        .collect();
    
    let row_to_entry_map: HashMap<ListBoxRow, Box<dyn ClipboardEntry>> = rows
        .iter()
        .cloned()
        .zip(entries.into_iter())
        .collect();
    
    for row in &rows {
        list_box.add(row);
    }
    
    (rows, row_to_entry_map)
}

pub fn create_list_view(
    entries: Vec<Box<dyn ClipboardEntry>>,
    width: i32,
) -> (ScrolledWindow, ListBox, Vec<ListBoxRow>, HashMap<ListBoxRow, Box<dyn ClipboardEntry>>) {
    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(width)
        .build();
    
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);
    
    let (rows, row_to_entry_map) = append_to_list_view(&list_box, entries, width);
    
    list_scrolled_window.add(&list_box);
    
    (list_scrolled_window, list_box, rows, row_to_entry_map)
}

