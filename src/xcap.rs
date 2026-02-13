use fs_extra::dir;
use std::path::PathBuf;
use ::xcap::Monitor;
use chrono::Local;
use std::path::Path;
use arboard::Clipboard;
use image::ImageReader;
use crate::region::select_region;

fn monitor_data() -> (Monitor, (u32, u32)) {

    let monitors = Monitor::all().unwrap();
    let current = monitors
                    .iter()
                    .find(|m| m.is_primary().unwrap_or(false))
                    .unwrap_or(&monitors[0]);

    let res = (current.width().expect("error"),
    current.height().expect(""));
    return (current.clone(), res);
}

pub fn fullscreen_shot() {
    let date = Local::now();
    let datetime = date.format("%Y-%m-%d_%H-%M-%S").to_string();

    let (monitor, (w, h)) = monitor_data();

    let mut dir_path = dirs::home_dir().unwrap_or(PathBuf::from("."));
    dir_path.push("Pictures/Screenshots");
    dir::create_all(&dir_path, false).unwrap();

    let image = monitor
                                                    .capture_region(0, 0, w, h)
                                                    .expect("capture_region failed");

    let filename = format!("screenshot_{}.png", datetime);
    let mut file_path = dir_path;
    file_path.push(filename);

    image.save(&file_path).unwrap();
    clipboard(&file_path);
    println!("Screenshot saved to: {}", file_path.display());
    println!("Screenshot copied to clipboard");

}

pub fn region_screenshot() {
    let (monitor, (_w,_h)) = monitor_data();
    let date = Local::now();
    let datetime = date.format("%Y-%m-%d_%H-%M-%S").to_string();

    let mut dir_path = dirs::home_dir().unwrap_or(PathBuf::from("."));
    dir_path.push("Pictures/Screenshots");
    dir::create_all(&dir_path, false).unwrap();

    let (x, y, width, height) = select_region();

    let image = monitor
        .capture_region(x, y, width, height)
        .expect("Error capturing region");

    let filename = format!("screenshot_{}.png", datetime);
    let mut file_path = dir_path;
    file_path.push(filename);

    image.save(&file_path).unwrap();
    clipboard(&file_path);
    println!("Screenshot saved to: {}", file_path.display());
    println!("Screenshot copied to clipboard");
}

pub fn clipboard(file_path: &Path) {
    let img = ImageReader::open(file_path).expect("Failed to open image").decode().expect("Failed to decode image");

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let mut copy = Clipboard::new().expect("Failed to open clipboard");
    copy.set_image(arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Borrowed(&rgba),
    }).expect("Failed to copy image to clipboard");
}