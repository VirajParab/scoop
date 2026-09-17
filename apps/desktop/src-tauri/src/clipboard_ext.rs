use arboard::Clipboard;

use crate::error::{ScoopError, ScoopResult};

pub fn copy_text(text: &str) -> ScoopResult<()> {
    let mut clipboard = Clipboard::new().map_err(|e| ScoopError::msg(e.to_string()))?;
    clipboard
        .set_text(text.to_string())
        .map_err(|e| ScoopError::msg(e.to_string()))?;
    Ok(())
}
