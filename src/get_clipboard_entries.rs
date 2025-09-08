use std::process::Command;
use std::io;

use clipboard_manager::clipboard_entries::clipboard_entry::ClipboardEntry;
use clipboard_manager::clipboard_entries::clipboard_text_entry::ClipboardTextEntry;
use clipboard_manager::clipboard_entries::clipboard_image_entry::ClipboardImageEntry;
use clipboard_manager::clipboard_entries::clipboard_file_entry::ClipboardFileEntry;

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

fn is_image(content: &str) -> bool {
    if content.starts_with(" [Image,") {
        return true;
    }

    if !content.starts_with(" [Files] ") {
        return false;
    }

    let path = content.trim_start_matches(" [Files] ").trim();
    let path_obj = std::path::Path::new(path);
    
    path_obj
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn execute_gpaste_command(args: &[&str]) -> Result<String, io::Error> {
    let output = Command::new("gpaste-client")
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("gpaste-client command failed with status: {}", output.status)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_entry_path(uuid: &str) -> Result<String, io::Error> {
    execute_gpaste_command(&["--raw", "get", uuid])
}

fn create_image_entry(
    uuid: String,
    width: i32,
    row_max_height: i32,
) -> Result<Box<dyn ClipboardEntry>, io::Error> {
    let path = get_entry_path(&uuid)?;
    let image_entry = ClipboardImageEntry::new(path, uuid, width, row_max_height);
    Ok(Box::new(image_entry))
}

fn create_file_entry(
    uuid: String,
    width: i32,
    row_max_height: i32,
) -> Result<Box<dyn ClipboardEntry>, io::Error> {
    let path = get_entry_path(&uuid)?;
    let file_entry = ClipboardFileEntry::new(path, uuid, width, row_max_height);
    Ok(Box::new(file_entry))
}

fn create_text_entry(
    content: String,
    uuid: String,
    width: i32,
    row_text_max_lines: i32,
) -> Box<dyn ClipboardEntry> {
    Box::new(ClipboardTextEntry::new(content, uuid, width, row_text_max_lines))
}

fn parse_clipboard_line(line: &str) -> Option<(String, String)> {
    let colon_pos = line.find(':')?;
    let uuid = line[..colon_pos].to_string();
    let content = line[colon_pos + 1..].to_string();
    Some((uuid, content))
}

fn create_clipboard_entry(
    uuid: String,
    content: String,
    row_width: i32,
    row_image_max_height: i32,
    row_text_max_lines: i32,
) -> Option<Box<dyn ClipboardEntry>> {
    if content.is_empty() {
        return None;
    }

    if is_image(&content) {
        create_image_entry(uuid.clone(), row_width, row_image_max_height)
            .map_err(|e| eprintln!("Error creating image entry for UUID {}: {}", uuid, e))
            .ok()
    } else if content.starts_with(" [Files] ") {
        create_file_entry(uuid.clone(), row_width, row_image_max_height)
            .map_err(|e| eprintln!("Error creating file entry for UUID {}: {}", uuid, e))
            .ok()
    } else {
        Some(create_text_entry(content, uuid, row_width, row_text_max_lines))
    }
}

pub fn get_clipboard_entries(
    limit: usize,
    row_width: i32,
    row_image_max_height: i32,
    row_text_max_lines: i32,
) -> Result<Vec<Box<dyn ClipboardEntry>>, io::Error> {
    let history = execute_gpaste_command(&["history", "--zero"])?;
    
    Ok(history
        .split('\0')
        .filter(|s| !s.is_empty())
        .take(limit)
        .filter_map(|line| {
            match parse_clipboard_line(line) {
                Some((uuid, content)) => create_clipboard_entry(
                    uuid,
                    content,
                    row_width,
                    row_image_max_height,
                    row_text_max_lines,
                ),
                _ => {
                    eprintln!("Invalid clipboard entry format: {}", line);
                    None
                }
            }
        })
        .collect()
    )
}
