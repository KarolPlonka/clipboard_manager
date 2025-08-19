use gtk::prelude::*;
use gtk::{ApplicationWindow, Label};

pub fn show_error(window: &ApplicationWindow, message: &str) {
    for child in window.children() {
        window.remove(&child);
    }
    let error_label = Label::new(Some(message));
    error_label.set_margin(10);

    let window_clone = window.clone();

    window.connect_key_press_event(move |_, _event| {
        window_clone.close();
        return Inhibit(true);
    });

    window.add(&error_label);

    window.show_all();
}
