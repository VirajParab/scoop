use std::path::Path;

use arboard::{Clipboard, ImageData};
use image::ImageReader;

use crate::error::{ScoopError, ScoopResult};

pub fn copy_text(text: &str) -> ScoopResult<()> {
    let mut clipboard = Clipboard::new().map_err(|e| ScoopError::msg(e.to_string()))?;
    clipboard
        .set_text(text.to_string())
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    Ok(())
}

/// Copy a PNG/JPEG file to the system clipboard as an image (paste into docs, chat, etc.).
pub fn copy_image_file(path: &Path) -> ScoopResult<()> {
    let img = ImageReader::open(path)
        .map_err(|e| ScoopError::msg(format!("Open image for clipboard: {e}")))?
        .decode()
        .map_err(|e| ScoopError::msg(format!("Decode image for clipboard: {e}")))?
        .into_rgba8();
    let (width, height) = img.dimensions();
    let bytes = img.into_raw();

    let mut clipboard = Clipboard::new().map_err(|e| ScoopError::msg(e.to_string()))?;
    clipboard
        .set_image(ImageData {
            width: width as usize,
            height: height as usize,
            bytes: bytes.into(),
        })
        .map_err(|e| ScoopError::msg(format!("Clipboard image copy failed: {e}")))?;
    Ok(())
}
