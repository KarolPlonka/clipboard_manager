use std::io;
use std::process::Command;

pub mod clipboard_entries;

pub fn copy_to_clipboard_by_gpaste_uuid(uuid: &str) -> Result<(), io::Error> {
    let mut command = Command::new("gpaste-client");
    command.args(&["select", uuid]);
    let output = command.output()?;
    
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("gpaste-client command failed with status: {}", output.status)
        ));
    }
    
    Ok(())
}
