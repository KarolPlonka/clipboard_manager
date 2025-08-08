use gtk::{ListBoxRow, Label};
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
            
            if line.len() > 100 {
                let truncated = format!("{}...", &line[..47]); // 47 chars + 3 dots = 50
                result.push(truncated);
            } else {
                result.push(line.to_string());
            }
        }
        
        return result.join("\n");
    }
}

// Implement the trait for ClipboardTextEntry
impl ClipboardEntry for ClipboardTextEntry {
    fn get_entry_row(&self) -> ListBoxRow {
        let row = ListBoxRow::new();
        let label = Label::new(Some(&self.shorten_content));
        label.set_xalign(0.0);
        label.set_margin_start(10);
        label.set_margin_end(10);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        row.add(&label);
        return row;
    }
    
    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }
}

