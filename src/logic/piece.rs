use crate::states::AppState;
use bitflags::bitflags;

use super::{
    board::{get_best_next_move, BoardRecource},
    GRID_BLOCK_SIZE, GRID_SIZE, SQUARE_SIZE, TILE_NUMBER,
};
use bevy::{
    app::App,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
                init_pieces_recources,
                spawn_pieces.after(init_pieces_recources),
            ),
        )
        .add_systems(
            Update,
            (select_piece, render_possible_routes).run_if(in_state(AppState::InGame)),
        )
        // .add_systems(Update, play_ai)
        .init_resource::<PieceData>()
        .init_resource::<SelectedPiece>();
    }
}

#[derive(Resource, Default)]
pub struct PieceData {
    mesh_handle: Mesh2dHandle,
    images: Vec<Handle<Image>>, // Use a vector to hold all image handles
}

impl PieceData {
    pub fn get(&self, piece_type: PieceTypes, color: bool) -> Handle<Image> {
        if color {
            self.images[piece_type as usize * 2].clone()
        } else {
            self.images[piece_type as usize * 2 + 1].clone()
        }
    }
}

macro_rules! load_piece_image {
    ($asset_server:expr, $color:expr, $piece_type:expr) => {{
        let path = format!("pieces_png\\{}-{}.png", $color, $piece_type);
        $asset_server.load(path).clone()
    }};
}

macro_rules! build_piece_data {
    ($enum_name:ident, [$( $variant:ident,)*] ) => {

        #[derive(Default, Copy, Clone, Hash, PartialEq, Eq, Debug)]
        pub enum $enum_name {
            #[default]
            $( $variant, )*
        }

        fn init_pieces_recources(
            mut meshes: ResMut<Assets<Mesh>>,
            asset_server: Res<AssetServer>,
            mut piece_recourecs: ResMut<PieceData>,
        ) {
            *piece_recourecs = {
                let mesh = Mesh::from(Rectangle::default());
                let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

                let mut images = Vec::new();
                $(
                    let data = load_piece_image!(asset_server, "white", stringify!($variant).to_lowercase());
                    images.push(data);
                    let data = load_piece_image!(asset_server, "black", stringify!($variant).to_lowercase());
                    images.push(data);
                )*

                PieceData {
                    mesh_handle,
                    images,
                }
            };
        }
    };
}

build_piece_data!(
    PieceTypes,
    [
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
        Jester,
        Amazon,
        GrandCommander,
        Abbess,
    ]
);

// ---------------------------------------------------------------

// system's Components, Bundles, Structs and data types
// ---------------------------------------------------------------
#[derive(Component, Default)]
pub struct ComponentPiece {
    pub color: bool,
    pub piece_type: PieceTypes,
}

#[derive(Bundle)]
pub struct PieceBundle<M: bevy::sprite::Material2d> {
    pub piece: ComponentPiece,
    pub sprite: MaterialMesh2dBundle<M>,
}

#[derive(Debug, Clone, Copy)]
pub struct Rules {
    pub movment_rules: MovementsRules,
    pub step_over_rule: bool,
    pub max_distance: Option<i32>,
    pub multiple_direction_rule: bool,
}

impl Rules {
    pub fn new(
        movment_rules: MovementsRules,
        step_over: bool,
        max_distance: Option<i32>,
        multi_direction: bool,
    ) -> Self {
        Rules {
            movment_rules,
            step_over_rule: step_over,
            max_distance,
            multiple_direction_rule: multi_direction,
        }
    }
}

bitflags! {
    pub struct MovementsRules: u32 {
        const PAWN_MOVMENT = 1 << 0;
        const VERTICAL_MOVMENT = 1 << 1;
        const HORIZONTAL_MOVMENT = 1 << 2;
        const DIAGONAL_MOVMENT = 1 << 3;
        const SHIFT_STEP_MOVMENT = 1 << 4;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceTypes,
    color: bool,
    has_moved: bool,
    id: Option<Entity>,
    rules: Rules,
}

impl Piece {
    pub fn new(piece_type: PieceTypes, color: bool, rules: Rules) -> Self {
        Piece {
            piece_type,
            color,
            has_moved: false,
            id: None,
            rules,
        }
    }
    pub fn get_id(&self) -> Option<Entity> {
        self.id
    }
    pub fn get_type(&self) -> PieceTypes {
        self.piece_type
    }
    pub fn get_color(&self) -> bool {
        self.color
    }
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }
    pub fn set_has_moved(&mut self) {
        self.has_moved = true;
    }
    pub fn get_rules(&self) -> Rules {
        self.rules
    }
}

#[derive(Resource, Default)]
struct SelectedPiece {
    selected: Option<usize>,
    optional_paths: Vec<Entity>,
}
// ---------------------------------------------------------------

// needed functions
// ---------------------------------------------------------------
#[allow(unused_assignments)]
fn pos_to_cords(src: Vec2) -> Option<usize> {
    let src_x = (src.x + SQUARE_SIZE / 2.0).floor() as i32;
    let src_y = (src.y + SQUARE_SIZE / 2.0).floor() as i32;
    let sqr_size = SQUARE_SIZE as i32;
    let mut x = 0;
    let mut y = 0;

    if (src_x as f32) > (GRID_SIZE / 2.0) || (src_x as f32) < (-GRID_SIZE / 2.0) {
        return None;
    }
    if (src_y as f32) > (GRID_SIZE / 2.0) || (src_y as f32) < (-GRID_SIZE / 2.0) {
        return None;
    }

    if src_x <= 0 {
        x = (TILE_NUMBER as i32 / 2) - 1 - src_x / -sqr_size;
    } else {
        x = (TILE_NUMBER as i32 / 2) + src_x / sqr_size;
    }
    if src_y <= 0 {
        y = (TILE_NUMBER as i32 / 2) + src_y / -sqr_size;
    } else {
        y = (TILE_NUMBER as i32 / 2) - 1 - src_y / sqr_size;
    }
    Some(y as usize * TILE_NUMBER + x as usize)
}

