extern crate bevy_svg;

use bevy::prelude::*;
use bevy_editor_pls::*;

mod icon;
mod board;

use board::BoardPlugin;

fn main() {
    App::new()
    .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor {
            width: 768.,
            height: 768.,
            title: "Chess 2 Board Editor".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(icon::set_icon)
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugin(BoardPlugin)
        .run()
}
