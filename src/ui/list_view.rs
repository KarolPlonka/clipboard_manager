use gtk::prelude::*;
use gtk::{ListBox, ScrolledWindow, Label};
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

pub fn create_list_view(
    entries: &[Box<dyn ClipboardEntry>],
    width: i32,
) -> (ScrolledWindow, ListBox) {
    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(width)
        .build();

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    // Add entries to list box
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

    // Select first row by default
    if !entries.is_empty() {
        list_box.select_row(list_box.row_at_index(0).as_ref());
    }

    list_scrolled_window.add(&list_box);

    (list_scrolled_window, list_box)
}
