use gtk::prelude::*;
use gtk::Application;

mod get_clipboard_entries;
mod ui;
mod keyboard_handler;
mod constants;

use ui::build_ui;

fn main() {
    let app = Application::builder()
        .application_id("com.example.clipboard-manager")
        .build();
    app.connect_activate(build_ui);
    app.run();
}
