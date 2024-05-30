use bevy::{app::App, math::vec2, prelude::*};

use crate::{states::AppState, tuple_as};

use super::{
    piece::{Piece, PieceTypes},
    range_inclusive, GRID_BLOCK_SIZE, SQUARE_SIZE, TILE_NUMBER,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardRecource>().add_systems(
            OnEnter(AppState::InGame),
            (setup_board, spawn_grid.after(setup_board)),
        );
    }
}

#[derive(Resource, Default, Clone)]
pub struct BoardRecource {
    pub grid: Vec<Option<Piece>>,
    pub turn: bool,
}

impl BoardRecource {
    pub fn change_turn(&mut self) {
        self.turn = !self.turn;
    }
}

fn setup_board(mut board: ResMut<BoardRecource>) {
    board.turn = true;
    board.grid = vec![None; TILE_NUMBER * TILE_NUMBER];
    for i in 0..TILE_NUMBER {
        board.grid[to_board_index(i, TILE_NUMBER - 2)] = Some(Piece::new(PieceTypes::Pawn, true));
        board.grid[to_board_index(i, 1)] = Some(Piece::new(PieceTypes::Pawn, false));
    }
    board.grid[to_board_index(0, 0)] = Some(Piece::new(PieceTypes::Rook, false));
    board.grid[to_board_index(TILE_NUMBER - 1, 0)] = Some(Piece::new(PieceTypes::Rook, false));
    board.grid[to_board_index(TILE_NUMBER - 1, TILE_NUMBER - 1)] =
        Some(Piece::new(PieceTypes::Rook, true));
    board.grid[to_board_index(0, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Rook, true));

    board.grid[to_board_index(1, 0)] = Some(Piece::new(PieceTypes::Knight, false));
    board.grid[to_board_index(TILE_NUMBER - 2, 0)] = Some(Piece::new(PieceTypes::Knight, false));
    board.grid[to_board_index(TILE_NUMBER - 2, TILE_NUMBER - 1)] =
        Some(Piece::new(PieceTypes::Knight, true));
    board.grid[to_board_index(1, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Knight, true));

    board.grid[to_board_index(2, 0)] = Some(Piece::new(PieceTypes::Bishop, false));
    board.grid[to_board_index(TILE_NUMBER - 3, 0)] = Some(Piece::new(PieceTypes::Bishop, false));
    board.grid[to_board_index(TILE_NUMBER - 3, TILE_NUMBER - 1)] =
        Some(Piece::new(PieceTypes::Bishop, true));
    board.grid[to_board_index(2, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Bishop, true));

    board.grid[to_board_index(3, 0)] = Some(Piece::new(PieceTypes::King, false));
    board.grid[to_board_index(3, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::King, true));

    board.grid[to_board_index(4, 0)] = Some(Piece::new(PieceTypes::Queen, false));
    board.grid[to_board_index(4, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Queen, true));

    // board.grid[to_board_index(5, 0)] = Some(Piece::new(PieceTypes::Jester, false));
    // board.grid[to_board_index(5, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Jester, true));

    // board.grid[to_board_index(6, 0)] = Some(Piece::new(PieceTypes::Amazon, false));
    // board.grid[to_board_index(6, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Amazon, true));

    // board.grid[to_board_index(7, 0)] = Some(Piece::new(PieceTypes::Abbess, false));
    // board.grid[to_board_index(7, TILE_NUMBER - 1)] = Some(Piece::new(PieceTypes::Abbess, true));

    // board.grid[to_board_index(8, 0)] = Some(Piece::new(PieceTypes::GrandCommander, false));
    // board.grid[to_board_index(8, TILE_NUMBER - 1)] =
    //     Some(Piece::new(PieceTypes::GrandCommander, true));
}

fn to_board_index(x: usize, y: usize) -> usize {
    TILE_NUMBER * y + x
}

fn to_cord_index(index: usize) -> (usize, usize) {
    (index % TILE_NUMBER, index / TILE_NUMBER)
}

