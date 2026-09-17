use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use image::imageops::{self, FilterType};
use image::{DynamicImage, GrayImage, Luma};
use serde::Serialize;

use crate::error::{ScoopError, ScoopResult};
use crate::paths::{captures_dir, tessdata_dir};

const DEFAULT_LANG: &str = "eng";
const TESSDATA_FAST_BASE: &str =
    "https://github.com/tesseract-ocr/tessdata_fast/raw/main";

/// Languages Scoop can download on demand (tessdata_fast).
pub const DOWNLOADABLE_LANGS: &[&str] = &["eng", "spa", "fra", "deu", "ita", "por", "rus", "jpn", "chi_sim"];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrStatus {
    pub engine_installed: bool,
    pub engine_version: Option<String>,
    pub language_ready: bool,
    pub installed_languages: Vec<String>,
    pub missing_languages: Vec<String>,
    pub tessdata_dir: String,
    pub ready: bool,
    pub hint: String,
}

fn tesseract_bin() -> PathBuf {
    for candidate in [
        "tesseract",
        "/usr/bin/tesseract",
        "/usr/local/bin/tesseract",
        "/snap/bin/tesseract",
    ] {
        let p = PathBuf::from(candidate);
        if candidate == "tesseract" {
            if Command::new("tesseract").arg("--version").output().is_ok() {
                return PathBuf::from("tesseract");
            }
        } else if p.exists() {
            return p;
        }
    }
    PathBuf::from("tesseract")
}

fn engine_version() -> Option<String> {
    let output = Command::new(tesseract_bin()).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stderr);
    // tesseract prints version on stderr: "tesseract 5.x.x"
    text.lines()
        .next()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
}

pub fn is_engine_installed() -> bool {
    Command::new(tesseract_bin())
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn system_tessdata_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(prefix) = std::env::var("TESSDATA_PREFIX") {
        let p = PathBuf::from(prefix);
        if p.is_dir() {
            dirs.push(p);
        }
    }
    for candidate in [
        "/usr/share/tesseract-ocr/5/tessdata",
        "/usr/share/tesseract-ocr/4.00/tessdata",
        "/usr/share/tesseract-ocr/4.0/tessdata",
        "/usr/share/tessdata",
        "/usr/local/share/tessdata",
    ] {
        let p = PathBuf::from(candidate);
        if p.is_dir() {
            dirs.push(p);
        }
    }
    dirs
}

fn lang_file_name(lang: &str) -> String {
    format!("{lang}.traineddata")
}

fn app_has_lang(lang: &str) -> bool {
    tessdata_dir()
        .map(|d| d.join(lang_file_name(lang)).is_file())
        .unwrap_or(false)
}

fn system_has_lang(lang: &str) -> bool {
    let name = lang_file_name(lang);
    system_tessdata_dirs()
        .into_iter()
        .any(|d| d.join(&name).is_file())
}

pub fn has_language(lang: &str) -> bool {
    app_has_lang(lang) || system_has_lang(lang)
}

fn list_langs_in_dir(dir: &Path) -> Vec<String> {
    let mut langs = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return langs;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(lang) = name.strip_suffix(".traineddata") {
            if !lang.is_empty() && lang != "osd" && lang != "equ" {
                langs.push(lang.to_string());
            }
        }
    }
    langs.sort();
    langs.dedup();
    langs
}

pub fn installed_languages() -> Vec<String> {
    let mut langs = Vec::new();
    if let Ok(dir) = tessdata_dir() {
        langs.extend(list_langs_in_dir(&dir));
    }
    for dir in system_tessdata_dirs() {
        langs.extend(list_langs_in_dir(&dir));
    }
    langs.sort();
    langs.dedup();
    langs
}

/// Prefer app-managed tessdata when it contains the language pack.
fn tessdata_dir_for_run(lang: &str) -> Option<PathBuf> {
    if app_has_lang(lang) {
        tessdata_dir().ok()
    } else {
        None
    }
}

pub fn status() -> ScoopResult<OcrStatus> {
    let engine_installed = is_engine_installed();
    let engine_version = if engine_installed {
        engine_version()
    } else {
        None
    };
    let installed = installed_languages();
    let language_ready = has_language(DEFAULT_LANG);
    let missing: Vec<String> = DOWNLOADABLE_LANGS
        .iter()
        .filter(|l| !has_language(l))
        .map(|s| (*s).to_string())
        .collect();
    let tessdata = tessdata_dir()?.display().to_string();
    let ready = engine_installed && language_ready;

    let hint = if !engine_installed {
        "Tesseract is not installed. On Ubuntu/Debian: sudo apt install tesseract-ocr — or run make install-ocr. Language packs can still be downloaded below.".to_string()
    } else if !language_ready {
        "Tesseract is installed, but the English language pack is missing. Download it below.".to_string()
    } else {
        "OCR is ready.".to_string()
    };

    Ok(OcrStatus {
        engine_installed,
        engine_version,
        language_ready,
        installed_languages: installed,
        missing_languages: missing,
        tessdata_dir: tessdata,
        ready,
        hint,
    })
}

