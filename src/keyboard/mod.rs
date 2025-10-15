use gtk::{
    gdk, prelude::*, ApplicationWindow, Box as GTKBox, Inhibit, ListBox, ScrolledWindow,
    SearchEntry,
};
use std::rc::Rc;

use crate::ui::AppState;

pub mod cursor_movement;
pub mod detail_views;
pub mod entry_actions;
pub mod entry_loading;
pub mod search;
pub mod help;

use cursor_movement::move_cursor;
use detail_views::{show_big_detail, toggle_detail};
use entry_actions::{handle_copy_and_close, handle_open_in_external_app};
use entry_loading::{load_all_entries, load_all_entries_if_reached_end};
use search::enter_search_mode;
use help::show_help_window;

pub fn setup_keyboard_handler(
    window: &ApplicationWindow,
    list_box: &ListBox,
    main_box: &GTKBox,
    root_box: &GTKBox,
    search_entry: &SearchEntry,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    app_state: Rc<AppState>,
) {
    let window_clone = window.clone();
    let list_box_clone = list_box.clone();
    let main_box_clone = main_box.clone();
    let root_box_clone = root_box.clone();
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
                if search_entry_clone.parent().is_none() {
                    root_box_clone.pack_start(&search_entry_clone, false, false, 0);
                }
                search_entry_clone.show();
                enter_search_mode(
                    &search_entry_clone,
                    &list_box_clone,
                    &app_state,
                    &window_clone,
                );
                Inhibit(true)
            },
            "Return" | "y" => handle_copy_and_close(&window_clone, &list_box_clone, &app_state, false),
            "p" => handle_copy_and_close(&window_clone, &list_box_clone, &app_state, true),
            "F1" => {
                show_help_window(&window_clone);
                Inhibit(true)
            }
            "Escape" | "q" => {
                window_clone.close();
                Inhibit(true)
            }
            _ => Inhibit(false)
        }
    });
}
