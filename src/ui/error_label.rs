use gtk::prelude::*;
use gtk::{ApplicationWindow, Label, Inhibit};
use gtk::gdk::{keys, ModifierType};

pub fn show_error(window: &ApplicationWindow, message: &str) {
    for child in window.children() {
        window.remove(&child);
    }
    
    // Add key information to the message
    let full_message = format!("{}\n\n(Press Escape or Ctrl+C to close)", message);
    
    let error_label = Label::new(Some(&full_message));
    error_label.set_margin(10);
    
    let window_clone = window.clone();
    window.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let state = event.state();
        
        // Check for Escape key
        if keyval == keys::constants::Escape {
            window_clone.close();
            return gtk::Inhibit(true);
        }
        
        // Check for Ctrl+C
        if state.contains(ModifierType::CONTROL_MASK) && keyval == keys::constants::c {
            window_clone.close();
            return Inhibit(true);
        }
        
        Inhibit(false)
    });
    
    window.add(&error_label);
    error_label.show();
    // window.show_all();
}
