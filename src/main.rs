use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ScrolledWindow, Box, Orientation};
use gtk::gdk;
use std::rc::Rc;
use std::cell::RefCell;


mod get_clipboard_entries;
use get_clipboard_entries::get_clipboard_entries;


fn main() {
    let app = Application::builder()
        .application_id("com.example.clipboard-manager")
        .build();
    app.connect_activate(build_ui);
    app.run();
}


fn build_ui(app: &Application) {
    let app_height = 400;
    let entries_width = 500;
    let info_box_width = 500;

    // Fetch clipboard entries
    let entries = match get_clipboard_entries(20) {
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
        .default_width(entries_width) // Start with just the list width
        .default_height(app_height)
        .decorated(false)
        .build();

    // Create horizontal box to hold both sides
    let main_box = Box::new(Orientation::Horizontal, 0);

    // Create scrolled window for list
    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(entries_width)
        .build();

    // Create scrolled window for detail view (but don't add it yet)
    let detail_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(info_box_width)
        .build();

    // Create list box
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    // Container for detail view (will hold the widget from get_more_info)
    let detail_container = gtk::Box::new(Orientation::Vertical, 0);
    detail_scrolled_window.add(&detail_container);

    // Add entries to list box
    if entries.is_empty() {
        let empty_row = gtk::Label::new(Some("No clipboard entries available."));
        empty_row.set_xalign(0.0);
        list_box.add(&empty_row);
    } else {
        for entry in entries.iter() {
            let row = entry.get_entry_row(entries_width);
            list_box.add(&row);
        }
    }

    // Select first row by default (but don't show details)
    if !entries.is_empty() {
        list_box.select_row(list_box.row_at_index(0).as_ref());
    }

    list_scrolled_window.add(&list_box);
    
    // Initially only pack the list scrolled window
    main_box.pack_start(&list_scrolled_window, false, false, 0);
    
    window.add(&main_box);

    // Track whether details are visible
    let details_visible = Rc::new(RefCell::new(false));

    // Set up keyboard event handling
    let current_index = Rc::new(RefCell::new(0));
    let max_index = if entries.is_empty() { 0 } else { entries.len() - 1 };
    let entries_rc = Rc::new(entries);

    let list_box_clone = list_box.clone();
    let window_clone = window.clone();
    let entries_clone = entries_rc.clone();

    // Handle list selection changes (but only update detail view if visible)
    let entries_for_selection = entries_rc.clone();
    let detail_container_for_selection = detail_container.clone();
    let details_visible_for_selection = details_visible.clone();
    
    list_box.connect_row_selected(move |_, row| {
        // Only update detail view if it's visible
        if *details_visible_for_selection.borrow() {
            if let Some(row) = row {
                let index = row.index() as usize;
                
                if let Some(entry) = entries_for_selection.get(index) {
                    // Clear previous content
                    for child in detail_container_for_selection.children() {
                        detail_container_for_selection.remove(&child);
                    }
                    // Add new content
                    let detail_widget = entry.get_more_info(info_box_width, app_height);
                    detail_container_for_selection.add(&detail_widget);
                    detail_container_for_selection.show_all();
                }
            }
        }
    });

    // Clone for keyboard handler
    let current_index_clone = current_index.clone();
    let details_visible_clone = details_visible.clone();
    let main_box_clone = main_box.clone();
    let detail_scrolled_window_clone = detail_scrolled_window.clone();
    let entries_for_toggle = entries_rc.clone();
    let detail_container_for_toggle = detail_container.clone();
    let list_box_for_toggle = list_box.clone();
    let window_for_toggle = window.clone();
    
    window.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
        let state = event.state();

        if state.contains(gdk::ModifierType::CONTROL_MASK) && keyname == "c" {
            window_clone.close();
            return Inhibit(true);
        }

        match keyname.as_str() {
            "j" => {
                // Move down
                let mut index = current_index_clone.borrow_mut();
                if *index < max_index {
                    *index += 1;
                    if let Some(row) = list_box_clone.row_at_index(*index as i32) {
                        list_box_clone.select_row(Some(&row));
                        row.grab_focus();
                    }
                }
                Inhibit(true)
            }
            "k" => {
                // Move up
                let mut index = current_index_clone.borrow_mut();
                if *index > 0 {
                    *index -= 1;
                    if let Some(row) = list_box_clone.row_at_index(*index as i32) {
                        list_box_clone.select_row(Some(&row));
                        row.grab_focus();
                    }
                }
                Inhibit(true)
            }
            "i" => {
                // Toggle detail view
                let mut visible = details_visible_clone.borrow_mut();
                *visible = !*visible;
                
                if *visible {
                    // Show detail view
                    main_box_clone.pack_start(&detail_scrolled_window_clone, true, true, 0);
                    
                    // Update window width to accommodate both panels
                    window_for_toggle.resize(entries_width + info_box_width, app_height);
                    
                    // Update detail content for currently selected row
                    if let Some(selected_row) = list_box_for_toggle.selected_row() {
                        let index = selected_row.index() as usize;
                        if let Some(entry) = entries_for_toggle.get(index) {
                            // Clear previous content
                            for child in detail_container_for_toggle.children() {
                                detail_container_for_toggle.remove(&child);
                            }
                            // Add new content
                            let detail_widget = entry.get_more_info(info_box_width, app_height);
                            detail_container_for_toggle.add(&detail_widget);
                            detail_container_for_toggle.show_all();
                        }
                    }
                    
                    main_box_clone.show_all();
                } else {
                    // Hide detail view
                    main_box_clone.remove(&detail_scrolled_window_clone);
                    
                    // Resize window back to list-only width
                    window_for_toggle.resize(entries_width, app_height);
                }
                
                Inhibit(true)
            }
            "Return" | "y" => {
                // Get the currently selected row instead of using current_index
                if let Some(selected_row) = list_box_clone.selected_row() {
                    let index = selected_row.index() as usize;
                    if let Some(entry) = entries_clone.get(index) {
                        if let Err(e) = entry.copy_to_clipboard() {
                            eprintln!("Error copying to clipboard: {}", e);
                        }
                    }
                }
                window_clone.close();
                Inhibit(true)
            }
            "Escape" | "q"  => {
                window_clone.close();
                Inhibit(true)
            }
            _ => Inhibit(false)
        }
    });

    window.show_all();
}
