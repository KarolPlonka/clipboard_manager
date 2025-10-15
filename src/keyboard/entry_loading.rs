use gtk::{prelude::*, ApplicationWindow, ListBox};

use crate::{
    constants::*,
    get_clipboard_entries::get_clipboard_entries,
    ui::{append_to_list_view, show_error, AppState},
};

pub fn load_all_entries_if_reached_end(
    list_box: &ListBox,
    app_state: &AppState,
    window: &ApplicationWindow,
) {
    if *app_state.all_entries_loaded.borrow() {
        return;
    }

    let Some(selected_row) = list_box.selected_row() else {
        return;
    };

    let index = selected_row.index();

    if index != list_box.children().len() as i32 - 1 {
        return;
    }

    load_all_entries(list_box, app_state, window);

    if let Some(row) = list_box.row_at_index(index) {
        list_box.select_row(Some(&row));
        row.grab_focus();
    }
}

pub fn load_all_entries(list_box: &ListBox, app_state: &AppState, window: &ApplicationWindow) {
    if *app_state.all_entries_loaded.borrow() {
        return;
    }

    let entries = match get_clipboard_entries(
        MAX_ENTRIES,
        ENTRIES_WIDTH,
        ROW_IMAGE_MAX_HEIGHT,
        ROW_TEXT_MAX_LINES,
    ) {
        Ok(entries) if !entries.is_empty() => entries,
        Ok(_) => {
            show_error(window, "No more clipboard entries available.");
            return;
        }
        Err(e) => {
            show_error(window, &format!("Error fetching clipboard entries: {}", e));
            return;
        }
    };

    let new_entries = entries.into_iter().skip(INITIAL_ENTRIES).collect();

    let (rows, row_to_entry_map) = append_to_list_view(list_box, new_entries);

    app_state.rows.borrow_mut().extend(rows);
    app_state.row_to_entry_map.borrow_mut().extend(row_to_entry_map);

    app_state.all_entries_loaded.replace(true);

    list_box.show_all();
}