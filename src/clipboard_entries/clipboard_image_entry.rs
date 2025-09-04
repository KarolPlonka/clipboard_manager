use gtk::{ListBoxRow, Label, Image, Box as GTKBox, Orientation};
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
    image_path: String,
    uuid: String,
    row: ListBoxRow,
    more_info_box: GTKBox,
}

impl ClipboardImageEntry {
    const MARGIN: i32 = 10;

    pub fn new(
        image_path: String,
        uuid: String,
        row_width: i32,
        row_max_height: i32,
        more_info_width: i32,
        more_info_height: i32
    ) -> Self {
        let row = Self::create_entry_row(&image_path, row_width, row_max_height, Self::MARGIN);
        let more_info_box = GTKBox::new(Orientation::Vertical, 0);
        if let Some(more_info_image) = Self::create_more_info_image(&image_path, more_info_width, more_info_height, Self::MARGIN) {
            Self::pack_image_to_more_info_box(&more_info_box, &more_info_image, Self::MARGIN);
        }
        Self {image_path, uuid, row, more_info_box}
    }

    fn scale_pixbuf(pixbuf: &Pixbuf, max_width: i32, max_height: i32) -> Option<Pixbuf> {
        let orig_width = pixbuf.width() as f64;
        let orig_height = pixbuf.height() as f64;
        let image_scale = orig_width / orig_height;
        let canvas_height = max_height as f64;
        let canvas_width = max_width as f64;
        let canvas_scale = canvas_width / canvas_height;
        
        let (new_width, new_height) = if image_scale > canvas_scale {
            (canvas_width, canvas_width / image_scale)
        } else {
            (canvas_height * image_scale, canvas_height)
        };

        let scaled_pixbuf = pixbuf.scale_simple(
            new_width as i32,
            new_height as i32,
            gtk::gdk_pixbuf::InterpType::Bilinear
        );

        return scaled_pixbuf;
    }

    fn create_entry_row(image_path: &String, width: i32, max_height: i32, margin: i32) -> ListBoxRow {
        let row = ListBoxRow::new();
        
        let hbox = GTKBox::new(Orientation::Horizontal, margin);
        hbox.set_margin_start(margin);
        hbox.set_margin_end(margin);
        hbox.set_margin_top(margin);
        hbox.set_margin_bottom(margin);

        let (image, dimensions) = (|| {
            let pixbuf = Pixbuf::from_file(image_path).ok()?;
            let scaled_pixbuf = Self::scale_pixbuf(&pixbuf, width - (2 * margin), max_height - (2 * margin))?;
            let img = Image::from_pixbuf(Some(&scaled_pixbuf));
            Some((img, Some((pixbuf.width() as u32, pixbuf.height() as u32))))
        })().unwrap_or_else(|| {
            (Image::from_icon_name(Some("image-missing"), gtk::IconSize::Dialog), None)
        });

        let info_vbox = GTKBox::new(Orientation::Vertical, 2);
        info_vbox.set_halign(gtk::Align::Start);
        
        let path_obj = Path::new(image_path);
        let extension = path_obj.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown");
        
        let size_str = match fs::metadata(image_path) {
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
            _ => "Unknown dimensions".to_string()
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
        return row;
    }

    fn create_more_info_image(path: &String, width: i32, height: i32, margin: i32) -> Option<Image> {
        let available_width = width - (margin * 2);
        let available_height = height - (margin * 2);

        let pixbuf = Pixbuf::from_file(path).ok()?;
        let scaled_pixbuf = Self::scale_pixbuf(&pixbuf, available_width, available_height)?;
        let image = Image::from_pixbuf(Some(&scaled_pixbuf));
        Some(image)
    }

    fn pack_image_to_more_info_box(more_info_box: &GTKBox, image: &Image, margin: i32) {
        for child in more_info_box.children() {
            more_info_box.remove(&child);
        }
        more_info_box.set_margin_top(margin);
        more_info_box.set_margin_bottom(margin);
        more_info_box.pack_start(image, true, true, 0);
    }
}

impl ClipboardEntry for ClipboardImageEntry {
    fn get_row(&self) -> ListBoxRow {
        self.row.clone()
    }

    fn get_more_info_widget(&self, _search_query: Option<String>) -> gtk::Widget {
        self.more_info_box.clone().upcast::<gtk::Widget>()
    }

    fn set_more_info_widget_size(&self, width: i32, height: i32) {
        if let Some(new_image) = Self::create_more_info_image(&self.image_path, width, height, Self::MARGIN){
            Self::pack_image_to_more_info_box(&self.more_info_box, &new_image, Self::MARGIN);
        }
    }

    fn copy_to_clipboard(&self) -> Result<(), io::Error> {
        copy_to_clipboard_by_gpaste_uuid(&self.uuid)
    }


    fn open_in_external_app(&self) -> Result<(), io::Error> {
        open_in_external_app(&self.image_path)
    }
}
