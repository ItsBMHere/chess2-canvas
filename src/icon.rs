use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;
use bevy::prelude::NonSend;
use std::path::Path;

pub fn set_icon(window: NonSend<WinitWindows>) {
    let primary = window.get_window(WindowId::primary()).unwrap();
    let path = Path::new("assets\\pieces\\icon.png");

    // image - not bevy-native but bevy doesn't support changing the icon yet
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