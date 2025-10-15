use gtk::{glib::{idle_add_local, Continue}, prelude::*, Inhibit, ListBox};

pub fn move_cursor(list_box: &ListBox, offset: i32) -> Inhibit {
    let target_index = if let Some(selected_row) = list_box.selected_row() {
        selected_row.index() + offset
    } else {
        0
    };

    if target_index < 0 || target_index >= list_box.children().len() as i32 {
        return Inhibit(true);
    }

    if let Some(target_row) = list_box.row_at_index(target_index) {
        list_box.select_row(Some(&target_row));
        let target_row_clone = target_row.clone();
        idle_add_local(move || {
            target_row_clone.grab_focus();
            Continue(false) 
        });
    }
    
    Inhibit(true)
}

pub fn navigate_and_refocus(list_box: &ListBox, search_entry: &gtk::SearchEntry, direction: i32) {
    move_cursor(list_box, direction);
    let search_entry_clone = search_entry.clone();
    idle_add_local(move || {
        search_entry_clone.grab_focus();
        search_entry_clone.set_position(-1);
        Continue(false)
    });
}

pub fn select_first_row(list_box: &ListBox, grab_focus: bool) {
    if let Some(first_row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&first_row));
        if grab_focus {
            first_row.grab_focus();
        }
    }
}