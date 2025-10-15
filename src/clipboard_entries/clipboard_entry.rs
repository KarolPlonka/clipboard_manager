use gtk::ListBoxRow;
use std::io;


pub trait ClipboardEntry {
    fn get_row(&self) -> ListBoxRow;
    fn copy_to_clipboard(&self, copy_path: bool) -> Result<(), io::Error>;
    fn create_more_info_widget(&self, width: i32, height: i32, _search_query: Option<String>) -> gtk::Widget;
    fn open_in_external_app(&self) -> Result<(), io::Error>;
    fn contains_text(&self, _search_text: &String) -> bool {
        return false;
    }
    fn set_highlight_in_row(&self, _search_text: Option<String>) {}
}
