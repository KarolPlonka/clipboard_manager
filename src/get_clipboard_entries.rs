use std::process::Command;
use std::io;

use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
use clipboard_manager::clipboard_entries::clipboard_text_entry::ClipboardTextEntry;
use clipboard_manager::clipboard_entries::clipboard_image_entry::ClipboardImageEntry;


fn create_image_entry(uuid: String) -> Result<Box<dyn ClipboardEntry>, io::Error> {
    let output = Command::new("gpaste-client")
        .args(&["--raw", "get", &uuid])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("gpaste-client command failed with status: {}", output.status)
        ));
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(Box::new(ClipboardImageEntry::new(path, uuid)))
}

pub fn get_clipboard_entries(limit: usize) -> Result<Vec<Box<dyn ClipboardEntry>>, io::Error> {
    let output = Command::new("gpaste-client")
        .args(&["history", "--zero"])
        .output()?;
    
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("gpaste-client command failed with status: {}", output.status)
        ));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout)
        .split('\0')
        .filter(|s| !s.is_empty())
        .take(limit)  // Apply the limit here
        .filter_map(|line| {
            // Split at the first colon to separate UUID from content
            if let Some(colon_pos) = line.find(':') {
                let uuid = line[..colon_pos].to_string();
                let content = line[colon_pos + 1..].to_string();
                // Check if the content starts with "image/" to determine if it's an image entry
                if content.starts_with(" [Image,") {
                    match create_image_entry(uuid.clone()) {
                        Ok(entry) => Some(entry),
                        Err(e) => {
                            eprintln!("Error creating image entry for UUID {}: {}", uuid, e);
                            None
                        }
                    }
                } else if content.is_empty() {
                    // Skip empty entries
                    None
                } else {
                    Some(Box::new(ClipboardTextEntry::new(content, uuid)) as Box<dyn ClipboardEntry>)
                }
            } else {
                eprintln!("Invalid clipboard entry format: {}", line);
                None
            }
        })
        .collect())
}
