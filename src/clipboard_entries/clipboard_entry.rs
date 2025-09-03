use gtk::ListBoxRow;
use gtk::Widget;
use std::io;


pub trait ClipboardEntry {
    fn create_entry_row(&self, width: i32) -> ListBoxRow;
    fn copy_to_clipboard(&self) -> Result<(), io::Error>;
    fn get_more_info(&self, width: i32, height: i32, search_query: Option<String>) -> Widget;
    fn open_in_external_app(&self) -> Result<(), io::Error>;
    fn contains_text(&self, _search_text: &String) -> bool {
        return false;
    }
}
