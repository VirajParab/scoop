use std::path::PathBuf;

use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ImageEncoder, RgbaImage};
use uuid::Uuid;
use xcap::Monitor;

use crate::error::{ScoopError, ScoopResult};
use crate::paths::captures_dir;
use crate::session::Region;

/// Frozen monitor capture bound to a Tauri overlay window rectangle.
#[derive(Debug)]
pub struct DesktopCapture {
    pub path: PathBuf,
    pub pixels: RgbaImage,
    /// Overlay geometry in Tauri physical coordinates (source of truth for placement).
    pub window_x: i32,
    pub window_y: i32,
    pub window_w: u32,
    pub window_h: u32,
}

fn save_png_fast(path: &PathBuf, img: &RgbaImage) -> ScoopResult<()> {
    if img.width() == 0 || img.height() == 0 {
        return Err(ScoopError::msg("Cannot save empty capture"));
    }
    let file = std::fs::File::create(path)?;
    let encoder = PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::Adaptive);
    encoder
        .write_image(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| ScoopError::msg(format!("PNG encode: {e}")))?;
    Ok(())
}

fn clamp_crop(img_w: u32, img_h: u32, x: i64, y: i64, w: i64, h: i64) -> ScoopResult<(u32, u32, u32, u32)> {
    if img_w == 0 || img_h == 0 {
        return Err(ScoopError::msg("Desktop capture is empty"));
    }
    let x = x.clamp(0, img_w as i64 - 1);
    let y = y.clamp(0, img_h as i64 - 1);
    let w = w.max(1);
    let h = h.max(1);
    let w = w.min(img_w as i64 - x).max(1);
    let h = h.min(img_h as i64 - y).max(1);
    Ok((x as u32, y as u32, w as u32, h as u32))
}

