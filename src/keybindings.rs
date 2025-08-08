use gtk::prelude::*;
use gtk::{ApplicationWindow, ListBox};
use gtk::gdk;
use std::rc::Rc;
use std::cell::RefCell;
use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;

pub struct KeyHandler {
    window: ApplicationWindow,
    list_box: ListBox,
    entries: Rc<Vec<Box<dyn ClipboardEntry>>>,
    current_index: Rc<RefCell<usize>>,
}

impl KeyHandler {
    pub fn new(
        window: ApplicationWindow,
        list_box: ListBox,
        entries: Rc<Vec<Box<dyn ClipboardEntry>>>,
    ) -> Self {
        let current_index = Rc::new(RefCell::new(0));

        Self {
            window,
            list_box,
            entries,
            current_index,
        }
    }

    pub fn setup_key_bindings(&self) {
        let window_clone = self.window.clone();
        let list_box_clone = self.list_box.clone();
        let entries_clone = self.entries.clone();
        let current_index_clone = self.current_index.clone();
        let max_index = if self.entries.is_empty() { 0 } else { self.entries.len() - 1 };

        self.window.connect_key_press_event(move |_, event| {
            let keyval = event.keyval();
            let keyname = keyval.name().unwrap_or_else(|| gtk::glib::GString::from(""));
            let state = event.state();

            // Handle Ctrl+C
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
                    // Copy selected entry to clipboard
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
                "Escape" | "q" => {
                    // Quit application
                    window_clone.close();
                    Inhibit(true)
                }
                _ => Inhibit(false)
            }
        });
    }
}

// Optional: Create a struct to hold the keybinding configuration
pub struct KeyBinding {
    pub key: &'static str,
    pub modifiers: Option<gdk::ModifierType>,
    pub description: &'static str,
}

impl KeyBinding {
    pub const KEYBINDINGS: &'static [KeyBinding] = &[
        KeyBinding {
            key: "j",
            modifiers: None,
            description: "Move down in the list",
        },
        KeyBinding {
            key: "k",
            modifiers: None,
            description: "Move up in the list",
        },
        KeyBinding {
            key: "Return",
            modifiers: None,
            description: "Copy selected entry and close",
        },
        KeyBinding {
            key: "y",
            modifiers: None,
            description: "Copy selected entry and close (vim-style)",
        },
        KeyBinding {
            key: "Escape",
            modifiers: None,
            description: "Close without copying",
        },
        KeyBinding {
            key: "q",
            modifiers: None,
            description: "Quit application",
        },
        KeyBinding {
            key: "c",
            modifiers: Some(gdk::ModifierType::CONTROL_MASK),
            description: "Close application (Ctrl+C)",
        },
    ];
}
