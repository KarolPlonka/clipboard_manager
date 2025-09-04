use gtk::ListBoxRow;
use gtk::Widget;
use std::io;


pub trait ClipboardEntry {
    fn get_row(&self) -> ListBoxRow;
    fn copy_to_clipboard(&self) -> Result<(), io::Error>;
    fn get_more_info_widget(&self, search_query: Option<String>) -> Widget;
    fn set_more_info_widget_size(&self, width: i32, height: i32);
    fn open_in_external_app(&self) -> Result<(), io::Error>;
    fn contains_text(&self, _search_text: &String) -> bool {
        return false;
    }
    fn set_highlight_in_row(&self, _search_text: Option<String>) {}
}
