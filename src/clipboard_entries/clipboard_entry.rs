use gtk::ListBoxRow;
use gtk::Widget;
use std::io;

pub trait ClipboardEntry {
    fn get_entry_row(&self, width: i32) -> ListBoxRow;
    fn copy_to_clipboard(&self) -> Result<(), io::Error>;
    fn get_more_info(&self, width: i32, height: i32) -> Widget;
}
