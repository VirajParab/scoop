use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{ScoopError, ScoopResult};

fn tesseract_bin() -> PathBuf {
    for candidate in [
        "tesseract",
        "/usr/bin/tesseract",
        "/usr/local/bin/tesseract",
        "/snap/bin/tesseract",
    ] {
        let p = PathBuf::from(candidate);
        if candidate == "tesseract" {
            // Resolve via PATH
            if Command::new("tesseract").arg("--version").output().is_ok() {
                return PathBuf::from("tesseract");
            }
        } else if p.exists() {
            return p;
        }
    }
    PathBuf::from("tesseract")
}

pub fn is_available() -> bool {
    Command::new(tesseract_bin())
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run system `tesseract` CLI when available.
pub fn recognize_image(path: &Path) -> ScoopResult<String> {
    if !path.exists() {
        return Err(ScoopError::msg(format!(
            "OCR_FAILED: capture file missing: {}",
            path.display()
        )));
    }

    let bin = tesseract_bin();
    let output = Command::new(&bin)
        .arg(path)
        .arg("stdout")
        .arg("-l")
        .arg("eng")
        .arg("--psm")
        .arg("6")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() {
                Err(ScoopError::msg(
                    "OCR_FAILED: no text detected in selection. Try a larger/clearer region, or type text manually.",
                ))
            } else {
                Ok(text)
            }
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
            Err(ScoopError::msg(format!(
                "OCR_FAILED: tesseract error: {err}"
            )))
        }
        Err(_) => Err(ScoopError::msg(
            "OCR_FAILED: tesseract is not installed. Run: sudo apt install tesseract-ocr tesseract-ocr-eng   (or: make install-ocr)",
        )),
    }
}
