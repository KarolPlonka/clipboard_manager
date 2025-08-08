use gtk::{ListBoxRow, Label, Image, Box, Orientation};
use gtk::prelude::*;
use gtk::gdk_pixbuf::Pixbuf;
use super::clipboard_entry::ClipboardEntry;
use crate::copy_to_clipboard_by_gpaste_uuid;
use std::io;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone)]
pub struct ClipboardImageEntry {
    path: String,
    // thumbnail_path: String,
    uuid: String,
}

impl ClipboardImageEntry {
    pub fn new(path: String, uuid: String) -> Self {
        Self { path, uuid }
    }
}

impl ClipboardEntry for ClipboardImageEntry {
    fn get_entry_row(&self) -> ListBoxRow {
        let row = ListBoxRow::new();
        
        // Create a horizontal box to contain the image
        let hbox = Box::new(Orientation::Horizontal, 5);
        hbox.set_margin_start(10);
        hbox.set_margin_end(10);
        hbox.set_margin_top(8);
        hbox.set_margin_bottom(8);
        
        // Try to load and display the image
        let (image, dimensions) = match Pixbuf::from_file(&self.path) {
            Ok(pixbuf) => {
                // Get original dimensions
                let orig_width = pixbuf.width();
                let orig_height = pixbuf.height();
                
                // Scale the image to a thumbnail size while preserving aspect ratio
                let max_size = 128;
                let width = pixbuf.width();
                let height = pixbuf.height();
                
                let scale = if width > height {
                    max_size as f64 / width as f64
                } else {
                    max_size as f64 / height as f64
                };
                
                let new_width = (width as f64 * scale) as i32;
                let new_height = (height as f64 * scale) as i32;
                
                let scaled_pixbuf = pixbuf.scale_simple(
                    new_width,
                    new_height,
                    gtk::gdk_pixbuf::InterpType::Bilinear
                ).unwrap_or(pixbuf);
                
                let img = Image::from_pixbuf(Some(&scaled_pixbuf));
                (img, Some((orig_width, orig_height)))
            }
            Err(_) => {
                // If image loading fails, show a placeholder
                let img = Image::from_icon_name(Some("image-missing"), gtk::IconSize::Dialog);
                (img, None)
            }
        };
        
        // Create a vertical box for the text information
        let info_vbox = Box::new(Orientation::Vertical, 2);
        // Use set_halign instead of set_xalign for Box
        info_vbox.set_halign(gtk::Align::Start);
        
        // Get file information
        let path_obj = Path::new(&self.path);
        let extension = path_obj.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown");
        
        // Get file size
        let size_str = match fs::metadata(&self.path) {
            Ok(metadata) => {
                let size = metadata.len();
                if size < 1024 {
                    format!("{} B", size)
                } else if size < 1024 * 1024 {
                    format!("{:.1} KB", size as f64 / 1024.0)
                } else {
                    format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
                }
            }
            Err(_) => "Unknown size".to_string()
        };
        
        let size_label = Label::new(Some(&size_str));
        size_label.set_xalign(0.0);
        size_label.style_context().add_class("dim-label");
        
        let dimensions_label = Label::new(Some(&match dimensions {
            Some((w, h)) => format!("{}×{} px", w, h),
            None => "Unknown dimensions".to_string()
        }));
        dimensions_label.set_xalign(0.0);
        dimensions_label.style_context().add_class("dim-label");
        
        let ext_label = Label::new(Some(&format!(".{}", extension.to_uppercase())));
        ext_label.set_xalign(0.0);
        ext_label.style_context().add_class("dim-label");
        
        info_vbox.pack_start(&size_label, false, false, 0);
        info_vbox.pack_start(&dimensions_label, false, false, 0);
        info_vbox.pack_start(&ext_label, false, false, 0);
        
        hbox.pack_start(&image, false, false, 0);
        hbox.pack_start(&info_vbox, true, true, 0);
        
        row.add(&hbox);
        row.show_all();
        row
    }
    
    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        // Could have different implementation for images
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }
}
