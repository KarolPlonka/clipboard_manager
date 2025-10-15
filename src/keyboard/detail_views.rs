use gtk::{prelude::*, ApplicationWindow, Box as GTKBox, Inhibit, ListBox, ScrolledWindow};

use crate::{
    constants::*,
    ui::{AppState, DetailsVisibility},
};

fn get_current_entry<'a>(
    list_box: &ListBox,
    app_state: &'a AppState,
) -> Option<std::cell::Ref<'a, Box<dyn clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry>>> {
    let selected_row = list_box.selected_row()?;
    let map = app_state.row_to_entry_map.borrow();
    
    if map.contains_key(&selected_row) {
        Some(std::cell::Ref::map(map, |m| m.get(&selected_row).unwrap()))
    } else {
        None
    }
}

pub fn hide_detail(
    window: &ApplicationWindow,
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    app_state: &AppState,
) -> Inhibit {
    main_box.remove(detail_scrolled_window);
    window.resize(ENTRIES_WIDTH, APP_HEIGHT);
    *app_state.details_visibility.borrow_mut() = DetailsVisibility::Hidden;
    Inhibit(true)
}

pub fn show_normal_detail(
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    *app_state.details_visibility.borrow_mut() = DetailsVisibility::Normal;
    
    if detail_scrolled_window.parent().is_none() {
        detail_scrolled_window.set_size_request(INFO_BOX_WIDTH, APP_HEIGHT);
        main_box.pack_start(detail_scrolled_window, true, true, 0);
    } else {
        detail_scrolled_window.set_size_request(INFO_BOX_WIDTH, APP_HEIGHT);
    }
    
    if let Some(entry) = get_current_entry(list_box, app_state) {
        for child in detail_container.children() {
            detail_container.remove(&child);
        }
        let detail_widget = entry.create_more_info_widget(
            INFO_BOX_WIDTH,
            APP_HEIGHT,
            app_state.search_query.borrow().clone()
        );
        detail_container.add(&detail_widget);
        detail_container.show_all();
    }
    
    detail_scrolled_window.show_all();
    Inhibit(true)
}

pub fn show_big_detail(
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    *app_state.details_visibility.borrow_mut() = DetailsVisibility::Big;
    
    if detail_scrolled_window.parent().is_none() {
        detail_scrolled_window.set_size_request(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT);
        main_box.pack_start(detail_scrolled_window, true, true, 0);
    } else {
        detail_scrolled_window.set_size_request(INFO_BOX_BIG_WIDTH, INFO_BOX_BIG_HEIGHT);
    }
    
    if let Some(entry) = get_current_entry(list_box, app_state) {
        for child in detail_container.children() {
            detail_container.remove(&child);
        }
        let detail_widget = entry.create_more_info_widget(
            INFO_BOX_BIG_WIDTH,
            INFO_BOX_BIG_HEIGHT,
            app_state.search_query.borrow().clone()
        );
        detail_container.add(&detail_widget);
        detail_container.show_all();
    }
    
    detail_scrolled_window.show_all();
    Inhibit(true)
}

pub fn toggle_detail(
    window: &ApplicationWindow,
    main_box: &GTKBox,
    detail_scrolled_window: &ScrolledWindow,
    detail_container: &GTKBox,
    list_box: &ListBox,
    app_state: &AppState,
) -> Inhibit {
    let next_visibility = {
        let current = app_state.details_visibility.borrow();
        match *current {
            DetailsVisibility::Hidden => DetailsVisibility::Normal,
            DetailsVisibility::Normal => DetailsVisibility::Big,
            DetailsVisibility::Big => DetailsVisibility::Hidden,
        }
    };
    
    match next_visibility {
        DetailsVisibility::Hidden => {
            hide_detail(window, main_box, detail_scrolled_window, app_state);
        },
        DetailsVisibility::Normal => {
            show_normal_detail(main_box, detail_scrolled_window, detail_container, list_box, app_state);
        },
        DetailsVisibility::Big => {
            show_big_detail(main_box, detail_scrolled_window, detail_container, list_box, app_state);
        },
    }
    
    Inhibit(true)
}