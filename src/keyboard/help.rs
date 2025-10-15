use gtk::{prelude::*, Align, ApplicationWindow, Box, Grid, Label, Orientation, Window};

pub fn show_help_window(window: &ApplicationWindow) {
    let help_window = Window::builder()
        .transient_for(window)
        .modal(true)
        .resizable(false)
        .decorated(false)
        .build();
    
    let main_box = Box::new(Orientation::Vertical, 12);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    
    let title = Label::new(Some("Keyboard Shortcuts"));
    title.set_margin_bottom(12);
    
    let normal_title = Label::new(Some("NORMAL MODE"));
    normal_title.style_context().add_class("dim-label");
    normal_title.set_halign(Align::Start);
    normal_title.set_margin_top(8);
    
    let normal_shortcuts = create_shortcuts_box(vec![
        ("j / k", "Navigate up/down"),
        ("Enter / y", "Copy and exit"),
        ("p", "Copy file path and exit"),
        ("o", "Open in external app"),
        ("e", "Open in editor"),
        ("i", "Show more/less info"),
        ("Ctrl+s", "Enter search mode"),
        ("Ctrl+c / Esc / q", "Exit app"),
    ]);
    
    let search_title = Label::new(Some("SEARCH MODE"));
    search_title.style_context().add_class("dim-label");
    search_title.set_halign(Align::Start);
    search_title.set_margin_top(16);
    
    let search_shortcuts = create_shortcuts_box(vec![
        ("Ctrl+j / Ctrl+k", "Navigate up/down"),
        ("Enter / Ctrl+y", "Copy and exit"),
        ("Ctrl+p", "Copy file path and exit"),
        ("Ctrl+o", "Open in external app"),
        ("Ctrl+e", "Open in editor"),
        ("Ctrl+i", "Show more/less info"),
        ("Ctrl+c / Esc", "Exit search mode"),
    ]);
    
    let close_label = Label::new(Some("Press Escape, Ctrl+c, or q to close"));
    close_label.set_margin_top(16);
    
    main_box.add(&title);
    main_box.add(&normal_title);
    main_box.add(&normal_shortcuts);
    main_box.add(&search_title);
    main_box.add(&search_shortcuts);
    main_box.add(&close_label);
    
    main_box.show_all();
    
    help_window.set_child(Some(&main_box));
    
    let help_window_clone = help_window.clone();
    help_window.connect_key_press_event(move |_, event| {
        match event.keyval() {
            gtk::gdk::keys::constants::Escape | gtk::gdk::keys::constants::q => {
                help_window_clone.close();
                Inhibit(true)
            }
            gtk::gdk::keys::constants::c if event.state().contains(gtk::gdk::ModifierType::CONTROL_MASK) => {
                help_window_clone.close();
                Inhibit(true)
            }
            _ => Inhibit(false)
        }
    });
    
    help_window.present();
}


fn create_shortcuts_box(shortcuts: Vec<(&str, &str)>) -> Box {
    let shortcuts_box = Box::new(Orientation::Vertical, 4);
    
    let grid = Grid::new();
    grid.set_column_spacing(12);
    grid.set_row_spacing(4);
    
    for (row, (key, description)) in shortcuts.iter().enumerate() {
        let key_label = Label::new(Some(key));
        key_label.set_markup(&format!("<tt>{}</tt>", key));
        key_label.set_xalign(0.0);
        key_label.set_size_request(120, -1);
        
        let desc_label = Label::new(Some(description));
        desc_label.set_xalign(0.0);
        desc_label.set_hexpand(true); 
        
        grid.attach(&key_label, 0, row as i32, 1, 1);
        grid.attach(&desc_label, 1, row as i32, 1, 1);
    }
    
    shortcuts_box.add(&grid);
    shortcuts_box.show_all();
    shortcuts_box
}
