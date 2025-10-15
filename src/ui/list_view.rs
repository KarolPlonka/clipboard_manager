use gtk::{prelude::*, Box as GTKBox, ListBox, ListBoxRow, ScrolledWindow};
use std::{collections::HashMap, rc::Rc};

use clipboard_manager::clipboard_entries::ClipboardEntry;
use crate::{constants::*, ui::app_state::{AppState, DetailsVisibility}};


pub fn append_to_list_view(
    list_box: &ListBox,
    entries: Vec<Box<dyn ClipboardEntry>>,
) -> (Vec<ListBoxRow>, HashMap<ListBoxRow, Box<dyn ClipboardEntry>>) {
    let rows: Vec<ListBoxRow> = entries
        .iter()
        .map(|entry| entry.get_row())
        .collect();
    
    let row_to_entry_map: HashMap<ListBoxRow, Box<dyn ClipboardEntry>> = rows
        .iter()
        .cloned()
        .zip(entries.into_iter())
        .collect();
    
    for row in &rows {
        list_box.add(row);
    }
    
    return (rows, row_to_entry_map);
}

pub fn create_list_view(
    entries: Vec<Box<dyn ClipboardEntry>>,
    width: i32,
) -> (ScrolledWindow, ListBox, Vec<ListBoxRow>, HashMap<ListBoxRow, Box<dyn ClipboardEntry>>) {
    let list_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(width)
        .build();
    
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);
    
    let (rows, row_to_entry_map) = append_to_list_view(&list_box, entries);
    
    list_scrolled_window.add(&list_box);
    
    (list_scrolled_window, list_box, rows, row_to_entry_map)
}

pub fn setup_list_selection_handler(
    list_box: &ListBox,
    detail_container: &GTKBox,
    app_state: Rc<AppState>,
) {
    let detail_container_clone = detail_container.clone();
    
    list_box.connect_row_selected(move |_, row| {
        if *app_state.details_visibility.borrow() == DetailsVisibility::Hidden {
            return;
        }

        for child in detail_container_clone.children() {
            detail_container_clone.remove(&child);
        }

        let Some(row) = row else {
            detail_container_clone.show_all();
            return;
        };

        let row_map = app_state.row_to_entry_map.borrow();

        let Some(entry) = row_map.get(&row) else {
            detail_container_clone.show_all();
            return;
        };

        let search_query = app_state.search_query.borrow();
        entry.set_highlight_in_row(search_query.clone());
        
        let (width, height) = match *app_state.details_visibility.borrow() {
            DetailsVisibility::Big => (INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT),
            DetailsVisibility::Normal => (INFO_BOX_WIDTH, APP_HEIGHT),
            _ => (INFO_BOX_WIDTH, APP_HEIGHT),
        };
        let detail_widget = entry.create_more_info_widget(width, height, search_query.clone());
        
        detail_container_clone.add(&detail_widget);
        detail_container_clone.show_all();
    });
}



