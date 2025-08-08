use gtk::{ListBoxRow, Label, Widget};
use gtk::prelude::*;
use super::clipboard_entry::ClipboardEntry;
use crate::copy_to_clipboard_by_gpaste_uuid;
use std::io;

#[derive(Debug, Clone)]
pub struct ClipboardTextEntry {
    full_content: String,
    shorten_content: String,
    uuid: String,
}

impl ClipboardTextEntry {
    pub fn new(full_content: String, uuid: String) -> Self {
        let shorten_content = Self::create_shorten_content(&full_content);
        return Self { full_content, shorten_content, uuid };
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
            
            // if line.len() > 20 {
            //     let truncated = format!("{}...", &line[..17]); // 47 chars + 3 dots = 50
            //     result.push(truncated);
            // } else {
            //     result.push(line.to_string());
            // }
        }
        
        return result.join("\n");
    }
}

// Implement the trait for ClipboardTextEntry
impl ClipboardEntry for ClipboardTextEntry {
    fn get_entry_row(&self, width: i32) -> ListBoxRow {
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

    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }

    fn get_more_info(&self, width: i32, height: i32) -> Widget {
        let label = Label::new(Some(&self.full_content));
        // let label = Label::new(Some("test"));
        // println!("Hello, world!");

        label.set_xalign(0.0);
        label.set_margin_start(10);
        label.set_margin_end(10);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_width_chars(width);

        // label.set_height_chars(height);
        return label.upcast::<Widget>();
    }
}

