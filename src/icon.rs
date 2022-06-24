use bevy::prelude::NonSend;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use std::path::Path;
use winit::window::Icon;

// Use bevy's WindowId struct and the winit library to display a taskbar/favicon.
pub fn set_icon(window: NonSend<WinitWindows>) {
    let primary = window.get_window(WindowId::primary()).unwrap();
    let path = Path::new("assets\\icon.png");

    // image - not bevy-native but bevy doesn't support changing the icon yet
    // Image can be loaded with an asset server - the method below is more direct, and simpler.
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Couldn't open image path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}
