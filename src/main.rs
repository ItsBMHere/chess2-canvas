extern crate bevy_svg;

use bevy::prelude::*;
use bevy_editor_pls::*;


mod board;
mod icon;
mod screenshot;

use board::BoardPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 }) // Anti-aliasing
        .insert_resource(WindowDescriptor {
            width: 768.,
            height: 768.,
            title: "Chess 2 Board Editor".to_owned(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(icon::set_icon)
        .add_system(bevy::input::system::exit_on_esc_system) // Exit game on ESC
        .add_plugins(DefaultPlugins)
        // Debugging stuff
        //.add_plugin(EditorPlugin)
        //.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        // Game plugin
        .add_plugin(BoardPlugin) // BoardPlugin + ScreenshotPlugin
        .run();
}
