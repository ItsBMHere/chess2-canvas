use super::*;
use win_screenshot::addon::*;
use win_screenshot::capture::*;
use chrono::prelude::{Local};

#[cfg(target_os = "windows")]
pub fn take_screenshot(
    kbd: Res<Input<KeyCode>>,
) {
    if (kbd.pressed(KeyCode::LControl) || kbd.pressed(KeyCode::RControl)) && kbd.just_pressed(KeyCode::S) {
        info!("Screenshot saved.");

        match capture_window(find_window("Chess 2 Board Editor").unwrap()).ok() {
            Some(image) => {
                let timestamp = Local::now().format("%Y%m%d_%H-%M-%S").to_string();
                let filepath = format!("chess2-canvas_{}.png", timestamp);
                image
                    .save(filepath)
                    .unwrap()
            },
            None => error!("Could not find the window..."),
        }

    }
}