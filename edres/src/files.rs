//! TODO

use std::path::Path;

use edres_core::Error;

/// TODO
pub fn ensure_destination(path: &Path, create_dirs: bool) -> Result<(), Error> {
    if create_dirs {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
    }

    Ok(())
}

/// TODO
pub fn write_destination(
    destination: &Path,
    output: String,
    write_only_if_changed: bool,
) -> Result<(), Error> {
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
        std::fs::write(destination, output)?;
    }
    Ok(())
}
