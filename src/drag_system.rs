use super::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorMoved;

fn print_mouse_events(
    mut cursor_event_reader: EventReader<CursorMoved>,
) {
    for event in cursor_event_reader.iter() {
        println!("{:?}", event);
    }
}

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(print_mouse_events)
            .run()
    }
}
