use gtk::prelude::*;
use gtk::{ApplicationWindow, ListBox, Box, ScrolledWindow, gdk};
use std::rc::Rc;
// use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

use crate::ui::AppState;
use crate::constants::{ENTRIES_WIDTH, INFO_BOX_WIDTH, APP_HEIGHT};
use crate::ui::populate_list_view;

pub fn setup_keyboard_handler(
    window: &ApplicationWindow,
    list_box: &ListBox,
    main_box: &Box,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &Box,
    app_state: Rc<AppState>,
) {
    let window_clone = window.clone();
    let list_box_clone = list_box.clone();
    let main_box_clone = main_box.clone();
    let detail_scrolled_window_clone = detail_scrolled_window.clone();
    let detail_container_clone = detail_container.clone();
    
    window.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
        let state = event.state();

        if state.contains(gdk::ModifierType::CONTROL_MASK) && keyname == "c" {
            window_clone.close();
            return Inhibit(true);
        }

        match keyname.as_str() {
            "j" => handle_move_down(&list_box_clone, &app_state, &window_clone),
            "k" => handle_move_up(&list_box_clone, &app_state),
            "i" => handle_toggle_detail(
                &window_clone,
                &main_box_clone,
                &detail_scrolled_window_clone,
                &detail_container_clone,
                &list_box_clone,
                &app_state
            ),
            "e" | "o" => handle_open_in_external_app(&list_box_clone, &app_state),
            // "a" => load_all_entries(&list_box_clone, &app_state, &window_clone),
            "Return" | "y" => handle_copy_and_close(&window_clone, &list_box_clone, &app_state),
            "Escape" | "q" => {
                window_clone.close();
                Inhibit(true)
            }
            _ => Inhibit(false)
        }
    });
}

fn load_all_entries(list_box: &ListBox, app_state: &AppState, window: &ApplicationWindow) {
    let entries = populate_list_view(list_box, 100, ENTRIES_WIDTH);

    app_state.entries.replace(entries);

    window.show_all();
}

fn handle_move_down(list_box: &ListBox, app_state: &AppState, window: &ApplicationWindow) -> Inhibit {
    let mut current_index = app_state.current_index.borrow_mut();
    let mut max_index = app_state.max_index();

    if *current_index == max_index {
        load_all_entries(list_box, app_state, window);
    }

    max_index = app_state.max_index();
    
    if *current_index < max_index {
        *current_index += 1;
        if let Some(row) = list_box.row_at_index(*current_index as i32) {
            list_box.select_row(Some(&row));
            row.grab_focus();
        }
    }
    Inhibit(true)
}

fn handle_move_up(list_box: &ListBox, app_state: &AppState) -> Inhibit {
    let mut current_index = app_state.current_index.borrow_mut();
    
    if *current_index > 0 {
        *current_index -= 1;
        if let Some(row) = list_box.row_at_index(*current_index as i32) {
            list_box.select_row(Some(&row));
            row.grab_focus();
        }
    }
    Inhibit(true)
}

fn handle_toggle_detail(
    window: &ApplicationWindow,
    main_box: &Box,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &Box,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    let mut details_visible = app_state.details_visible.borrow_mut();
    *details_visible = !*details_visible;
    
    if *details_visible {
        // Show detail view
        main_box.pack_start(detail_scrolled_window, true, true, 0);
        
        // Update window width to accommodate both panels
        window.resize(ENTRIES_WIDTH + INFO_BOX_WIDTH, APP_HEIGHT);
        
        // Update detail content for currently selected row
        if let Some(selected_row) = list_box.selected_row() {
            let index = selected_row.index() as usize;
            if let Some(entry) = app_state.entries.borrow().get(index) {
                // Clear previous content
                for child in detail_container.children() {
                    detail_container.remove(&child);
                }
                // Add new content
                let detail_widget = entry.get_more_info(INFO_BOX_WIDTH, APP_HEIGHT);
                detail_container.add(&detail_widget);
                detail_container.show_all();
            }
        }
        
        main_box.show_all();
    } else {
        // Hide detail view
        main_box.remove(detail_scrolled_window);
        
        // Resize window back to list-only width
        window.resize(ENTRIES_WIDTH, APP_HEIGHT);
    }
    
    Inhibit(true)
}

fn handle_copy_and_close(
    window: &ApplicationWindow,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    // Get the currently selected row
    if let Some(selected_row) = list_box.selected_row() {
        let index = selected_row.index() as usize;
        if let Some(entry) = app_state.entries.borrow().get(index) {
            if let Err(e) = entry.copy_to_clipboard() {
                eprintln!("Error copying to clipboard: {}", e);
            }
        }
    }
    window.close();
    Inhibit(true)
}

fn handle_open_in_external_app(
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    // Get the currently selected row
    if let Some(selected_row) = list_box.selected_row() {
        let index = selected_row.index() as usize;
        if let Some(entry) = app_state.entries.borrow().get(index) {
            if let Err(e) = entry.open_in_external_app() {
                eprintln!("Error opening in external app: {}", e);
            }
        }
    }
    Inhibit(true)
}
