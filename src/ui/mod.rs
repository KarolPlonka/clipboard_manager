use gtk::{
    prelude::*, Application, ApplicationWindow, Box as GTKBox, Orientation,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    constants::*,
    get_clipboard_entries::get_clipboard_entries,
    keyboard::setup_keyboard_handler,
};

mod app_state;
mod detail_view;
mod error_label;
mod list_view;

use detail_view::create_detail_view;
use list_view::{create_list_view, setup_list_selection_handler};

pub use app_state::{AppState, DetailsVisibility};
pub use error_label::show_error;
pub use list_view::append_to_list_view;



pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .default_height(APP_HEIGHT)
        .decorated(false)
        .modal(true)  
        .build();

    window.connect_is_active_notify(|window| {
        if !window.is_active() {
            window.close();
        }
    });

    let root_box = GTKBox::new(Orientation::Vertical, 0);

    let main_box = GTKBox::new(Orientation::Horizontal, 0);

    let entries = match get_clipboard_entries(
        INITIAL_ENTRIES,
        ENTRIES_WIDTH,
        ROW_IMAGE_MAX_HEIGHT,
        ROW_TEXT_MAX_LINES,
    ) {
        Ok(entries) if !entries.is_empty() => entries,
        Ok(_) => {
            show_error(&window, "No clipboard entries available.");
            return;
        }
        Err(e) => {
            show_error(&window, &format!("Error fetching clipboard entries: {}", e));
            return;
        }
    };

    let (list_scrolled_window, list_box, rows, row_to_entry_map) = create_list_view(entries, ENTRIES_WIDTH);

    let (detail_scrolled_window, detail_container) = create_detail_view(INFO_BOX_WIDTH);

    main_box.pack_start(&list_scrolled_window, false, false, 0);
    root_box.pack_start(&main_box, true, true, 0);

    let search_entry = gtk::SearchEntry::new();
    search_entry.set_placeholder_text(Some("Press 's' to search..."));
    search_entry.set_margin(5);

    window.add(&root_box);

    let app_state = Rc::new(AppState {
        rows: RefCell::new(rows),
        row_to_entry_map: RefCell::new(row_to_entry_map),
        details_visibility: RefCell::new(DetailsVisibility::Hidden),
        all_entries_loaded: RefCell::new(false),
        filtered_rows: RefCell::new(None),
        search_query: RefCell::new(None),
        search_cache: RefCell::new(HashMap::new()),
        last_selected_row: RefCell::new(None),
    });

    setup_list_selection_handler(
        &list_box,
        &detail_container,
        app_state.clone(),
    );
    
    setup_keyboard_handler(
        &window,
        &list_box,
        &main_box,
        &root_box,
        &search_entry,
        &detail_scrolled_window,
        &detail_container,
        app_state,
    );

    window.show_all();
}