fn validate_lang(lang: &str) -> ScoopResult<&str> {
    let lang = lang.trim();
    if lang.is_empty()
        || !lang
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(ScoopError::msg("Invalid OCR language code"));
    }
    if !DOWNLOADABLE_LANGS.contains(&lang) {
        return Err(ScoopError::msg(format!(
            "Language '{lang}' is not in Scoop's download list. Supported: {}",
            DOWNLOADABLE_LANGS.join(", ")
        )));
    }
    Ok(lang)
}

/// Download a tessdata_fast `*.traineddata` into the app tessdata directory.
pub fn download_language(lang: &str) -> ScoopResult<OcrStatus> {
    let lang = validate_lang(lang)?;
    let dir = tessdata_dir()?;
    let dest = dir.join(lang_file_name(lang));
    let tmp = dir.join(format!("{lang}.traineddata.partial"));
    let url = format!("{TESSDATA_FAST_BASE}/{lang}.traineddata");

    let client = reqwest::blocking::Client::builder()
        .user_agent(concat!("Scoop/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    let mut response = client.get(&url).send()?.error_for_status()?;
    let mut file = File::create(&tmp).map_err(|e| {
        ScoopError::msg(format!("Cannot write OCR data file: {e}"))
    })?;
    response
        .copy_to(&mut file)
        .map_err(|e| ScoopError::msg(format!("OCR download interrupted: {e}")))?;
    file.flush()?;
    drop(file);

    let meta = std::fs::metadata(&tmp).map_err(|e| ScoopError::msg(e.to_string()))?;
    if meta.len() < 10_000 {
        let _ = std::fs::remove_file(&tmp);
        return Err(ScoopError::msg(
            "Downloaded OCR file looks too small — check your network and try again.",
        ));
    }

    std::fs::rename(&tmp, &dest).map_err(|e| {
        ScoopError::msg(format!("Failed to install OCR language pack: {e}"))
    })?;

    status()
}

/// OCR is usable when the engine binary and at least English data are present.
pub fn is_available() -> bool {
    is_engine_installed() && has_language(DEFAULT_LANG)
}

fn mean_luma(img: &GrayImage) -> f32 {
    let mut sum: u64 = 0;
    for p in img.pixels() {
        sum += u64::from(p.0[0]);
    }
    let n = (img.width() * img.height()).max(1) as f64;
    (sum as f64 / n) as f32
}

/// Stretch contrast around mid-gray so faint UI text separates from the background.
fn enhance_contrast(img: &GrayImage, factor: f32) -> GrayImage {
    let mut out = img.clone();
    for p in out.pixels_mut() {
        let v = p.0[0] as f32;
        let stretched = ((v - 128.0) * factor + 128.0).clamp(0.0, 255.0);
        *p = Luma([stretched as u8]);
    }
    out
}

fn mild_sharpen(img: &GrayImage) -> GrayImage {
    // Unsharp-ish 3x3 kernel approximation.
    let w = img.width() as i32;
    let h = img.height() as i32;
    let mut out = img.clone();
    let get = |x: i32, y: i32| -> i16 {
        let x = x.clamp(0, w - 1) as u32;
        let y = y.clamp(0, h - 1) as u32;
        i16::from(img.get_pixel(x, y).0[0])
    };
    for y in 0..h {
        for x in 0..w {
            let c = get(x, y);
            let v = (5 * c
                - get(x - 1, y)
                - get(x + 1, y)
                - get(x, y - 1)
                - get(x, y + 1))
            .clamp(0, 255) as u8;
            out.put_pixel(x as u32, y as u32, Luma([v]));
        }
    }
    out
}

/// Prepare a capture for Tesseract: dark UIs are inverted, contrast boosted, and
/// small crops are upscaled. Returns a temporary PNG path (caller deletes).
fn preprocess_for_ocr(path: &Path) -> ScoopResult<PathBuf> {
    let img = image::open(path).map_err(|e| ScoopError::msg(format!("OCR image open: {e}")))?;
    let mut gray = img.to_luma8();

    // Dark theme / light text → invert so Tesseract sees dark-on-light.
    if mean_luma(&gray) < 140.0 {
        imageops::invert(&mut gray);
    }

    gray = enhance_contrast(&gray, 1.65);
    gray = mild_sharpen(&gray);

    // Upscale small selections; Tesseract struggles below ~300px glyphs.
    let (w, h) = gray.dimensions();
    let long = w.max(h);
    if long > 0 && long < 1600 {
        let scale = (1600.0 / long as f32).clamp(1.5, 4.0);
        let nw = ((w as f32) * scale).round().max(1.0) as u32;
        let nh = ((h as f32) * scale).round().max(1.0) as u32;
        gray = imageops::resize(&gray, nw, nh, FilterType::Lanczos3);
        gray = enhance_contrast(&gray, 1.15);
    }

    let dest = captures_dir()?.join(format!(
        "ocr-prep-{}.png",
        uuid::Uuid::new_v4()
    ));
    DynamicImage::ImageLuma8(gray)
        .save(&dest)
        .map_err(|e| ScoopError::msg(format!("OCR prep save: {e}")))?;
    Ok(dest)
}

fn run_tesseract(bin: &Path, image: &Path, psm: &str) -> Option<String> {
    let mut cmd = Command::new(bin);
    cmd.arg(image)
        .arg("stdout")
        .arg("-l")
        .arg(DEFAULT_LANG)
        .arg("--psm")
        .arg(psm)
        .arg("-c")
        .arg("preserve_interword_spaces=1");

    if let Some(dir) = tessdata_dir_for_run(DEFAULT_LANG) {
        cmd.arg("--tessdata-dir").arg(dir);
    }

    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(cleanup_ocr_text(&text))
    }
}

fn cleanup_ocr_text(text: &str) -> String {
    let mut lines: Vec<String> = text
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        // Drop near-empty garbage lines of only punctuation / symbols.
        .filter(|l| {
            let alnum = l.chars().filter(|c| c.is_alphanumeric()).count();
            alnum >= 2 || l.chars().any(|c| c.is_ascii_digit())
        })
        .collect();

    // Collapse duplicate consecutive lines.
    lines.dedup();
    lines.join("\n")
}

fn score_ocr(text: &str) -> i32 {
    let mut score = 0i32;
    let mut letters = 0i32;
    let mut spaces = 0i32;
    let mut junk = 0i32;
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            letters += 1;
        } else if c.is_whitespace() {
            spaces += 1;
        } else if !c.is_ascii_punctuation() {
            junk += 2;
        } else if matches!(c, '|' | '~' | '^' | '`' | '{' | '}') {
            junk += 1;
        }
    }
    score += letters * 3;
    score += spaces;
    score -= junk * 4;
    // Prefer multi-word / multi-line readable blocks.
    score += text.lines().count() as i32;
    if text.split_whitespace().count() >= 3 {
        score += 12;
    }
    score
}

