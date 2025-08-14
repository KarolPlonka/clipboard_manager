use gtk::{ListBoxRow, Label, Image, Box, Orientation};
use gtk::prelude::*;
use gtk::gdk_pixbuf::Pixbuf;
use super::clipboard_entry::ClipboardEntry;
use crate::copy_to_clipboard_by_gpaste_uuid;
use crate::open_in_external_app;
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
    fn get_entry_row(&self, width: i32) -> ListBoxRow {
        let margin = 10;
        let row = ListBoxRow::new();
        
        // Create a horizontal box to contain the image
        let hbox = Box::new(Orientation::Horizontal, 5);
        hbox.set_margin_start(margin);
        hbox.set_margin_end(margin);
        hbox.set_margin_top(margin);
        hbox.set_margin_bottom(margin);
        
        // Try to load and display the image
        let (image, dimensions) = match Pixbuf::from_file(&self.path) {
            Ok(pixbuf) => {
                // Get original dimensions
                let orig_width = pixbuf.width();
                let orig_height = pixbuf.height();
                
                // Calculate available width for image (accounting for margins and spacing)
                // Subtract margins (left + right), spacing, and leave some room for info text
                let available_width = width - (margin * 2) - 5 - 200; // 200px reserved for text info
                let max_size = available_width.max(64).min(256); // Clamp between 64 and 256 pixels
                
                // Scale the image to fit within available width while preserving aspect ratio
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

    fn get_more_info(&self, width: i32, height: i32) -> gtk::Widget {
        let margin = 10;
        // Account for both left/right and top/bottom margins
        let available_width = width - (margin * 2);
        let available_height = height - (margin * 2);
        
        let image = match Pixbuf::from_file(&self.path) {
            Ok(pixbuf) => {
                let original_width = pixbuf.width() as f64;
                let original_height = pixbuf.height() as f64;
                
                // Calculate scale factors for both dimensions
                let width_scale = available_width as f64 / original_width;
                let height_scale = available_height as f64 / original_height;
                
                // Use the smaller scale factor to ensure image fits in both dimensions
                let scale = width_scale.min(height_scale);
                
                let scaled_width = (original_width * scale) as i32;
                let scaled_height = (original_height * scale) as i32;
                
                let scaled_pixbuf = pixbuf.scale_simple(
                    scaled_width,
                    scaled_height,
                    gtk::gdk_pixbuf::InterpType::Bilinear
                ).unwrap_or(pixbuf);
                
                Image::from_pixbuf(Some(&scaled_pixbuf))
            }
            Err(_) => Image::from_icon_name(Some("image-missing"), gtk::IconSize::Dialog),
        };
        
        let container = Box::new(Orientation::Vertical, 0);
        container.set_margin_top(margin);
        container.set_margin_bottom(margin);
        container.pack_start(&image, true, true, 0);
        container.upcast::<gtk::Widget>()
    }

    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }


    fn open_in_external_app(&self) -> Result<(), io::Error> {
        open_in_external_app(&self.path)
    }
}
