use std::process::Command;
use std::io;

use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
use clipboard_manager::clipboard_entries::clipboard_text_entry::ClipboardTextEntry;
use clipboard_manager::clipboard_entries::clipboard_image_entry::ClipboardImageEntry;


// TODO: Denest


fn create_image_entry(
    uuid: String,
    width: i32,
    row_max_height: i32,
) -> Result<Box<dyn ClipboardEntry>, io::Error> {
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

    let image_entry = ClipboardImageEntry::new(
        path,
        uuid,
        width,
        row_max_height,
    );

    Ok(Box::new(image_entry) as Box<dyn ClipboardEntry>)
}

pub fn get_clipboard_entries(
    limit: usize,
    row_width: i32,
    row_image_max_height: i32,
    row_text_max_lines: i32,
) -> Result<Vec<Box<dyn ClipboardEntry>>, io::Error> {
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
                    match create_image_entry(
                        uuid.clone(),
                        row_width,
                        row_image_max_height,
                    ) {
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
                    let entry = ClipboardTextEntry::new(content, uuid, row_width, row_text_max_lines);
                    Some(Box::new(entry) as Box<dyn ClipboardEntry>)
                }
            } else {
                eprintln!("Invalid clipboard entry format: {}", line);
                None
            }
        })
        .collect())
}
