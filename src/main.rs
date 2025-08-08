use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ScrolledWindow, Paned, Orientation};
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
        .default_width(entries_width + info_box_width)
        .default_height(app_height)
        .decorated(false)
        .build();

    // Create paned container for split view
    let paned = Paned::new(Orientation::Horizontal);
    paned.set_position(entries_width); // Set initial split position

    // Create scrolled window for list
    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();

    // Create scrolled window for detail view
    let detail_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
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
            let row = entry.get_entry_row(entries_width); // Use a fixed width for the row
            list_box.add(&row);
        }
    }

    // Select first row by default and show its detail
    if !entries.is_empty() {
        list_box.select_row(list_box.row_at_index(0).as_ref());
        
        // Show first entry's detail
        if let Some(entry) = entries.get(0) {
            // Clear previous content
            for child in detail_container.children() {
                detail_container.remove(&child);
            }
            // Add new content
            let detail_widget = entry.get_more_info(info_box_width, app_height);
            detail_container.add(&detail_widget);
            detail_container.show_all();
        }
    }

    list_scrolled_window.add(&list_box);
    
    // Add both scrolled windows to paned
    paned.pack1(&list_scrolled_window, true, false);
    paned.pack2(&detail_scrolled_window, true, false);
    
    window.add(&paned);

    // Set up keyboard event handling
    let current_index = Rc::new(RefCell::new(0));
    let max_index = if entries.is_empty() { 0 } else { entries.len() - 1 };
    let entries_rc = Rc::new(entries);

    let list_box_clone = list_box.clone();
    let window_clone = window.clone();
    let entries_clone = entries_rc.clone();

    // Handle list selection changes
    let entries_for_selection = entries_rc.clone();
    let detail_container_for_selection = detail_container.clone();
    
    list_box.connect_row_selected(move |list_box, row| {
        if let Some(row) = row {
            let index = row.index() as usize;
            
            // Update detail view
            if let Some(entry) = entries_for_selection.get(index) {
                // Clear previous content
                for child in detail_container_for_selection.children() {
                    detail_container_for_selection.remove(&child);
                }
                // Add new content
                let detail_widget = entry.get_more_info(entries_width, app_height);
                detail_container_for_selection.add(&detail_widget);
                detail_container_for_selection.show_all();
            }
        }
    });

    // Clone for keyboard handler
    let current_index_clone = current_index.clone();
    
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
