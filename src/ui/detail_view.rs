use gtk::{prelude::*, Box, Orientation, ScrolledWindow};

pub fn create_detail_view(width: i32) -> (ScrolledWindow, Box) {
    let detail_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(width)
        .build();

    let detail_container = Box::new(Orientation::Vertical, 0);
    detail_scrolled_window.add(&detail_container);

    (detail_scrolled_window, detail_container)
}
