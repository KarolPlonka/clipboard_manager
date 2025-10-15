use gtk::{glib::markup_escape_text, prelude::*, Label, ListBoxRow, Widget};
use std::io;

use crate::{copy_to_clipboard_by_gpaste_uuid, open_in_external_app, save_to_tmp_file};

use super::clipboard_entry::ClipboardEntry;

#[derive(Debug, Clone)]
pub struct ClipboardTextEntry {
    full_content: String,
    shorten_content: Option<String>,
    uuid: String,
    row: ListBoxRow,
    row_label: Label,
}

impl ClipboardTextEntry {
    const MARGIN: i32 = 10;
    const HIGHLIGHT_FORMAT: &'static str = "<span background='#E95420' foreground='white' weight='bold'>%s</span>";
    const HIGHLIGHT_FORMAT_SELECTED: &'static str = "<span background='#333' foreground='white' weight='bold'>%s</span>";

    pub fn new(full_content: String, uuid: String, row_width: i32, row_max_lines: i32) -> Self {
        let shorten_content = Self::create_shorten_content(&full_content, row_max_lines as usize);
        let entry_row_content = match &shorten_content {
            Some(shortened) => format!("{}\n...", shortened),
            _ => full_content.clone(),
        };
        let (row, row_label) = Self::create_entry_row(&entry_row_content, row_width);
        return Self { full_content, shorten_content, uuid, row, row_label }
    }

    fn create_entry_row(text: &String, width: i32) -> (ListBoxRow, Label) {
        let label = Label::new(Some(text));
        label.set_xalign(0.0);
        label.set_margin(Self::MARGIN);
        label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        label.set_size_request(width - (2 * Self::MARGIN), -1);

        let row = ListBoxRow::new();
        row.add(&label);
        row.set_size_request(width, -1);

        return (row, label);
    }

    fn create_shorten_content(content: &str, max_lines: usize) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.len() <= max_lines {
            None
        } else {
            let shortened: Vec<_> = lines.iter()
                .take(max_lines)
                .map(|&line| line.to_string())
                .collect();
            Some(shortened.join("\n"))
        }
    }

    fn highlight_in_text(text: &str, query: &str, format: &str) -> Option<String> {
        if query.is_empty() {
            return None;
        }
        
        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();
        
        let mut result = String::new();
        let mut last_end = 0;
        let mut found_match = false; 
        
        let mut search_start = 0;
        while let Some(match_start) = text_lower[search_start..].find(&query_lower) {
            found_match = true; 
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
            return None; 
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

    fn create_more_info_widget(&self, _width: i32, _height: i32, search_query: Option<String>) -> gtk::Widget {
        let label = Label::new(None);
        label.set_margin(Self::MARGIN);
        label.set_xalign(0.0);

        match search_query.and_then(|query| ClipboardTextEntry::highlight_in_text(&self.full_content, &query, Self::HIGHLIGHT_FORMAT)) {
            Some(markup) => label.set_markup(&markup),
            _ => label.set_text(&self.full_content),
        }        

        label.upcast::<Widget>()
    }

    fn contains_text(&self, search_text: &String) -> bool {
        return self.full_content.to_lowercase().contains(&search_text.to_lowercase());
    }

    fn set_highlight_in_row(&self, search_query: Option<String>) {
        let (content, is_shortened) = match &self.shorten_content {
            Some(shortened) => (shortened, true),
            _ => (&self.full_content, false),
        };
        
        match search_query.filter(|query| !query.is_empty()) {
            Some(query) => {
                let format = if self.row.is_selected() {
                    Self::HIGHLIGHT_FORMAT_SELECTED
                } else {
                    Self::HIGHLIGHT_FORMAT
                };
                
                let markup = if let Some(highlighted) = ClipboardTextEntry::highlight_in_text(content, &query, format) {
                    if is_shortened {
                        format!("{}\n...", highlighted)
                    } else {
                        highlighted
                    }
                } else {
                    let highlighted_dots = format.replace("%s", "...");
                    format!("{}{}\n{}", 
                        markup_escape_text(content),
                        if is_shortened { "..." } else { "" },
                        highlighted_dots
                    )
                };
                
                self.row_label.set_markup(&markup);
            }
            _ => {
                let display_text = if is_shortened {
                    format!("{}\n...", content)
                } else {
                    content.to_string()
                };
                self.row_label.set_text(&display_text);
            }
        }
    }

    fn copy_to_clipboard(&self, copy_path: bool) -> Result<(), io::Error> {
        if copy_path {
            return Ok(());
        }
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }

    fn open_in_external_app(&self) -> Result<(), io::Error> {
        let file_path = save_to_tmp_file(&self.full_content)?;
        open_in_external_app(&file_path)
    }

}