/// Prefer the xcap monitor whose origin/size best matches the Tauri rect.
/// On this machine both APIs share the same X11 virtual desktop, so containment
/// by center is reliable; fall back to sorted-index + nearest center.
#[allow(dead_code)]
pub fn pick_xcap_index(monitors: &[Monitor], win_x: i32, win_y: i32, win_w: u32, win_h: u32) -> usize {
    let tcx = win_x + win_w as i32 / 2;
    let tcy = win_y + win_h as i32 / 2;

    if let Some(i) = monitors.iter().position(|m| {
        let x = m.x();
        let y = m.y();
        let w = m.width() as i32;
        let h = m.height() as i32;
        tcx >= x && tcy >= y && tcx < x + w && tcy < y + h
    }) {
        return i;
    }

    // Same enumeration order by geometry (both APIs).
    let mut order: Vec<usize> = (0..monitors.len()).collect();
    order.sort_by_key(|&i| {
        let m = &monitors[i];
        (m.x(), m.y(), m.width(), m.height())
    });

    monitors
        .iter()
        .enumerate()
        .min_by_key(|(_, m)| {
            let mcx = m.x() + m.width() as i32 / 2;
            let mcy = m.y() + m.height() as i32 / 2;
            let dx = (mcx - tcx).abs() as u64;
            let dy = (mcy - tcy).abs() as u64;
            let dw = (m.width() as i64 - win_w as i64).unsigned_abs();
            let dh = (m.height() as i64 - win_h as i64).unsigned_abs();
            dx + dy + dw + dh
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Capture the xcap monitor that best matches a Tauri monitor rectangle.
#[allow(dead_code)]
pub fn capture_for_window(
    window_x: i32,
    window_y: i32,
    window_w: u32,
    window_h: u32,
) -> ScoopResult<DesktopCapture> {
    let monitors = Monitor::all().map_err(|e| ScoopError::msg(format!("Monitor list: {e}")))?;
    if monitors.is_empty() {
        return Err(ScoopError::msg("No monitors found"));
    }

    let idx = pick_xcap_index(&monitors, window_x, window_y, window_w, window_h);
    let monitor = &monitors[idx];
    let shot = monitor
        .capture_image()
        .map_err(|e| ScoopError::msg(format!("Capture failed: {e}")))?;

    eprintln!(
        "Scoop capture: tauri=({window_x},{window_y},{window_w}x{window_h}) → xcap[{idx}]={}@{},{} img={}x{}",
        monitor.name(),
        monitor.x(),
        monitor.y(),
        shot.width(),
        shot.height()
    );

    let path = captures_dir()?.join(format!("desktop-{}.png", Uuid::new_v4()));
    save_png_fast(&path, &shot)?;

    Ok(DesktopCapture {
        path,
        pixels: shot,
        window_x,
        window_y,
        window_w,
        window_h,
    })
}

/// Stitch every xcap monitor into one image covering the virtual desktop.
/// Overlay should be placed at (origin_x, origin_y) with size (width, height).
pub fn capture_virtual_desktop() -> ScoopResult<DesktopCapture> {
    let monitors = Monitor::all().map_err(|e| ScoopError::msg(format!("Monitor list: {e}")))?;
    if monitors.is_empty() {
        return Err(ScoopError::msg("No monitors found"));
    }

    let min_x = monitors.iter().map(|m| m.x()).min().unwrap_or(0);
    let min_y = monitors.iter().map(|m| m.y()).min().unwrap_or(0);
    let max_x = monitors
        .iter()
        .map(|m| m.x() + m.width() as i32)
        .max()
        .unwrap_or(min_x);
    let max_y = monitors
        .iter()
        .map(|m| m.y() + m.height() as i32)
        .max()
        .unwrap_or(min_y);
    let width = (max_x - min_x).max(1) as u32;
    let height = (max_y - min_y).max(1) as u32;

    let mut canvas = RgbaImage::from_pixel(width, height, image::Rgba([0, 0, 0, 255]));

    for (i, monitor) in monitors.iter().enumerate() {
        let shot = monitor
            .capture_image()
            .map_err(|e| ScoopError::msg(format!("Capture failed on {}: {e}", monitor.name())))?;
        let dx = (monitor.x() - min_x).max(0) as u32;
        let dy = (monitor.y() - min_y).max(0) as u32;
        image::imageops::replace(&mut canvas, &shot, dx as i64, dy as i64);
        eprintln!(
            "Scoop stitch[{i}]: {} @{},{} → canvas {},{} ({}x{})",
            monitor.name(),
            monitor.x(),
            monitor.y(),
            dx,
            dy,
            shot.width(),
            shot.height()
        );
    }

    let path = captures_dir()?.join(format!("desktop-{}.png", Uuid::new_v4()));
    save_png_fast(&path, &canvas)?;

    Ok(DesktopCapture {
        path,
        pixels: canvas,
        window_x: min_x,
        window_y: min_y,
        window_w: width,
        window_h: height,
    })
}

/// Crop using overlay-local CSS coordinates (what the user dragged).
/// Maps through the frozen image via the overlay's CSS size — no screen-space math.
pub fn crop_from_desktop_client(
    desktop: &DesktopCapture,
    client: &Region,
    css_w: f64,
    css_h: f64,
) -> ScoopResult<PathBuf> {
    if client.width == 0 || client.height == 0 {
        return Err(ScoopError::msg("Selection is empty"));
    }
    let css_w = css_w.max(1.0);
    let css_h = css_h.max(1.0);
    let sx = desktop.pixels.width() as f64 / css_w;
    let sy = desktop.pixels.height() as f64 / css_h;

    let (x, y, w, h) = clamp_crop(
        desktop.pixels.width(),
        desktop.pixels.height(),
        (client.x as f64 * sx).round() as i64,
        (client.y as f64 * sy).round() as i64,
        (client.width as f64 * sx).round() as i64,
        (client.height as f64 * sy).round() as i64,
    )?;

    let cropped = image::imageops::crop_imm(&desktop.pixels, x, y, w, h).to_image();
    if cropped.width() == 0 || cropped.height() == 0 {
        return Err(ScoopError::msg("Crop produced an empty image"));
    }
    let path = captures_dir()?.join(format!("{}.png", Uuid::new_v4()));
    save_png_fast(&path, &cropped)?;
    Ok(path)
}

/// Map Tauri-physical screen region → image pixels via window size ratio.
#[allow(dead_code)]
pub fn crop_from_desktop(desktop: &DesktopCapture, region: &Region) -> ScoopResult<PathBuf> {
    if region.width == 0 || region.height == 0 {
        return Err(ScoopError::msg("Selection is empty"));
    }

    let sx = desktop.pixels.width() as f64 / desktop.window_w.max(1) as f64;
    let sy = desktop.pixels.height() as f64 / desktop.window_h.max(1) as f64;

    let (local_x, local_y, w, h) = clamp_crop(
        desktop.pixels.width(),
        desktop.pixels.height(),
        ((region.x - desktop.window_x) as f64 * sx).round() as i64,
        ((region.y - desktop.window_y) as f64 * sy).round() as i64,
        (region.width as f64 * sx).round() as i64,
        (region.height as f64 * sy).round() as i64,
    )?;

    let cropped = image::imageops::crop_imm(&desktop.pixels, local_x, local_y, w, h).to_image();
    if cropped.width() == 0 || cropped.height() == 0 {
        return Err(ScoopError::msg("Crop produced an empty image"));
    }
    let path = captures_dir()?.join(format!("{}.png", Uuid::new_v4()));
    save_png_fast(&path, &cropped)?;
    Ok(path)
}

pub fn capture_region(region: &Region) -> ScoopResult<PathBuf> {
    let monitors = Monitor::all().map_err(|e| ScoopError::msg(format!("Monitor list: {e}")))?;
    if monitors.is_empty() {
        return Err(ScoopError::msg("No monitors found"));
    }

    let monitor = monitors
        .iter()
        .find(|m| {
            let mx = m.x();
            let my = m.y();
            let mw = m.width() as i32;
            let mh = m.height() as i32;
            region.x >= mx && region.y >= my && region.x < mx + mw && region.y < my + mh
        })
        .unwrap_or(&monitors[0]);

    let mx = monitor.x();
    let my = monitor.y();
    let full = monitor
        .capture_image()
        .map_err(|e| ScoopError::msg(format!("Capture failed: {e}")))?;

    let (local_x, local_y, w, h) = clamp_crop(
        full.width(),
        full.height(),
        (region.x - mx) as i64,
        (region.y - my) as i64,
        region.width as i64,
        region.height as i64,
    )?;

    let cropped: RgbaImage = image::imageops::crop_imm(&full, local_x, local_y, w, h).to_image();
    let path = captures_dir()?.join(format!("{}.png", Uuid::new_v4()));
    save_png_fast(&path, &cropped)?;
    Ok(path)
}

pub fn cleanup_capture(path: &str) {
    let _ = std::fs::remove_file(path);
}

/// Keep the session crop at a stable path so later Save-to-Library always finds it.
pub fn hold_session_capture(src: &PathBuf) -> ScoopResult<PathBuf> {
    let dest = captures_dir()?.join("session-selection.png");
    if src != &dest {
        std::fs::copy(src, &dest)
            .map_err(|e| ScoopError::msg(format!("Failed to keep selection image: {e}")))?;
        // Remove the ephemeral UUID crop; session now owns session-selection.png
        if src.file_name().and_then(|n| n.to_str()) != Some("session-selection.png") {
            let _ = std::fs::remove_file(src);
        }
    }
    if !dest.exists() {
        return Err(ScoopError::msg("Selection image missing after hold"));
    }
    Ok(dest)
}

pub fn read_data_url(path: &std::path::Path) -> ScoopResult<String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    let bytes = std::fs::read(path)
        .map_err(|e| ScoopError::msg(format!("Read capture failed: {e}")))?;
    if bytes.is_empty() {
        return Err(ScoopError::msg("Capture file is empty"));
    }
    Ok(format!(
        "data:image/png;base64,{}",
        STANDARD.encode(bytes)
    ))
}
