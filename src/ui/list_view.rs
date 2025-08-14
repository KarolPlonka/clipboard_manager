use gtk::prelude::*;
use gtk::{ListBox, ScrolledWindow, Label};
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
use crate::get_clipboard_entries::get_clipboard_entries;

pub fn populate_list_view(
    list_box: &ListBox,
    max_entries: usize,
    width: i32,
) -> Vec<Box<dyn ClipboardEntry>> {
    let entries = match get_clipboard_entries(max_entries) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error fetching clipboard entries: {}", e);
            vec![]
        }
    };

    list_box.foreach(|child| {
        list_box.remove(child);
    });

    if entries.is_empty() {
        let empty_row = Label::new(Some("No clipboard entries available."));
        empty_row.set_xalign(0.0);
        list_box.add(&empty_row);
    } else {
        for entry in entries.iter() {
            let row = entry.get_entry_row(width);
            list_box.add(&row);
        }
    }

    if !entries.is_empty() {
        list_box.select_row(list_box.row_at_index(0).as_ref());
    }

    entries
}

pub fn create_list_view(
    max_entries: usize,
    width: i32,
) -> (ScrolledWindow, ListBox, Vec<Box<dyn ClipboardEntry>>) {

    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(width)
        .build();

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    let entries = populate_list_view(&list_box, max_entries, width);

    list_scrolled_window.add(&list_box);

    (list_scrolled_window, list_box, entries)
}
