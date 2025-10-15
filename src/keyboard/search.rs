use gtk::{gdk, prelude::*, ApplicationWindow, Inhibit, ListBox, ListBoxRow, SearchEntry};
use std::rc::Rc;

use crate::ui::AppState;

use super::{
    cursor_movement::{navigate_and_refocus, select_first_row},
    entry_actions::{handle_copy_and_close, handle_open_in_external_app},
};

fn set_search_entry_appearance(search_entry: &SearchEntry, error: bool) {
    let style_context = search_entry.style_context();
    if error {
        style_context.add_class("error");
    } else {
        style_context.remove_class("error");
    }
}

pub fn enter_search_mode(
    search_entry: &SearchEntry,
    list_box: &ListBox,
    app_state: &Rc<AppState>,
    window: &ApplicationWindow,
) {
    search_entry.grab_focus();

    let list_box_clone = list_box.clone();
    let search_entry_clone = search_entry.clone();
    let app_state_clone = app_state.clone();
    let window_clone = window.clone();

    let app_state_for_changed = app_state.clone();
    let list_box_for_changed = list_box.clone();
    let search_entry_for_change = search_entry.clone();
    
    search_entry.connect_changed(move |entry| {
        let search_text = entry.text();

        rebuild_list(
            &list_box_for_changed,
            &app_state_for_changed,
            &search_text,
            &search_entry_for_change
        );
    });

    search_entry.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
        let state = event.state();
        let ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);

        if keyname == "Escape" || (ctrl && keyname == "c") {
            search_entry_clone.set_text("");
            select_first_row(&list_box_clone, true);
            return Inhibit(true);
        } else if keyname == "Return" {
            select_first_row(&list_box_clone, true);
            return Inhibit(true);
        } else if ctrl && keyname == "y" {
            return handle_copy_and_close(&window_clone, &list_box_clone, &app_state_clone, false);
        } else if ctrl && keyname == "p" {
            return handle_copy_and_close(&window_clone, &list_box_clone, &app_state_clone, false);
        } else if ctrl && keyname == "j" {
            navigate_and_refocus(&list_box_clone, &search_entry_clone, 1);
            return Inhibit(true);
        } else if ctrl && keyname == "k" {
            navigate_and_refocus(&list_box_clone, &search_entry_clone, -1);
            return Inhibit(true);
        } else if ctrl && (keyname == "e" || keyname == "o") {
            return handle_open_in_external_app(&list_box_clone, &app_state_clone);
        }

        return Inhibit(false);
    });
}

pub fn rebuild_list(
    list_box: &ListBox,
    app_state: &Rc<AppState>,
    search_text: &str,
    search_entry: &SearchEntry,
) {
    list_box.unselect_all();
    clear_list(list_box);

    if search_text.is_empty() {
        handle_empty_search(list_box, app_state, search_entry);
    } else {
        let cached_results = app_state.search_cache.borrow()
            .get(search_text)
            .cloned();
            
        if let Some(results) = cached_results {
            apply_search_results(list_box, app_state, &results, search_text, search_entry);
        } else {
            perform_new_search(list_box, app_state, search_text, search_entry);
        }
    }

    select_first_row(list_box, false);
}

fn handle_empty_search(
    list_box: &ListBox,
    app_state: &Rc<AppState>,
    search_entry: &SearchEntry,
) {
    set_search_entry_appearance(search_entry, true);

    for row in app_state.rows.borrow().iter() {
        list_box.add(row);
        if let Some(entry) = app_state.row_to_entry_map.borrow().get(row) {
            entry.set_highlight_in_row(None);
        }
    }

    *app_state.filtered_rows.borrow_mut() = None;
    *app_state.search_query.borrow_mut() = None;
}

fn apply_search_results(
    list_box: &ListBox,
    app_state: &Rc<AppState>,
    results: &Vec<ListBoxRow>,
    search_text: &str,
    search_entry: &SearchEntry,
) {
    for row in results {
        list_box.add(row);
        if let Some(entry) = app_state.row_to_entry_map.borrow().get(row) {
            entry.set_highlight_in_row(Some(search_text.to_string()));
        }
    }

    update_search_state(app_state, results, search_text, search_entry);
}

fn perform_new_search(
    list_box: &ListBox,
    app_state: &Rc<AppState>,
    search_text: &str,
    search_entry: &SearchEntry,
) {
    let last_query = app_state.search_query.borrow().clone();
    let rows_to_search: Vec<ListBoxRow> = {
        let should_use_filtered = if let Some(ref last_q) = last_query {
            search_text.starts_with(last_q) && app_state.filtered_rows.borrow().is_some()
        } else {
            false
        };
        
        if should_use_filtered {
            app_state.filtered_rows.borrow().as_ref().unwrap().clone()
        } else {
            app_state.rows.borrow().clone()
        }
    };

    let mut filtered_rows = Vec::new();

    {
        let row_map = app_state.row_to_entry_map.borrow();
        for row in &rows_to_search {
            if let Some(entry) = row_map.get(row) {
                if entry.contains_text(&search_text.to_lowercase()) {
                    entry.set_highlight_in_row(Some(search_text.to_string()));
                    filtered_rows.push(row.clone());
                    list_box.add(row);
                }
            }
        }
    } 

    app_state.search_cache.borrow_mut().insert(
        search_text.to_string(),
        filtered_rows.clone()
    );

    update_search_state(app_state, &filtered_rows, search_text, search_entry);
}

fn update_search_state(
    app_state: &Rc<AppState>,
    results: &Vec<ListBoxRow>,
    search_text: &str,
    search_entry: &SearchEntry,
) {
    set_search_entry_appearance(search_entry, results.is_empty());
    
    *app_state.filtered_rows.borrow_mut() = if results.is_empty() {
        None
    } else {
        Some(results.clone())
    };
    
    *app_state.search_query.borrow_mut() = if search_text.is_empty() {
        None
    } else {
        Some(search_text.to_string())
    };
}

fn clear_list(list_box: &ListBox) {
    for row in list_box.children() {
        list_box.remove(&row);
    }
}
