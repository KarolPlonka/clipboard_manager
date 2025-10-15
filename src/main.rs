use gtk::{gio, prelude::*, Application};

use crate::ui::build_ui;

mod constants;
mod get_clipboard_entries;
mod keyboard;
mod ui;

fn main() {
    let app = Application::builder()
        .application_id("com.example.clipboard-manager")
        .flags(gio::ApplicationFlags::REPLACE)
        .build();

    app.connect_activate(build_ui);
    app.run();
}
