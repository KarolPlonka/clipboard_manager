use gtk::{ListBoxRow, Label, Widget, glib};
use gtk::prelude::*;
use super::clipboard_entry::ClipboardEntry;
use crate::open_in_external_app;
use crate::save_to_tmp_file;
use crate::copy_to_clipboard_by_gpaste_uuid;
use std::io;
use glib::markup_escape_text;

#[derive(Debug, Clone)]
pub struct ClipboardTextEntry {
    full_content: String,
    shorten_content: String,
    uuid: String,
    row: ListBoxRow,
    row_label: Label,
    more_info_label: Label,
}

impl ClipboardTextEntry {
    const MARGIN: i32 = 10;
    const HIGHLIGHT_FORMAT: &'static str = "<span background='#E95420' foreground='white' weight='bold'>%s</span>";
    const HIGHLIGHT_FORMAT_SELECTED: &'static str = "<span background='#333' foreground='white' weight='bold'>%s</span>";

    pub fn new(full_content: String, uuid: String, row_width: i32, row_max_lines: i32, more_info_width: i32) -> Self {
        let shorten_content = Self::create_shorten_content(&full_content, row_max_lines as usize);
        let (row, row_label) = Self::create_entry_row(&shorten_content, row_width);
        let more_info_label = Self::create_more_info_label(&full_content, more_info_width);
        return Self{full_content, shorten_content, uuid, row, row_label, more_info_label}
    }

    fn create_entry_row(text: &String, width: i32) -> (ListBoxRow, Label) {
        let label = Label::new(Some(text));
        label.set_xalign(0.0);
        label.set_margin_start(Self::MARGIN);
        label.set_margin_end(Self::MARGIN);
        label.set_margin_top(Self::MARGIN);
        label.set_margin_bottom(Self::MARGIN); label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_size_request(width - (2 * Self::MARGIN), -1);
        let row = ListBoxRow::new();
        row.add(&label);
        row.set_size_request(width, -1);
        return (row, label);
    }

    fn create_more_info_label(text: &String, width: i32) -> Label {
        let label = Label::new(Some(text));
        
        label.set_xalign(0.0);
        label.set_margin_start(10);
        label.set_margin_end(10);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_width_chars(width);
        
        return label;
    }

    fn create_shorten_content(content: &str, max_lines: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        
        for (i, line) in lines.iter().enumerate() {
            if i >= max_lines {
                if i == max_lines {
                    result.push("...".to_string());
                }
                break;
            }
            result.push(line.to_string());
        }
        
        return result.join("\n");
    }

    fn highlight_text(&self, text: &str, query: &str, format: &str) -> Option<String> {
        if query.is_empty() {
            return None;
        }
        
        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();
        
        let mut result = String::new();
        let mut last_end = 0;
        
        let mut search_start = 0;
        while let Some(match_start) = text_lower[search_start..].find(&query_lower) {
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
        
        if last_end < text.len() {
            result.push_str(&markup_escape_text(&text[last_end..]));
        }
        
        Some(result)
    }
}

impl ClipboardEntry for ClipboardTextEntry {
    fn get_row(&self) -> ListBoxRow {
        return self.row.clone();
    }

    fn get_more_info_widget(&self, search_query: Option<String>) -> Widget {
        let label = self.more_info_label.clone();

        let highlighted_content = search_query
            .filter(|query| !query.is_empty())
            .and_then(|query| {
                self.highlight_text(&self.shorten_content, &query, Self::HIGHLIGHT_FORMAT)
            });

        match highlighted_content {
            Some(markup) => label.set_markup(&markup),
            _ => label.set_text(&self.shorten_content),
        }
        
        label.show_all();
        label.upcast::<Widget>()
    }

    fn set_more_info_widget_size(&self, width: i32, _height: i32) {
        self.more_info_label.set_width_chars(width);
    }

    fn contains_text(&self, search_text: &String) -> bool {
        return self.full_content.to_lowercase().contains(&search_text.to_lowercase());
    }

    fn set_highlight_in_row(&self, search_query: Option<String>) {
        let highlighted_content = search_query
            .filter(|query| !query.is_empty())
            .and_then(|query| {
                let format = if self.row.is_selected() {
                    Self::HIGHLIGHT_FORMAT_SELECTED
                } else {
                    Self::HIGHLIGHT_FORMAT
                };
                self.highlight_text(&self.shorten_content, &query, format)
            });

        match highlighted_content {
            Some(markup) => self.row_label.set_markup(&markup),
            _ => self.row_label.set_text(&self.shorten_content),
        }
    }

    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }

    fn open_in_external_app(&self) -> Result<(), io::Error> {
        let file_path = save_to_tmp_file(&self.full_content)?;
        open_in_external_app(&file_path)
    }

}


