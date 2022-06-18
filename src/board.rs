use bevy::prelude::*;
use bevy::window::CursorMoved;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

// A square on the board.
#[derive(Component)]
struct Square;

// A highlight on the board.
#[derive(Component)]
struct HighlightSquare;

// Component for the midline, which is really just a simple rectangle drawn over the board, and serves no other function than decoration.
#[derive(Component)]
struct Midline;

#[derive(Component)]
enum FilesRanks {
    File,
    Rank,
}

// A co-ordinate of an entity on the board.
// Not the same as algebraic notation, which itself is not necessary for the board editor
#[derive(Component, Clone, Copy, PartialEq, Eq, Default, Inspectable, Debug)]
struct Position {
    x: i32,
    y: i32,
}

// A marked square on the board.
#[derive(Component)]
struct Marker;

// Component signifying a piece on the board and its position.
#[derive(Component)]
struct Piece {
    pos: Position,
}

// Determines the size of other components, except pieces
#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub const fn square(x: f32) -> Self {
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

// Determines the size of pieces - kept separate because when I query for Size, I get Piece components thrown in the mix...
#[derive(Component)]
struct PieceSize {
    width: f32,
    height: f32,
}
impl PieceSize {
    pub const fn size(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

// A component to keep track of the cursor's position in the window.
#[derive(Default, Component)]
struct CursorPos {
    cursor_pos: Vec2,
    cursor_grid_pos: Position,
    sprite: Option<(Entity, Vec3)>,
}

// Square colours
const DARK: Color = Color::rgb(0.71, 0.533, 0.388);
const LIGHT: Color = Color::rgb(0.941, 0.851, 0.71);
const HIGHLIGHT: Color = Color::rgba(0.39, 0.54, 0.42, 0.75);
// Notation strings
const RANKS: &str = "12345678";
const FILES: &str = "abcdefgh";

struct PieceDragEvent(Entity, Position);
struct PieceDropEvent(Entity);
#[derive(Debug)]
struct DeletePieceEvent(Entity);
struct DrawPieceEvent(Entity);
struct DrawMarkerEvent(Position);

// This system writes the files and rank numbers.
// At present, it does not scale to window size. I need to think of an algorithm that might solve this problem...
fn draw_notation(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setting the alignment of the text to bottom and left
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Left,
    };

    // Draw out the letters and numbers
    for i in 0..8 {
        let (file, rank) = (FILES.as_bytes()[i] as char, RANKS.as_bytes()[i] as char);
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    file,
                    TextStyle {
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
                    text_alignment,
                ),
                ..default()
            })
            .insert(FilesRanks::File)
            .insert(Position { x: i as i32, y: 0 }); // each letter/number needs to be placed

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    rank,
                    TextStyle {
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
                    text_alignment,
                ),
                ..default()
            })
            .insert(FilesRanks::Rank)
            .insert(Position { x: 7, y: i as i32 });
    }
}

// Spawns camera bundle;
// Create square entities, with position and size.
fn setup_board(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for x in 0..8 {
        for y in 0..8 {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: {
                            if (x + y + 1) % 2 == 0 {
                                LIGHT
                            } else {
                                DARK
                            }
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(Square)
                .insert(Position { x, y })
                .insert(Size::square(1.));
        }
    }
}

// Dummy function that draws 8 pawns.
// Spawn piece entity with Position and PieceSize.
// To be honest, I'm not entirely sure that Piece needs to have Position as a field here.
fn draw_piece_dummy(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..8 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pieces\\n_p.png"),
                ..default()
            })
            .insert(Piece {
                pos: Position { x, y: 6 },
            })
            .insert(Position { x, y: 6 })
            .insert(PieceSize::size(0.67));

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pieces\\C_Pw.png"),
                ..default()
            })
            .insert(Piece {
                pos: Position { x, y: 1 },
            })
            .insert(PieceSize::size(0.67));
    }
}

// The midline is an Entity here - Could I draw it as just a plain old rectangular line?
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

// Scale entities with Size to fit the window size.
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    // This function is where the magic happens, since we're on a 8x8 board we divide by 8
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / 8f32 * window.width() as f32,
            sprite_size.height / 8f32 * window.height() as f32,
            0.,
        );
    }
}

// Scale entities with PieceSize to fit the window size.
// Pieces get resized when they're moved in much the same way.
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

// This function is used in several others to sprites on the board, in the middle of its square.
fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

