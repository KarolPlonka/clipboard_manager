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
}

impl ClipboardTextEntry {
    pub fn new(full_content: String, uuid: String) -> Self {
        let shorten_content = Self::create_shorten_content(&full_content);
        return Self{ full_content, shorten_content, uuid};
    }

    fn create_shorten_content(content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        
        for (i, line) in lines.iter().enumerate() {
            if i >= 2 {
                if i == 2 {
                    result.push("...".to_string());
                }
                break;
            }
            result.push(line.to_string());
        }
        
        return result.join("\n");
    }

    fn highlight_text(&self, text: &str, query: &str) -> String {
        let escaped_text = glib::markup_escape_text(text);
        
        let query_lower = query.to_lowercase();
        let mut result = String::new();
        let mut last_end = 0;
        
        for (start, _) in escaped_text.match_indices(|c: char| {
            query_lower.chars().next() == Some(c.to_lowercase().next().unwrap_or(c))
        }) {
            if let Some(end) = escaped_text[start..].to_lowercase().find(&query_lower) {
                if end == 0 {
                    result.push_str(&markup_escape_text(&escaped_text[last_end..start]));

                    let match_end = start + query.len();
                    result.push_str("<span background='#E95420' foreground='white' weight='bold'>");
                    result.push_str(&markup_escape_text(&escaped_text[start..match_end]));
                    result.push_str("</span>");

                    last_end = match_end;
                }
            }
        } 
        result.push_str(&escaped_text[last_end..]);
        
        result
    }
}

impl ClipboardEntry for ClipboardTextEntry {
    fn create_entry_row(&self, width: i32) -> ListBoxRow {
        let margin = 10; // Adjust margin as needed
        let row = ListBoxRow::new();
        let label = Label::new(Some(&self.shorten_content));
        label.set_xalign(0.0);
        label.set_margin_start(margin);
        label.set_margin_end(margin);
        label.set_margin_top(margin);
        label.set_margin_bottom(margin);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_size_request(width - (2 * margin), -1);
        row.add(&label);
        row.set_size_request(width, -1);
        return row;
    }

    fn contains_text(&self, search_text: &String) -> bool {
        return self.full_content.to_lowercase().contains(&search_text.to_lowercase());
    }

    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }

    fn get_more_info(&self, width: i32, _height: i32, search_query: Option<String>) -> Widget {
        let label = Label::new(None);


        match search_query {
            Some(query) if !query.is_empty() => {
                let highlighted_content = self.highlight_text(&self.full_content, &query);
                label.set_markup(&highlighted_content);
            }
            _ => {
                label.set_text(&self.full_content);
            }
        }
        
        label.set_xalign(0.0);
        label.set_margin_start(10);
        label.set_margin_end(10);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_width_chars(width);
        
        return label.upcast::<Widget>();
    }


    fn open_in_external_app(&self) -> Result<(), io::Error> {
        let file_path = save_to_tmp_file(&self.full_content)?;
        open_in_external_app(&file_path)
    }

}


