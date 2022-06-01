
use super::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};

#[derive(Component)]
struct Square;

#[derive(Component)]
struct Midline;

#[derive(Component)]
enum FilesRanks {
    File,
    Rank,
}

#[derive(Component)]
struct Piece;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }

    pub fn rectangle(x: f32) -> Self {
        Self {
            width: x,
            height: x / 128.,
        }
    }
}

#[derive(Component)]
struct PieceSize {
    width: f32,
    height: f32,
}
impl PieceSize {
    pub fn size(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}


const DARK: Color = Color::rgb(0.71, 0.533, 0.388);
const LIGHT: Color = Color::rgb(0.941, 0.851, 0.71);

const RANKS: &'static str = "12345678";
const FILES: &'static str = "abcdefgh";


fn draw_notation(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Left,
    };

    for i in 0..8 {

        let (file, rank) = (FILES.as_bytes()[i] as char, RANKS.as_bytes()[i] as char);
        commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(file, TextStyle {
                font: asset_server.load("fonts\\NotoSans-Bold.ttf"),
                font_size: 18.,
                color: {
                    if i % 2 == 0 {
                        LIGHT
                    } else {
                        DARK
                    }
                },
            },
            text_alignment),
            ..default()
        })
        .insert(FilesRanks::File)
        .insert(Position {x: i as i32, y: 0} );

        commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(rank, TextStyle {
                font: asset_server.load("fonts\\NotoSans-Bold.ttf"),
                font_size: 18.,
                color: {
                    if i % 2 == 0 {
                        DARK
                    } else {
                        LIGHT
                    }
                },
            },
            text_alignment),
            ..default()
        })
        .insert(FilesRanks::Rank)
        .insert(Position {x: 7, y: i as i32} );
    }
}

// fn setup_board(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());

//     for x in 0..8 {
//         for y in 0..8 {
//             commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: {
//                         if (x + y + 1) % 2 == 0 {
//                             LIGHT
//                         } else {
//                             DARK
//                         }
//                     },
//                     ..default()
//                 },
//                 ..default()
//             })
//             .insert(Square)
//             .insert(Position {x, y})
//             .insert(Size::square(1.));
//         }
//     }
// }

fn light_or_dark(x: i32, y: i32) -> Color {
    if (x + y + 1) % 2 == 0 {
        LIGHT
    } else {
        DARK
    }
}

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    for x in 0..8 {
        for y in 0..8 {
            commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: materials.add(ColorMaterial::from(light_or_dark(x, y))),
                ..default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(Square)
            .insert(Position {x, y})
            .insert(Size::square(1.));
        }
    }
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(PickingCameraBundle::default());
}

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}

fn draw_piece_dummy(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..8 {
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("pieces\\c_p.png"),
            ..default()
        })
        .insert(Piece)
        .insert(Position {x, y: 6})
        .insert(PieceSize::size(0.67));

        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("pieces\\C_Pw.png"),
            ..default()
        })
        .insert(Piece)
        .insert(Position {x, y: 1})
        .insert(PieceSize::size(0.67));

    }
 
}




fn draw_midline(mut commands: Commands) {
    commands
    .spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::CYAN,
            ..default()
        },
        ..default()
    })
    .insert(Midline)
    .insert(Size::rectangle(9.5));
} 

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / 8f32 * window.width() as f32,
            sprite_size.height / 8f32 * window.height() as f32,
            0.,
        );
    }
}

fn piece_size_scaling(windows: Res<Windows>, mut q: Query<(&PieceSize, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / 8f32 * (window.width() as f32 / 96.),
            sprite_size.height / 8f32 * (window.height() as f32 / 96.),
            0.,
        );
    }
}


fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform, With<Square>)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform, _square) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, 8f32),
            convert(pos.y as f32, window.height() as f32, 8f32),
            0.0,
        );
    }
}

fn piece_position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform, With<Piece>)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform, _piece) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, 8f32),
            convert(pos.y as f32, window.height() as f32, 8f32),
            2.0,
        );
    }
}

fn notation_position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform, &FilesRanks)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32, notation_offset: &FilesRanks) -> f32 {
        let tile_size = bound_window / bound_game;
        match notation_offset {
            FilesRanks::File => pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.) - (tile_size / 2.3),
            FilesRanks::Rank => pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.) + (tile_size / 3.3)
        }
        
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform, notation) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, 8f32, notation),
            convert(pos.y as f32, window.height() as f32, 8f32, notation),
            1.0,
        );
    }
}


fn print_mouse_events(
    windows: Res<Windows>,
    mut cursor_event_reader: EventReader<CursorMoved>,
    mut mouse_press_event_reader: EventReader<MouseButtonInput>,
) {
    let window = windows.get_primary().unwrap();
    let tile_size = window.width() / 8f32;
    for event in cursor_event_reader.iter() {
        let coords = event.position.to_array();
        let pos = Position {
            x: (coords[0] / tile_size) as i32,
            y: (coords[1] / tile_size) as i32
        };
        println!("Cursor on Position: (x: {}, y: {})", pos.x, pos.y);
        
    }
    for mice in mouse_press_event_reader.iter() {
        println!("{:?}", mice);
    }
}

// fn cursor_position_translation(
//     windows: Res<Windows>, 
//     mut cursor_event_reader: EventReader<CursorMoved>,
// ) {
//     fn convert(pos: Res<CursorMoved>, bound_window: f32, bound_game: f32) -> (f32, f32) {
//         let tile_size = bound_window / bound_game;
//         let coords = pos.position.to_array();
//         (
//             coords[0] % tile_size,
//             coords[1] % tile_size,
//         )
//     }
// }


pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPickingPlugins)
            .add_startup_system_set(
                SystemSet::new()
                    .with_system(setup_board.before(draw_midline).before(draw_notation))
                    .with_system(draw_midline.before(draw_notation).after(setup_board))
                    .with_system(draw_notation.after(setup_board).after(draw_midline))
                    .with_system(draw_piece_dummy.after(draw_notation))
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label("board_scale")
                    .before("notation_scale")
                    .with_system(size_scaling.before(position_translation))
                    .with_system(position_translation.after(size_scaling))
                    .with_system(piece_position_translation.after(position_translation))
                    .with_system(piece_size_scaling)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label("notation_scale")
                    .after("board_scale")
                    .with_system(notation_position_translation)
            )
            .add_system_to_stage(CoreStage::PostUpdate, print_events)
            .run();
    }
}