// There's some black magic going on here with how the entities' positions in-game are translated to the Position struct.
fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform, With<Square>)>,
) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform, _square) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, 8f32),
            convert(pos.y as f32, window.height() as f32, 8f32),
            0.0,
        );
    }
}

//Do the same for highlights.
fn highlight_position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform, With<HighlightSquare>)>,
) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform, _square) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, 8f32),
            convert(pos.y as f32, window.height() as f32, 8f32),
            1.0,
        );
    }
}

// We need to use a similar function for notation as above...
fn notation_position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform, &FilesRanks)>,
) {
    // Overwrites the primary convert() function cos we need to include FilesRanks
    fn convert(pos: f32, bound_window: f32, bound_game: f32, notation_offset: &FilesRanks) -> f32 {
        let tile_size = bound_window / bound_game;
        match notation_offset {
            FilesRanks::File => {
                pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
                    - (tile_size / 2.3)
            }
            FilesRanks::Rank => {
                pos / bound_game * bound_window - (bound_window / 2.)
                    + (tile_size / 2.)
                    + (tile_size / 3.3)
            }
        }
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform, notation) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, 8f32, notation),
            convert(pos.y as f32, window.height() as f32, 8f32, notation),
            2.0,
        );
    }
}

// Calculates the difference between the cursor's position and the sprite's position
fn cursor_to_sprite_diff(cursor_pos: &Vec2, sprite_pos: &Vec3) -> Vec3 {
    Vec3::new(sprite_pos.x - cursor_pos.x, sprite_pos.y - cursor_pos.y, 2.)
}

fn move_piece_system(
    mut ev_drag: EventWriter<PieceDragEvent>,
    mut ev_drop: EventWriter<PieceDropEvent>,
    mut state: Local<CursorPos>,
    windows: Res<Windows>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut sprites: Query<(Entity, &Sprite, With<PieceSize>)>,
    mut pieces: Query<&mut Piece>,
    mut transforms: Query<&mut Transform>,
    mut highlight_q: Query<(Entity, &HighlightSquare)>,
) {
    let window = windows.get_primary().unwrap();
    let tile_size = window.width() / 8.;
    let half_window = Vec2::new(window.width() / 2., window.height() / 2.);
    if let Some(cursor_ev) = cursor_moved_event_reader.iter().last() {
        state.cursor_pos = cursor_ev.position - half_window;
        state.cursor_grid_pos = Position {
            x: (cursor_ev.position.x / tile_size) as i32,
            y: (cursor_ev.position.y / tile_size) as i32,
        };
    };

    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(sprite) = state.sprite {
            let mut sprite_pos = transforms.get_mut(sprite.0).unwrap();
            let mut piece_internal = pieces.get_mut(sprite.0).unwrap();
            sprite_pos.translation = Vec3::new(
                convert(state.cursor_grid_pos.x as f32, window.width() as f32, 8f32),
                convert(state.cursor_grid_pos.y as f32, window.height() as f32, 8f32),
                2.0,
            );

            piece_internal.pos.x = state.cursor_grid_pos.x;
            piece_internal.pos.y = state.cursor_grid_pos.y;

            info!(
                "Piece position on grid: ({}, {})",
                state.cursor_grid_pos.x, state.cursor_grid_pos.y
            );

            let hl = highlight_q.single_mut();
            ev_drop.send(PieceDropEvent(hl.0));
            state.sprite = None;
            return;
        }
    }
    if mouse_button_input.pressed(MouseButton::Left) && state.sprite.is_some() {
        let sprite = state.sprite.unwrap();

        let mut sprite_pos = transforms.get_mut(sprite.0).unwrap();

        sprite_pos.translation.x = state.cursor_pos.x;
        sprite_pos.translation.y = state.cursor_pos.y;
        sprite_pos.translation.z = 4.0;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, sprite, _piece_size) in sprites.iter_mut() {
            let sprite_pos = transforms.get_mut(entity).unwrap().translation;
            let diff = cursor_to_sprite_diff(&state.cursor_pos, &sprite_pos);
            let sprite_size = sprite
                .custom_size
                .unwrap_or_else(|| Vec2::new(tile_size, tile_size));
            if diff.length() < (sprite_size.x / 2.0) {
                state.sprite = Some((entity, diff));
                info!(
                    "Piece picked up on: ({}, {})",
                    state.cursor_grid_pos.x, state.cursor_grid_pos.y
                );
                ev_drag.send(PieceDragEvent(entity, state.cursor_grid_pos));
            }
        }
    }
}

