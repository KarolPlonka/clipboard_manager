use gtk::ListBoxRow;
use std::io;

pub trait ClipboardEntry {
    fn get_entry_row(&self) -> ListBoxRow;
    fn copy_to_clipboard(&self) -> Result<(), io::Error>;
}
