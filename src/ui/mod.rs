use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, Box as GTKBox, Orientation};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::constants::*;
use crate::keyboard_handler::setup_keyboard_handler;
use crate::get_clipboard_entries::get_clipboard_entries;

mod list_view;
mod detail_view;
mod app_state;
mod error_label;

use list_view::create_list_view;
use detail_view::create_detail_view;

pub use app_state::{AppState, DetailsVisibility};
pub use list_view::append_to_list_view;
pub use error_label::show_error;



pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Clipboard Manager")
        .default_height(APP_HEIGHT)
        .decorated(false)
        .build();

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
    root_box.pack_start(&search_entry, false, false, 0);

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
        &search_entry,
        &detail_scrolled_window,
        &detail_container,
        app_state,
    );

    window.show_all();
}

// TODO: denest
fn setup_list_selection_handler(
    list_box: &ListBox,
    detail_container: &GTKBox,
    app_state: Rc<AppState>,
) {
    let detail_container_clone = detail_container.clone();
    
    list_box.connect_row_selected(move |_, row| {
        // Only update detail view if it's visible
        if *app_state.details_visibility.borrow() != DetailsVisibility::Hidden {
            if let Some(row) = row {
                if let Some(entry) = app_state.row_to_entry_map.borrow().get(&row) {
                    // Clear previous content
                    for child in detail_container_clone.children() {
                        detail_container_clone.remove(&child);
                    }
                    let search_query = app_state.search_query.borrow();
                    entry.set_highlight_in_row(search_query.clone());
                    let (width, height) = if *app_state.details_visibility.borrow() == DetailsVisibility::Big {
                        (INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT)
                    } else {
                        (INFO_BOX_WIDTH, APP_HEIGHT)
                    };
                    let detail_widget = entry.create_more_info_widget(width, height, search_query.clone());
                    detail_container_clone.add(&detail_widget);
                    detail_container_clone.show_all();
                }
            } else {
                for child in detail_container_clone.children() {
                    detail_container_clone.remove(&child);
                }
                detail_container_clone.show_all();
            }
        }
    });
}