/// Run system `tesseract` with preprocessing + multi-PSM voting for dark UI text.
pub fn recognize_image(path: &Path) -> ScoopResult<String> {
    if !path.exists() {
        return Err(ScoopError::msg(format!(
            "OCR_FAILED: capture file missing: {}",
            path.display()
        )));
    }
    if !is_engine_installed() {
        return Err(ScoopError::msg(
            "OCR_FAILED: tesseract is not installed. Install with: sudo apt install tesseract-ocr   (or: make install-ocr). Language packs can be downloaded in Settings → OCR.",
        ));
    }
    if !has_language(DEFAULT_LANG) {
        return Err(ScoopError::msg(
            "OCR_FAILED: English OCR data missing. Download it in Settings → OCR.",
        ));
    }

    let bin = tesseract_bin();
    let prep = preprocess_for_ocr(path)?;
    let candidates = ["4", "6", "3", "11"];

    let mut best: Option<(i32, String)> = None;
    for psm in candidates {
        if let Some(text) = run_tesseract(&bin, &prep, psm) {
            let s = score_ocr(&text);
            if best.as_ref().map(|(bs, _)| s > *bs).unwrap_or(true) {
                best = Some((s, text));
            }
        }
    }

    // Fallback: try the original crop without prep (light-on-dark docs sometimes invert poorly).
    if best.as_ref().map(|(s, _)| *s < 20).unwrap_or(true) {
        for psm in candidates {
            if let Some(text) = run_tesseract(&bin, path, psm) {
                let s = score_ocr(&text);
                if best.as_ref().map(|(bs, _)| s > *bs).unwrap_or(true) {
                    best = Some((s, text));
                }
            }
        }
    }

    let _ = std::fs::remove_file(&prep);

    match best {
        Some((_, text)) if !text.trim().is_empty() => Ok(text),
        _ => Err(ScoopError::msg(
            "OCR_FAILED: no readable text detected. Try a larger/clearer region, or type text manually.",
        )),
    }
}
