use std::io::{self, Write};
use std::process::Command;
use tempfile::NamedTempFile;

pub mod clipboard_entries;

pub fn copy_to_clipboard_by_gpaste_uuid(uuid: &str) -> Result<(), io::Error> {
    let output = Command::new("gpaste-client")
        .args(&["select", uuid])
        .output()?;    

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("gpaste-client command failed with status: {}", output.status)
        ));
    }
    
    Ok(())
}

pub fn save_to_tmp_file(content: &str) -> Result<String, io::Error> {
    let mut tmp_file = NamedTempFile::new()?;
    tmp_file.write_all(content.as_bytes())?;
    tmp_file.flush()?;
    
    let path = tmp_file.path().to_path_buf();
    
    let _file = tmp_file.persist(&path)?;
    
    Ok(path.to_string_lossy().to_string())
}

pub fn open_in_external_app(file_path: &str) -> Result<(), io::Error> {
    let output = Command::new("xdg-open")
        .arg(file_path)
        .output()?;
    
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("xdg-open command failed with status: {}", output.status)
        ));
    }
    
    Ok(())
}