// fn piece_position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform, With<Piece>)>) {
fn piece_position_translation(windows: Res<Windows>, mut q: Query<(&Piece, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (piece, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(piece.pos.x as f32, window.width() as f32, 8f32),
            convert(piece.pos.y as f32, window.height() as f32, 8f32),
            2.0,
        );
    }
}

fn delete_piece(
    mut ev_delete: EventWriter<DeletePieceEvent>,
    mut state: Local<CursorPos>,
    windows: Res<Windows>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    pieces: Query<(Entity, &Piece)>,
) {
    let window = windows.get_primary().unwrap();
    let tile_size = window.width() / 8.;
    let half_window = Vec2::new(window.width() / 2., window.height() / 2.);
    if let Some(cursor_ev) = cursor_moved_event_reader.iter().last() {
        state.cursor_pos = cursor_ev.position - half_window;
        state.cursor_grid_pos = Position {
            x: (cursor_ev.position.x / tile_size) as i32,
            y: (cursor_ev.position.y / tile_size) as i32,
        };
    };

    if mouse_button_input.just_pressed(MouseButton::Middle) {
        for (ent, piece) in pieces.iter() {
            if piece.pos == state.cursor_grid_pos {
                warn!("Piece deleted at: ({}, {})", piece.pos.x, piece.pos.y);
                ev_delete.send(DeletePieceEvent(ent));
            }
        }
    }
}

// This system listens for a right-click and then sends a DrawMarkerEvent to draw_marker().
// When it receives a left-click, it sends an EraseMarkerEvent, which is heard by erase_markers()
fn marker_system(
    mut commands: Commands,
    mut state: Local<CursorPos>,
    windows: Res<Windows>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary().unwrap();
    let tile_size = window.width() / 8.;
    let half_window = Vec2::new(window.width() / 2., window.height() / 2.);
    if let Some(cursor_ev) = cursor_moved_event_reader.iter().last() {
        state.cursor_pos = cursor_ev.position - half_window;
        state.cursor_grid_pos = Position {
            x: (cursor_ev.position.x / tile_size) as i32,
            y: (cursor_ev.position.y / tile_size) as i32,
        };
    };

    if mouse_button_input.just_pressed(MouseButton::Right) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: HIGHLIGHT,
                    ..default()
                },
                ..default()
            })
            .insert(HighlightSquare)
            .insert(Marker)
            .insert(Position {x: state.cursor_grid_pos.x, y: state.cursor_grid_pos.y})
            .insert(Size::square(1.));
    }
}

fn draw_highlight(mut commands: Commands, mut ev_draw_highlight: EventReader<PieceDragEvent>) {
    if let Some(ev) = ev_draw_highlight.iter().last() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: HIGHLIGHT,
                    ..default()
                },
                ..default()
            })
            .insert(HighlightSquare)
            .insert(ev.1)
            .insert(Size::square(1.));
    }
}

fn erase_highlight(mut commands: Commands, mut ev_drop: EventReader<PieceDropEvent>) {
    for ev in ev_drop.iter() {
        commands.entity(ev.0).despawn();
    }
}

fn delete_piece_listener(
    mut commands: Commands,
    mut ev_delete_piece_listener: EventReader<DeletePieceEvent>,
) {
    for ev in ev_delete_piece_listener.iter() {
        commands.entity(ev.0).despawn();
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set_to_stage(
            StartupStage::PreStartup,
            SystemSet::new()
                .with_system(setup_board.before(draw_midline).before(draw_notation))
                .with_system(draw_midline.before(draw_notation).after(setup_board))
                .with_system(draw_notation.after(setup_board).after(draw_midline))
                .with_system(draw_piece_dummy.after(draw_notation)),
        )
        .add_startup_system_set_to_stage(
            StartupStage::Startup,
            SystemSet::new()
                //.with_system(size_scaling.before(position_translation))
                .with_system(position_translation)
                .with_system(piece_position_translation.after(position_translation))
                .with_system(piece_size_scaling.after(piece_position_translation)),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(notation_position_translation)
                .with_system(move_piece_system)
                .with_system(delete_piece.after(move_piece_system))
                .with_system(delete_piece_listener.after(delete_piece))
                .with_system(draw_highlight.before(highlight_position_translation))
                .with_system(highlight_position_translation.before(size_scaling))
                .with_system(erase_highlight.after(highlight_position_translation))
                .with_system(size_scaling)
                .with_system(marker_system),
        )
        .add_event::<PieceDragEvent>()
        .add_event::<PieceDropEvent>()
        .add_event::<DrawPieceEvent>()
        .add_event::<DeletePieceEvent>()
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Position>()
        .run();
    }
}
