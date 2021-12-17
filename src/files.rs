use std::path::Path;

use edres_core::WipError;

pub fn ensure_destination(path: &Path, create_dirs: bool) -> Result<(), WipError> {
    if create_dirs {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| WipError(e.to_string()))?;
        }
    }

    Ok(())
}

pub fn write_destination(
    destination: &Path,
    output: String,
    write_only_if_changed: bool,
) -> Result<(), WipError> {
    let should_write = if write_only_if_changed {
        let existing = std::fs::read_to_string(destination);
        match existing {
            Ok(existing) => existing != output,
            Err(_) => true,
        }
    } else {
        true
    };

    if should_write {
        std::fs::write(destination, output).map_err(|e| WipError(e.to_string()))
    } else {
        Ok(())
    }
}
