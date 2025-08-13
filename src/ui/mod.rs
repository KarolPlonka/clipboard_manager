use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ScrolledWindow, Box, Orientation};
use std::rc::Rc;
use std::cell::RefCell;

use crate::get_clipboard_entries::get_clipboard_entries;
use crate::constants::*;
use crate::keyboard_handler::setup_keyboard_handler;

mod list_view;
mod detail_view;
mod app_state;

use list_view::create_list_view;
use detail_view::create_detail_view;
pub use app_state::AppState;

pub fn build_ui(app: &Application) {
    // Fetch clipboard entries
    let entries = match get_clipboard_entries(MAX_ENTRIES) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error fetching clipboard entries: {}", e);
            vec![]
        }
    };

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .default_width(ENTRIES_WIDTH)
        .default_height(APP_HEIGHT)
        .decorated(false)
        .build();

    // Create horizontal box to hold both sides
    let main_box = Box::new(Orientation::Horizontal, 0);

    // Create list view
    let (list_scrolled_window, list_box) = create_list_view(&entries, ENTRIES_WIDTH);

    // Create detail view
    let (detail_scrolled_window, detail_container) = create_detail_view(INFO_BOX_WIDTH);

    // Initially only pack the list scrolled window
    main_box.pack_start(&list_scrolled_window, false, false, 0);
    window.add(&main_box);

    // Create app state
    let app_state = Rc::new(AppState {
        entries: Rc::new(entries),
        details_visible: RefCell::new(false),
        current_index: RefCell::new(0),
    });

    // Setup list selection handler
    setup_list_selection_handler(
        &list_box,
        &detail_container,
        app_state.clone(),
    );

    // Setup keyboard handler
    setup_keyboard_handler(
        &window,
        &list_box,
        &main_box,
        &detail_scrolled_window,
        &detail_container,
        app_state,
    );

    window.show_all();
}

fn setup_list_selection_handler(
    list_box: &ListBox,
    detail_container: &Box,
    app_state: Rc<AppState>,
) {
    let detail_container_clone = detail_container.clone();
    
    list_box.connect_row_selected(move |_, row| {
        // Only update detail view if it's visible
        if *app_state.details_visible.borrow() {
            if let Some(row) = row {
                let index = row.index() as usize;
                
                if let Some(entry) = app_state.entries.get(index) {
                    // Clear previous content
                    for child in detail_container_clone.children() {
                        detail_container_clone.remove(&child);
                    }
                    // Add new content
                    let detail_widget = entry.get_more_info(INFO_BOX_WIDTH, APP_HEIGHT);
                    detail_container_clone.add(&detail_widget);
                    detail_container_clone.show_all();
                }
            }
        }
    });
}
