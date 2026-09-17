use std::path::PathBuf;

use crate::error::{ScoopError, ScoopResult};

pub fn data_dir() -> ScoopResult<PathBuf> {
    let base = dirs::data_dir().ok_or_else(|| ScoopError::msg("No XDG data dir"))?;
    let dir = base.join("scoop");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn cache_dir() -> ScoopResult<PathBuf> {
    let base = dirs::cache_dir().ok_or_else(|| ScoopError::msg("No XDG cache dir"))?;
    let dir = base.join("scoop");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn captures_dir() -> ScoopResult<PathBuf> {
    let dir = cache_dir()?.join("captures");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Persisted selection screenshots (Library / Notes).
/// Stored in the user's Pictures/Screenshots folder.
pub fn screenshots_dir() -> ScoopResult<PathBuf> {
    let dir = dirs::picture_dir()
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Pictures")
        })
        .join("Screenshots");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn db_path() -> ScoopResult<PathBuf> {
    Ok(data_dir()?.join("scoop.db"))
}

/// App-managed Tesseract language data (`*.traineddata`).
pub fn tessdata_dir() -> ScoopResult<PathBuf> {
    let dir = data_dir()?.join("tessdata");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