fn from_index_to_srceen_position(index: usize) -> Vec3 {
    let x = index % TILE_NUMBER;
    let y = index / TILE_NUMBER;
    let mut position = Vec3::new(0., 0., 0.1);
    if x <= TILE_NUMBER / 2 {
        position.x -= SQUARE_SIZE * (TILE_NUMBER / 2 - x) as f32;
    } else {
        position.x += SQUARE_SIZE * (x - TILE_NUMBER / 2) as f32;
    }
    if y >= TILE_NUMBER / 2 {
        position.y -= SQUARE_SIZE * (y - (TILE_NUMBER / 2 - 1)) as f32;
    } else {
        position.y += SQUARE_SIZE * ((TILE_NUMBER / 2 - 1) - y) as f32;
    }
    position
}
// ---------------------------------------------------------------

// system logic
// ---------------------------------------------------------------
fn render_possible_routes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board: ResMut<BoardRecource>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut gizmos: Gizmos,
) {
    match selected_piece.selected {
        Some(cords) => {
            for route in &selected_piece.optional_paths {
                commands.entity(*route).despawn();
            }
            selected_piece.optional_paths = Vec::new();
            for route in board.get_possible_moves(cords) {
                if board.grid[route].is_some() {
                    gizmos.circle_2d(
                        from_index_to_srceen_position(route).xy(),
                        GRID_BLOCK_SIZE * 2.0,
                        Color::BLACK,
                    );
                    continue;
                }
                let id = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Circle {
                                radius: GRID_BLOCK_SIZE / 2.0,
                            })
                            .into(),
                        material: materials.add(Color::BLACK),
                        transform: Transform::from_translation(from_index_to_srceen_position(
                            route,
                        )),
                        ..default()
                    })
                    .id();
                selected_piece.optional_paths.push(id);
            }
        }
        None => {
            for route in &selected_piece.optional_paths {
                commands.entity(*route).despawn();
            }
            selected_piece.optional_paths = Vec::new();
        }
    }
}

fn select_piece(
    // mut commands: Commands,
    mut mouse: EventReader<MouseButtonInput>,
    mut board: ResMut<BoardRecource>,
    mut selected_piece: ResMut<SelectedPiece>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut pieces: Query<&mut Transform, With<ComponentPiece>>,
) {
    for ev in mouse.read() {
        match ev.state {
            ButtonState::Pressed => {}
            ButtonState::Released => {
                // get the mouse position on the screen and ajust them to the camera position
                let (camera, camera_transform) = q_camera.single();
                let position = q_windows
                    .single()
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor));
                // checks if there is a selected piece, if not, return the last selected piece to normal size
                if let Some(cords) = selected_piece.selected {
                    if let Some(piece) = board.grid[cords] {
                        let id = piece.get_id().unwrap();
                        if let Ok(mut component) = pieces.get_mut(id) {
                            component.scale -= Vec3::splat(SQUARE_SIZE / 2.0);
                        }
                    }
                }
                // get the last position and the new position and send them to check for the legal move,
                // if the move is legal then rerender anything that is needed and reset the selected piece
                if let Some(cords) = pos_to_cords(position.unwrap()) {
                    match selected_piece.selected {
                        None => {
                            selected_piece.selected = Some(cords);
                        }
                        Some(src) => {
                            let result = board.move_piece(src, cords);
                            for res in result {
                                if let Ok(mut component) = pieces.get_mut(res.0) {
                                    component.translation = from_index_to_srceen_position(res.1);
                                }
                            }
                            selected_piece.selected = None;
                        }
                    }
                }
                // if the selected piece exist as a normal piece, then increase it's size
                if let Some(cords) = selected_piece.selected {
                    match board.grid[cords] {
                        Some(piece) => {
                            let id = piece.get_id().unwrap();
                            if let Ok(mut component) = pieces.get_mut(id) {
                                component.scale += Vec3::splat(SQUARE_SIZE / 2.0);
                            }
                        }
                        None => {
                            selected_piece.selected = None;
                        }
                    }
                }
            }
        }
    }
}

fn spawn_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board: ResMut<BoardRecource>,
    piece_recourecs: Res<PieceData>,
) {
    for (index, piece) in board.grid.iter_mut().enumerate() {
        match piece {
            Some(p) => {
                let position = Transform::from_translation(from_index_to_srceen_position(index));
                let id = commands
                    .spawn(PieceBundle {
                        // mesh: mesh_handle,
                        piece: ComponentPiece {
                            color: p.color,
                            piece_type: p.piece_type,
                        },
                        sprite: MaterialMesh2dBundle {
                            transform: position.with_scale(Vec3::splat(SQUARE_SIZE)),
                            mesh: piece_recourecs.mesh_handle.clone(),
                            material: materials.add(piece_recourecs.get(p.piece_type, p.color)),
                            ..default()
                        },
                    })
                    .id();
                p.id = Some(id);
            }
            None => {}
        }
    }
}
// ---------------------------------------------------------------
