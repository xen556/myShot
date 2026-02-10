use fs_extra::dir;
use std::path::PathBuf;
use ::xcap::Monitor;
use chrono::Local;

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
    let datetime = date.format("%Y-%m-%d").to_string();

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

}
