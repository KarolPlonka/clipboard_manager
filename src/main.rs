use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ScrolledWindow};
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
        .default_width(600)
        .default_height(400)
        .decorated(false)
        .build();

    // Create scrolled window
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();

    // Create list box
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    // Add entries to list box
    if entries.is_empty() {
        let empty_row = gtk::Label::new(Some("No clipboard entries available."));
        empty_row.set_xalign(0.0);
        list_box.add(&empty_row);
    } else {
        for entry in entries.iter() {
            let row = entry.get_entry_row();
            list_box.add(&row);
        }
    }

    // Select first row by default
    list_box.select_row(list_box.row_at_index(0).as_ref());

    scrolled_window.add(&list_box);
    window.add(&scrolled_window);

    // Set up keyboard event handling
    let current_index = Rc::new(RefCell::new(0));
    let max_index = entries.len() - 1;
    let entries_rc = Rc::new(entries);

    let list_box_clone = list_box.clone();
    let window_clone = window.clone();
    let current_index_clone = current_index.clone();
    let entries_clone = entries_rc.clone();

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
                // Print selected entry content and exit
                let index = *current_index_clone.borrow();
                if let Some(entry) = entries_clone.get(index) {
                    if let Err(e) = entry.copy_to_clipboard() {
                        eprintln!("Error copying to clipboard: {}", e);
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
