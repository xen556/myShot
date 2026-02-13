#[cfg(target_os = "linux")]
pub fn select_region() -> (u32, u32, u32, u32) {
    use std::process::Command;

    let output = Command::new("slurp")
        .output()
        .expect("Error running slurp");
    let slurp_out = String::from_utf8(output.stdout).unwrap();
    let slurp_out = slurp_out.trim();
    let mut parts = slurp_out.split_whitespace();
    let pos = parts.next().unwrap();
    let size = parts.next().unwrap();
    let mut pos_it = pos.split(",");
    let x = pos_it.next().unwrap().parse().unwrap();
    let y = pos_it.next().unwrap().parse().unwrap();
    let mut size_it = size.split("x");
    let width = size_it.next().unwrap().parse().unwrap();
    let height = size_it.next().unwrap().parse().unwrap();

    (x, y, width, height)
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn select_region() -> (u32, u32, u32, u32) {
    // TODO: tu można w przyszłości zrobić transparent overlay z egui/winit
    // Na razie zwracamy full screen jako placeholder
    let width = 1920;
    let height = 1080;
    let x = 0;
    let y = 0;

    (x, y, width, height)
}
