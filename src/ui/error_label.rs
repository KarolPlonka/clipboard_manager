use gtk::{gdk::{keys, ModifierType}, prelude::*, ApplicationWindow, Inhibit, Label};

pub fn show_error(window: &ApplicationWindow, message: &str) {
    for child in window.children() {
        window.remove(&child);
    }
    
    let full_message = format!("{}\n\n(Press Escape or Ctrl+C to close)", message);
    
    let error_label = Label::new(Some(&full_message));
    error_label.set_margin(10);
    
    let window_clone = window.clone();
    window.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let state = event.state();
        
        if keyval == keys::constants::Escape {
            window_clone.close();
            return gtk::Inhibit(true);
        }
        
        if state.contains(ModifierType::CONTROL_MASK) && keyval == keys::constants::c {
            window_clone.close();
            return Inhibit(true);
        }
        
        Inhibit(false)
    });
    
    window.add(&error_label);
    error_label.show();
}