fn spawn_grid(mut commands: Commands) {
    let white = Color::hex("eeeed2").unwrap();
    let black = Color::hex("769656").unwrap();
    for y in (GRID_BLOCK_SIZE as i32 / -2)..(GRID_BLOCK_SIZE as i32 / 2) {
        for x in (GRID_BLOCK_SIZE as i32 / -2)..(GRID_BLOCK_SIZE as i32 / 2) {
            let color = if (x + y) % 2 == 0 { white } else { black };
            let transform =
                Transform::from_xyz(x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE, 0.0);
            commands.spawn(SpriteBundle {
                transform,
                sprite: Sprite {
                    color,
                    custom_size: Some(vec2(SQUARE_SIZE, SQUARE_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

impl BoardRecource {
    pub fn move_piece(&mut self, src: usize, dst: usize) -> Vec<(Entity, usize)> {
        // println!("from {:?} to {:?}", to_cord_index(src), to_cord_index(dst));
        let mut new_pieces_position = Vec::new();

        if !self.is_move_legal(src, dst) {
            return new_pieces_position;
        }
        self.grid[src].as_mut().unwrap().set_has_moved();
        self.change_turn();
        let src_id = self.grid[src].unwrap().get_id().unwrap();
        if let Some(piece) = self.grid[dst] {
            let dst_id = piece.get_id().unwrap();
            new_pieces_position.push((dst_id, src));
        }
        new_pieces_position.push((src_id, dst));
        self.grid.swap(src, dst);

        new_pieces_position
    }

    pub fn get_possible_moves(&mut self, src: usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();
        for i in 0..(TILE_NUMBER * TILE_NUMBER) {
            if self.is_move_legal(src, i) {
                possible_moves.push(i);
            }
        }
        possible_moves
    }

    pub fn is_move_legal(&mut self, src: usize, dst: usize) -> bool {
        let (src_col, src_row) = tuple_as!(to_cord_index(src), i32);
        let (dst_col, dst_row) = tuple_as!(to_cord_index(dst), i32);
        if (src_col, src_row) == (dst_col, dst_row) {
            return false;
        }
        if let Some(piece) = self.grid[src] {
            if self.turn != piece.get_color() {
                return false;
            }
            match piece.get_type() {
                PieceTypes::Pawn => {
                    #[allow(unused_assignments)]
                    let mut distance = 0i32;
                    if piece.get_color() {
                        distance = src_row - dst_row;
                    } else {
                        distance = dst_row - src_row;
                    }
                    if let Some(target) = self.grid[dst] {
                        if (target.get_color() == piece.get_color())
                            || ((src_col - dst_col).abs() != 1)
                            || (!(0..=1).contains(&distance))
                        {
                            return false;
                        }
                        return true;
                    } else {
                        if (src_col != dst_col)
                            || (distance < 0)
                            || (piece.has_moved() && distance > 1)
                            || (!piece.has_moved() && distance > 2)
                        {
                            return false;
                        }
                        if !piece.has_moved() && distance == 2 {
                            return (self.grid[to_board_index(
                                src_col as usize,
                                (src_row - ((src_row - dst_row) / 2)) as usize,
                            )])
                            .is_none();
                        }
                        return true;
                    }
                }
                PieceTypes::Rook => {
                    if let Some(target) = self.grid[dst] {
                        if target.get_color() == piece.get_color() {
                            return false;
                        }
                    }
                    if src_row == dst_row {
                        return range_inclusive(src_col as usize, dst_col as usize)
                            // .skip(1)
                            .map(|pos| self.grid[to_board_index(pos, src_row as usize)].is_some())
                            .all(|x| !x);
                    }
                    if src_col == dst_col {
                        return range_inclusive(src_row as usize, dst_row as usize)
                            // .skip(1)
                            .map(|pos| self.grid[to_board_index(src_col as usize, pos)].is_some())
                            .all(|x| !x);
                    }
                    return false;
                }
                PieceTypes::Knight => {
                    if let Some(target) = self.grid[dst] {
                        if target.get_color() == piece.get_color() {
                            return false;
                        }
                    }
                    if ((src_row - dst_row).abs() == 2 && (src_col - dst_col).abs() == 1)
                        || ((src_row - dst_row).abs() == 1 && (src_col - dst_col).abs() == 2)
                    {
                        return true;
                    }
                    return false;
                }
                PieceTypes::Bishop => {
                    if let Some(target) = self.grid[dst] {
                        if target.get_color() == piece.get_color() {
                            return false;
                        }
                    }
                    if (src_row - dst_row).abs() == (src_col - dst_col).abs() {
                        return range_inclusive(
                            0_usize,
                            (src_col - dst_col).unsigned_abs() as usize,
                        )
                        // .skip(1)
                        .map(|pos| {
                            if src_row - dst_row > 0 && src_col - dst_col > 0 {
                                return self.grid[to_board_index(
                                    src_col as usize - pos,
                                    src_row as usize - pos,
                                )]
                                .is_some();
                            }
                            if src_row - dst_row < 0 && src_col - dst_col > 0 {
                                return self.grid[to_board_index(
                                    src_col as usize - pos,
                                    src_row as usize + pos,
                                )]
                                .is_some();
                            }
                            if src_row - dst_row > 0 && src_col - dst_col < 0 {
                                return self.grid[to_board_index(
                                    src_col as usize + pos,
                                    src_row as usize - pos,
                                )]
                                .is_some();
                            }
                            if src_row - dst_row < 0 && src_col - dst_col < 0 {
                                return self.grid[to_board_index(
                                    src_col as usize + pos,
                                    src_row as usize + pos,
                                )]
                                .is_some();
                            }
                            false
                            // need fix
                        })
                        .all(|x| !x);
                    }
                    return false;
                }
                PieceTypes::Queen => {
                    if let Some(target) = self.grid[dst] {
                        if target.get_color() == piece.get_color() {
                            return false;
                        }
                    }
                    if (src_row - dst_row).abs() == (src_col - dst_col).abs() {
                        return range_inclusive(
                            0_usize,
                            (src_col - dst_col).unsigned_abs() as usize,
                        )
                        // .skip(1)
                        .map(|pos| {
                            if src_row - dst_row > 0 && src_col - dst_col > 0 {
                                return self.grid[to_board_index(
                                    src_col as usize - pos,
                                    src_row as usize - pos,
                                )]
                                .is_some();
                            }
                            if src_row - dst_row < 0 && src_col - dst_col > 0 {
                                return self.grid[to_board_index(
                                    src_col as usize - pos,
                                    src_row as usize + pos,
                                )]
                                .is_some();
                            }
                            if src_row - dst_row > 0 && src_col - dst_col < 0 {
                                return self.grid[to_board_index(
                                    src_col as usize + pos,
                                    src_row as usize - pos,
                                )]
                                .is_some();
                            }
                            if src_row - dst_row < 0 && src_col - dst_col < 0 {
                                return self.grid[to_board_index(
                                    src_col as usize + pos,
                                    src_row as usize + pos,
                                )]
                                .is_some();
                            }
                            false
                        })
                        .all(|x| !x);
                    }
                    if src_row == dst_row {
                        return range_inclusive(src_col as usize, dst_col as usize)
                            // .skip(1)
                            .map(|pos| self.grid[to_board_index(pos, src_row as usize)].is_some())
                            .all(|x| !x);
                    }
                    if src_col == dst_col {
                        return range_inclusive(src_row as usize, dst_row as usize)
                            // .skip(1)
                            .map(|pos| self.grid[to_board_index(src_col as usize, pos)].is_some())
                            .all(|x| !x);
                    }
                    return false;
                }
                PieceTypes::King => {
                    if let Some(target) = self.grid[dst] {
                        if target.get_color() == piece.get_color() {
                            return false;
                        }
                    }
                    if (src_row - dst_row).abs() > 1 || (src_col - dst_col).abs() > 1 {
                        return false;
                    }
                    return true;
                }
                PieceTypes::Jester => {
                    return true;
                }
                PieceTypes::Amazon => {
                    return true;
                }
                PieceTypes::GrandCommander => {
                    return true;
                }
                PieceTypes::Abbess => {
                    return true;
                }
            }
        }
        false
    }
}

pub fn get_best_next_move(root_board: &mut BoardRecource) -> Option<usize> {
    let mut cloned_board = root_board.clone();
    let mut best_move = 0_usize;
    for (index, piece_iter) in cloned_board.clone().grid.iter().enumerate() {
        if let Some(piece) = piece_iter {
            let possible_moves = cloned_board.get_possible_moves(index);
            if let Some(pos) = possible_moves.first().copied() {
                return Some(pos);
            }
        }
    }
    None
}
