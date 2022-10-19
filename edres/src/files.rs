//! Utility functions for working with output files.

use std::path::Path;

use edres_core::Error;

/// Utility function to create parent directories of a path.
///
/// The `create_dirs` parameter allows you to bypass this by
/// passing `false`.
pub fn ensure_destination(path: &Path, create_dirs: bool) -> Result<(), Error> {
    if create_dirs {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
    }

    Ok(())
}

/// Utility function to write output to a file, optionally only
/// if changed.
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
