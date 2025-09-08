use gtk::{ListBoxRow, Label, Widget, glib, Image, Box as GTKBox, Orientation};
use gtk::prelude::*;
use super::clipboard_entry::ClipboardEntry;
use crate::open_in_external_app;
use crate::copy_to_clipboard_by_gpaste_uuid;
use std::io;
// use std::path::Path;
use glib::markup_escape_text;
use std::fs;

#[derive(Debug, Clone)]
pub struct ClipboardFileEntry {
    file_path: String,
    uuid: String,
    row: ListBoxRow,
    row_label: Label,
    file_content: Result<String, String>,
}

impl ClipboardFileEntry {
    const MARGIN: i32 = 10;
    const ICON_SIZE: i32 = 32;
    const HIGHLIGHT_FORMAT: &'static str = "<span background='#E95420' foreground='white' weight='bold'>%s</span>";
    const HIGHLIGHT_FORMAT_SELECTED: &'static str = "<span background='#333' foreground='white' weight='bold'>%s</span>";

    pub fn new(file_path: String, uuid: String, row_width: i32, max_row_height: i32) -> Self {
        let row_height = if Self::ICON_SIZE + (2 * Self::MARGIN) > max_row_height {
            max_row_height
        } else {
            Self::ICON_SIZE + (2 * Self::MARGIN)
        };
        let (row, row_label) = Self::create_entry_row(&file_path, row_width, row_height);
        let file_content = Self::read_file_content(&file_path);
        return Self {file_path, uuid, row, row_label, file_content}
    }

    fn create_entry_row(text: &String, width: i32, height: i32) -> (ListBoxRow, Label) {
        let label = Label::new(Some(text));
        label.set_xalign(0.0);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let icon = Image::from_icon_name(Some("text-x-generic"), gtk::IconSize::LargeToolbar);
        icon.set_pixel_size(height - (2 * Self::MARGIN));

        let row_box = GTKBox::new(Orientation::Horizontal, Self::MARGIN);
        row_box.set_margin(Self::MARGIN);
        row_box.pack_start(&icon, false, false, 0);
        row_box.pack_start(&label, true, true, 0);

        let row = ListBoxRow::new();
        row.set_size_request(width, -1);
        row.add(&row_box);

        return (row, label);
    }

    fn highlight_in_text(text: &str, query: &str, format: &str) -> Option<String> {
        if query.is_empty() {
            return None;
        }
        
        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();
        
        let mut result = String::new();
        let mut last_end = 0;
        let mut found_match = false; // Track if we found any matches
        
        let mut search_start = 0;
        while let Some(match_start) = text_lower[search_start..].find(&query_lower) {
            found_match = true; // We found at least one match
            let absolute_start = search_start + match_start;
            let absolute_end = absolute_start + query.len();
            
            if absolute_start > last_end {
                result.push_str(&markup_escape_text(&text[last_end..absolute_start]));
            }
            
            let matched_text = &text[absolute_start..absolute_end];
            let highlighted = format.replace("%s", &markup_escape_text(matched_text));
            result.push_str(&highlighted);
            
            last_end = absolute_end;
            search_start = absolute_end;
        }
        
        if !found_match {
            return None; // No matches found, return None
        }
        
        if last_end < text.len() {
            result.push_str(&markup_escape_text(&text[last_end..]));
        }
        
        Some(result)
    }


    fn read_file_content(file_path: &String) -> Result<String, String> {
        match fs::read_to_string(file_path) {
            Ok(content) => Ok(content),
            Err(err) => Err(format!("Error reading file: {}", err)),
        }
    }
}

impl ClipboardEntry for ClipboardFileEntry {
    fn get_row(&self) -> ListBoxRow {
        return self.row.clone();
    }

    fn create_more_info_widget(&self, _width: i32, _height: i32, _search_query: Option<String>) -> gtk::Widget {
        let (content, is_error) = match &self.file_content {
            Ok(text) => (text.clone(), false),
            Err(err_msg) => (err_msg.clone(), true),
        };

        let label = Label::new(Some(&content));
        label.set_xalign(0.0);
        label.set_margin(Self::MARGIN);
        if is_error {
            label.style_context().add_class("error");
        } 

        label.upcast::<Widget>()
    }

    fn contains_text(&self, search_text: &String) -> bool {
        return self.file_path.to_lowercase().contains(&search_text.to_lowercase());
    }

    fn set_highlight_in_row(&self, search_query: Option<String>) {
        match search_query.filter(|query| !query.is_empty()) {
            Some(query) => {
                let format = if self.row.is_selected() {
                    Self::HIGHLIGHT_FORMAT_SELECTED
                } else {
                    Self::HIGHLIGHT_FORMAT
                };
                
                if let Some(highlighted) = ClipboardFileEntry::highlight_in_text(&self.file_path, &query, format) {
                    self.row_label.set_markup(&highlighted);
                } else {
                    self.row_label.set_text(&self.file_path);
                }
            }
            _ => {
                self.row_label.set_text(&self.file_path);
            }
        }
    }

    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }

    fn open_in_external_app(&self) -> Result<(), io::Error> {
        open_in_external_app(&self.file_path)
    }

}



