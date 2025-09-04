use gtk::prelude::*;
use gtk::{ApplicationWindow, ListBox, Box as GTKBox, SearchEntry, ScrolledWindow, gdk, ListBoxRow};
use std::rc::Rc;
use gtk::glib::idle_add_local;
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
// use clipboard_manager::clipboard_entries::clipboard_text_entry::ClipboardTextEntry;

use crate::get_clipboard_entries::get_clipboard_entries;
use crate::ui::{AppState, DetailsVisibility};
use crate::constants::*;
use crate::ui::append_to_list_view;
use crate::ui::show_error;


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


fn load_all_entries_if_reached_end(
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


pub fn setup_keyboard_handler(
    window: &ApplicationWindow,
    list_box: &ListBox,
    main_box: &GTKBox,
    search_entry: &SearchEntry,
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

    let window_for_move = window.clone();
    let list_box_for_move = list_box.clone();
    let app_state_for_move = app_state.clone();

    list_box.connect_move_cursor(move |_, _step, _count| {
        load_all_entries_if_reached_end(&list_box_for_move, &app_state_for_move, &window_for_move);
    });

    let list_box_clone_for_sc = list_box.clone();
    let app_state_for_sc = app_state.clone();
    list_box.connect_selected_rows_changed(move |_| {
        if let Some(search_entry) = app_state_for_sc.search_query.borrow().clone() {
            if search_entry.is_empty() {
                return;
            }
            if let Some(last_row) = app_state_for_sc.last_selected_row.borrow().clone() {
                if let Some(entry) = app_state_for_sc.row_to_entry_map.borrow().get(&last_row) {
                    entry.set_highlight_in_row(Some(search_entry));
                }
            }
        }
        if let Some(selected_row) = list_box_clone_for_sc.selected_row() {
            app_state_for_sc.last_selected_row.replace(Some(selected_row));
        }
    });
    
    main_box.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
        let state = event.state();

        if state.contains(gdk::ModifierType::CONTROL_MASK) && keyname == "c" {
            window_clone.close();
            return Inhibit(true);
        }

        match keyname.as_str() {
            "j" => {
                load_all_entries_if_reached_end(&list_box_clone, &app_state, &window_clone);
                return move_cursor(&list_box_clone, 1);
            }
            "k" => move_cursor(&list_box_clone, -1),
            "i" => toggle_detail(
                &window_clone,
                &main_box_clone,
                &detail_scrolled_window_clone,
                &detail_container_clone,
                &list_box_clone,
                &app_state
            ),
            "e" | "o" => handle_open_in_external_app(&list_box_clone, &app_state),
            "s" => {
                load_all_entries(&list_box_clone, &app_state, &window_clone);
                show_big_detail(
                    &main_box_clone,
                    &detail_scrolled_window_clone,
                    &detail_container_clone,
                    &list_box_clone,
                    &app_state
                );
                enter_search_mode(
                    &search_entry_clone,
                    &list_box_clone,
                    &app_state,
                    &window_clone,
                );
                Inhibit(true)
            },
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


fn move_cursor(list_box: &ListBox, offset: i32) -> Inhibit {
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


fn hide_detail(
    window: &ApplicationWindow,
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    app_state: &AppState,
) -> Inhibit {
    main_box.remove(detail_scrolled_window);
    window.resize(ENTRIES_WIDTH, APP_HEIGHT);
    *app_state.details_visibility.borrow_mut() = DetailsVisibility::Hidden;
    Inhibit(true)
}


fn show_normal_detail(
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    *app_state.details_visibility.borrow_mut() = DetailsVisibility::Normal;
    
    if detail_scrolled_window.parent().is_none() {
        detail_scrolled_window.set_size_request(INFO_BOX_WIDTH, APP_HEIGHT);
        main_box.pack_start(detail_scrolled_window, true, true, 0);
    } else {
        detail_scrolled_window.set_size_request(INFO_BOX_WIDTH, APP_HEIGHT);
    }
    
    if let Some(entry) = get_current_entry(list_box, app_state) {
        for child in detail_container.children() {
            detail_container.remove(&child);
        }
        let detail_widget = entry.create_more_info_widget(
            INFO_BOX_WIDTH,
            APP_HEIGHT,
            app_state.search_query.borrow().clone()
        );
        detail_container.add(&detail_widget);
        detail_container.show_all();
    }
    
    detail_scrolled_window.show_all();
    Inhibit(true)
}


fn show_big_detail(
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    *app_state.details_visibility.borrow_mut() = DetailsVisibility::Big;
    
    if detail_scrolled_window.parent().is_none() {
        detail_scrolled_window.set_size_request(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT);
        main_box.pack_start(detail_scrolled_window, true, true, 0);
    } else {
        detail_scrolled_window.set_size_request(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT);
    }
    
    if let Some(entry) = get_current_entry(list_box, app_state) {
        for child in detail_container.children() {
            detail_container.remove(&child);
        }
        let detail_widget = entry.create_more_info_widget(
            INFO_BOX_BIG_WIDTH,
            INFO_BOX_BIG_HEIGHT,
            app_state.search_query.borrow().clone()
        );
        detail_container.add(&detail_widget);
        detail_container.show_all();
    }
    
    detail_scrolled_window.show_all();
    Inhibit(true)
}


fn toggle_detail(
    window: &ApplicationWindow,
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    // Determine the next state
    let next_visibility = {
        let current = app_state.details_visibility.borrow();
        match *current {
            DetailsVisibility::Hidden => DetailsVisibility::Normal,
            DetailsVisibility::Normal => DetailsVisibility::Big,
            DetailsVisibility::Big => DetailsVisibility::Hidden,
        }
    }; // The borrow is dropped here
    
    match next_visibility {
        DetailsVisibility::Hidden => {
            hide_detail(window, main_box, detail_scrolled_window, app_state);
        },
        DetailsVisibility::Normal => {
            show_normal_detail(main_box, detail_scrolled_window, detail_container, list_box, app_state);
        },
        DetailsVisibility::Big => {
            show_big_detail(main_box, detail_scrolled_window, detail_container, list_box, app_state);
        },
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

fn set_search_entry_appearance(search_entry: &SearchEntry, error: bool) {
    let style_context = search_entry.style_context();
    if error {
        style_context.add_class("error");
    } else {
        style_context.remove_class("error");
    }
}

fn enter_search_mode(
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
        let search_str = search_text.as_str();

        list_box_for_changed.unselect_all();

        for row in list_box_for_changed.children() {
            list_box_for_changed.remove(&row);
        }
        
        if search_str.is_empty() {
            set_search_entry_appearance(&search_entry_for_change, true);

            if *app_state_for_changed.filtered_rows.borrow() == None &&
               *app_state_for_changed.search_query.borrow() == None 
            {
                return;
            }

            for row in app_state_for_changed.rows.borrow().iter() {
                list_box_for_changed.add(row);
                if let Some(entry) = app_state_for_changed.row_to_entry_map.borrow().get(row) {
                    entry.set_highlight_in_row(None);
                }
            }

            *app_state_for_changed.filtered_rows.borrow_mut() = None;
            *app_state_for_changed.search_query.borrow_mut() = None;

            if let Some(first_row) = list_box_for_changed.row_at_index(0) {
                list_box_for_changed.select_row(Some(&first_row));
            }

            return;
        }
        
        if let Some(cached_results) = app_state_for_changed.search_cache.borrow().get(search_str) {
            for row in cached_results {
                list_box_for_changed.add(row);
                if let Some(entry) = app_state_for_changed.row_to_entry_map.borrow().get(row) {
                    entry.set_highlight_in_row(Some(search_text.to_string()));
                }
            }

            set_search_entry_appearance(&search_entry_for_change, cached_results.is_empty());
            
            *app_state_for_changed.filtered_rows.borrow_mut() = if cached_results.is_empty() {
                None
            } else {
                Some(cached_results.clone())
            };
            
            *app_state_for_changed.search_query.borrow_mut() = if search_str.is_empty() {
                None
            } else {
                Some(search_str.to_string())
            };

            if let Some(first_row) = list_box_for_changed.row_at_index(0) {
                list_box_for_changed.select_row(Some(&first_row));
            }

            return;
        }

        let last_query = app_state_for_changed.search_query.borrow().clone();
        let row_map = app_state_for_changed.row_to_entry_map.borrow();

        let rows = app_state_for_changed.rows.clone();
        let rows_borrowed = rows.borrow();
        let rows_to_search: Vec<&ListBoxRow> = if let Some(ref last_q) = last_query {
            if search_str.starts_with(last_q) {
                if let Some(ref filtered) = *app_state_for_changed.filtered_rows.borrow() {
                    filtered.iter()
                        .filter_map(|row| row_map.get_key_value(row).map(|(k, _)| k))
                        .collect()
                } else {
                    rows_borrowed.iter().collect()
                }
            } else {
                rows_borrowed.iter().collect()
            }
        } else {
            rows_borrowed.iter().collect()
        };

        let mut filtered_rows: Vec<ListBoxRow> = Vec::new();
        for row in rows_to_search {
            if let Some(entry) = row_map.get(row) {
                if entry.contains_text(&search_str.to_lowercase()) {
                    entry.set_highlight_in_row(Some(search_text.to_string()));
                    filtered_rows.push(row.clone());
                    list_box_for_changed.add(row);
                }             
            }
        }

        set_search_entry_appearance(&search_entry_for_change, filtered_rows.is_empty());

        app_state_for_changed.search_cache.borrow_mut().insert(
            search_str.to_string(),
            filtered_rows.clone()
        );
        
        *app_state_for_changed.filtered_rows.borrow_mut() = if filtered_rows.is_empty() {
            None
        } else {
            Some(filtered_rows)
        };
        
        *app_state_for_changed.search_query.borrow_mut() = if search_str.is_empty() {
            None
        } else {
            Some(search_str.to_string())
        };

        if let Some(first_row) = list_box_for_changed.row_at_index(0) {
            list_box_for_changed.select_row(Some(&first_row));
        }
    });

    search_entry.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
        let state = event.state();
        let ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);

        if keyname == "Escape" || (ctrl && keyname == "c") {
            search_entry_clone.set_text("");
            list_box_clone.unselect_all();
            for row in list_box_clone.children() {
                list_box_clone.remove(&row);
            }
            for row in app_state_clone.rows.borrow().iter() {
                if let Some(entry) = app_state_clone.row_to_entry_map.borrow().get(row) {
                    entry.set_highlight_in_row(None);
                }
                list_box_clone.add(row);
            }
            set_search_entry_appearance(&search_entry_clone, false);
            app_state_clone.search_query.replace(None);
            app_state_clone.filtered_rows.replace(None);
            if let Some(first_row) = list_box_clone.row_at_index(0) {
                list_box_clone.select_row(Some(&first_row));
                first_row.grab_focus();
            }
            return Inhibit(true);
        }

        if keyname == "Return" {
            if let Some(first_row) = list_box_clone.row_at_index(0) {
                list_box_clone.select_row(Some(&first_row));
                first_row.grab_focus();
            }
            return Inhibit(true);
        }

        if ctrl && keyname == "y" {
            handle_copy_and_close(&window_clone, &list_box_clone, &app_state_clone);
            return Inhibit(true);
        }
        
        if ctrl && keyname == "j" {
            move_cursor(&list_box_clone, 1);
            let search_entry_for_grab_focus = search_entry_clone.clone();
            idle_add_local(move || {
                search_entry_for_grab_focus.grab_focus();
                search_entry_for_grab_focus.set_position(-1);
                Continue(false)
            });
            return Inhibit(true);
        }

        if ctrl && keyname == "k" {
            move_cursor(&list_box_clone, -1);
            let search_entry_for_grab_focus = search_entry_clone.clone();
            idle_add_local(move || {
                search_entry_for_grab_focus.grab_focus();
                search_entry_for_grab_focus.set_position(-1);
                Continue(false)
            });
            return Inhibit(true);
        }

        if ctrl && (keyname == "e" || keyname == "o") {
            handle_open_in_external_app(&list_box_clone, &app_state_clone);
            return Inhibit(true);
        }

        return Inhibit(false);
    });
}
