use gtk::prelude::*;
use gtk::{ApplicationWindow, ListBox, Box as GTKBox, Entry, ScrolledWindow, gdk};
use std::rc::Rc;
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
// use clipboard_manager::clipboard_entries::clipboard_text_entry::ClipboardTextEntry;

use crate::get_clipboard_entries::get_clipboard_entries;
use crate::ui::{AppState, DetailsVisibility};
use crate::constants::{ENTRIES_WIDTH, INFO_BOX_WIDTH, APP_HEIGHT, INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT};
use crate::ui::populate_list_view;
use crate::ui::show_error;


fn get_current_entry<'a>(
    list_box: &ListBox,
    app_state: &'a AppState,
) -> Option<std::cell::Ref<'a, Box<dyn ClipboardEntry>>> {
    let selected_row = list_box.selected_row()?;
    let map = app_state.row_to_entry_map.borrow();
    
    // Check if the entry exists
    if map.contains_key(&selected_row) {
        Some(std::cell::Ref::map(map, |m| m.get(&selected_row).unwrap()))
    } else {
        None
    }
}

pub fn setup_keyboard_handler(
    window: &ApplicationWindow,
    list_box: &ListBox,
    main_box: &GTKBox,
    search_entry: &Entry,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    app_state: Rc<AppState>,
) {
    let window_clone = window.clone();
    let list_box_clone = list_box.clone();
    let main_box_clone = main_box.clone();
    let detail_scrolled_window_clone = detail_scrolled_window.clone();
    let detail_container_clone = detail_container.clone();
    let search_entry_clone = search_entry.clone();
    
    main_box.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
        let state = event.state();

        if state.contains(gdk::ModifierType::CONTROL_MASK) && keyname == "c" {
            window_clone.close();
            return Inhibit(true);
        }

        match keyname.as_str() {
            "j" => handle_move_down(&list_box_clone, &app_state, &window_clone),
            "k" => handle_move_up(&list_box_clone),
            "i" => handle_toggle_detail(
                &window_clone,
                &main_box_clone,
                &detail_scrolled_window_clone,
                &detail_container_clone,
                &list_box_clone,
                &app_state
            ),
            "e" | "o" => handle_open_in_external_app(&list_box_clone, &app_state),
            // "s" => enter_search_mode(&search_entry_clone, &list_box_clone, &app_state),
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
    let entries = match get_clipboard_entries(100) {
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

    let row_to_entry_map = populate_list_view(list_box, entries, ENTRIES_WIDTH);

    app_state.row_to_entry_map.replace(row_to_entry_map);

    app_state.all_entries_loaded.replace(true);

    list_box.show_all();
}

fn handle_move_down(list_box: &ListBox, app_state: &AppState, window: &ApplicationWindow) -> Inhibit {
    if let Some(selected_row) = list_box.selected_row() {
        let next_index = selected_row.index() + 1;
        let mut next_row = list_box.row_at_index(next_index);
        
        if next_row.is_none() && !(*app_state.all_entries_loaded.borrow()) {
            load_all_entries(list_box, app_state, window);
            next_row = list_box.row_at_index(next_index);
        }
        
        if let Some(next_row) = next_row {
            list_box.select_row(Some(&next_row));
            next_row.grab_focus();
        }
    }
    
    Inhibit(true)
}

fn handle_move_up(list_box: &ListBox) -> Inhibit {
    if let Some(selected_row) = list_box.selected_row() {
        if let Some(next_row) = list_box.row_at_index(selected_row.index() - 1) {
            list_box.select_row(Some(&next_row));
            next_row.grab_focus();
        }
    }

    Inhibit(true)
}

fn handle_toggle_detail(
    window: &ApplicationWindow,
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    let mut details_visibility = app_state.details_visibility.borrow_mut();
    match *details_visibility {
        DetailsVisibility::Hidden => {
            *details_visibility = DetailsVisibility::Normal;
        }
        DetailsVisibility::Normal => {
            *details_visibility = DetailsVisibility::Big;
        }
        DetailsVisibility::Big => {
            *details_visibility = DetailsVisibility::Hidden;
        }
    }
    
    if *details_visibility != DetailsVisibility::Hidden {
        if detail_scrolled_window.parent().is_none() {
            if *details_visibility == DetailsVisibility::Big {
                detail_scrolled_window.set_size_request(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT);
            } else {
                detail_scrolled_window.set_size_request(INFO_BOX_WIDTH, APP_HEIGHT);
            }
            main_box.pack_start(detail_scrolled_window, true, true, 0);
        } else {
            if *details_visibility == DetailsVisibility::Big {
                detail_scrolled_window.set_size_request(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT);
            } else {
                detail_scrolled_window.set_size_request(INFO_BOX_WIDTH, APP_HEIGHT);
            }
        }
        
        if let Some(entry) = get_current_entry(list_box, app_state) {
            for child in detail_container.children() {
                detail_container.remove(&child);
            }
            let detail_widget = {
                if *details_visibility == DetailsVisibility::Big {
                    entry.get_more_info(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT)
                } else {
                    entry.get_more_info(INFO_BOX_WIDTH, APP_HEIGHT)
                }
            };
            detail_container.add(&detail_widget);
            detail_container.show_all();
        }
        
        main_box.show_all();
    } else {
        main_box.remove(detail_scrolled_window);
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
    if let Some(entry) = get_current_entry(list_box, app_state) {
        if let Err(e) = entry.copy_to_clipboard() {
            eprintln!("Error copying to clipboard: {}", e);
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
    if let Some(entry) = get_current_entry(list_box, app_state) {
        if let Err(e) = entry.open_in_external_app() {
            eprintln!("Error opening in external app: {}", e);
        }
    }
    Inhibit(true)
}

// fn enter_search_mode(
//     search_entry: &Entry,
//     list_box: &ListBox,
//     app_state: &Rc<AppState>,
// ) -> Inhibit {
//     search_entry.grab_focus();
//
//     let list_box_clone = list_box.clone();
//     let current_index = app_state.current_index.borrow().clone();
//     let search_entry_clone = search_entry.clone();
//     let app_state_clone = app_state.clone();
//
//     let list_box_for_changed = list_box.clone();
//     let app_state_for_changed = app_state.clone();
//     search_entry.connect_changed(move |entry| {
//         let search_text = entry.text();
//         let search_str = search_text.as_str();
//         let entries = app_state_for_changed.entries.borrow();
//         
//         for entry in entries.iter() {
//             if search_str.is_empty() || entry.contains_text(search_str.to_string()) {
//                 entry.get_entry_row().show();
//             } else {
//                 entry.get_entry_row().hide();
//             }
//         }
//         
//         for i in 0..entries.len() {
//             if let Some(row) = list_box_for_changed.row_at_index(i as i32) {
//                 if row.is_visible() {
//                     list_box_for_changed.select_row(Some(&row));
//                     break;
//                 }
//             }
//         }
//     });
//
//     search_entry.connect_key_press_event(move |_, event| {
//         let keyval = event.keyval();
//         let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
//         let state = event.state();
//
//         if state.contains(gdk::ModifierType::CONTROL_MASK) && keyname == "c" {
//             search_entry_clone.set_text("");
//             if let Some(row) = list_box_clone.row_at_index(current_index as i32) {
//                 list_box_clone.select_row(Some(&row));
//                 row.grab_focus();
//             }
//             return Inhibit(true);
//         }
//
//         match keyname.as_str() {
//             "Return" => {
//                 if let Some(row) = list_box_clone.row_at_index(current_index as i32) {
//                     list_box_clone.select_row(Some(&row));
//                     row.grab_focus();
//                 }
//                 println!("Search for: {}", search_entry_clone.text().as_str());
//                 Inhibit(true)
//             }
//             "Escape" => {
//                 search_entry_clone.set_text("");
//                 // Reset visibility of all entries
//                 let entries = app_state_clone.entries.borrow();
//                 for entry in entries.iter() {
//                     entry.get_entry_row().show();
//                 }
//                 if let Some(row) = list_box_clone.row_at_index(current_index as i32) {
//                     list_box_clone.select_row(Some(&row));
//                     row.grab_focus();
//                 }
//                 Inhibit(true)
//             }
//             _ => {
//                 
//                 Inhibit(false)
//             }
//         }
//     });
//
//     Inhibit(true)
// }
