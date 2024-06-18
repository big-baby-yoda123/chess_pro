use bevy::{app::App, math::vec2, prelude::*};
use bevy_pkv::PkvStore;

use crate::create_piece;
use crate::{states::AppState, tuple_as};

use super::{
    piece::{MovementsRules, Piece, PieceTypes, Rules},
    GRID_BLOCK_SIZE, SQUARE_SIZE, TILE_NUMBER,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardRecource>()
            .add_systems(
                OnEnter(AppState::InGame),
                (setup_board, spawn_grid.after(setup_board)),
            )
            .insert_resource(PkvStore::new("ChessPro", "ChessPro"));
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

fn setup_board(mut board: ResMut<BoardRecource>, mut pkv: ResMut<PkvStore>) {
    board.turn = true;
    board.grid = vec![None; TILE_NUMBER * TILE_NUMBER];
    // if let Ok(saved_board) = pkv.get::<Vec<Option<Piece>>>("default_board") {
    //     board.grid = saved_board;
    // } else {
    //     let saved_board: Vec<Option<Piece>> = vec![None; TILE_NUMBER * TILE_NUMBER];
    //     pkv.set("board", &saved_board)
    //         .expect("failed to store user");
    // }

    for i in 0..TILE_NUMBER {
        board.grid[to_board_index(i, TILE_NUMBER - 2)] = create_piece!(white PieceTypes::Pawn);
        board.grid[to_board_index(i, 1)] = create_piece!(black PieceTypes::Pawn);
    }
    board.grid[to_board_index(0, 0)] = create_piece!(black PieceTypes::Rook);
    board.grid[to_board_index(TILE_NUMBER - 1, 0)] = create_piece!(black PieceTypes::Rook);
    board.grid[to_board_index(TILE_NUMBER - 1, TILE_NUMBER - 1)] =
        create_piece!(white PieceTypes::Rook);
    board.grid[to_board_index(0, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Rook);

    board.grid[to_board_index(1, 0)] = create_piece!(black PieceTypes::Bishop);
    board.grid[to_board_index(TILE_NUMBER - 2, 0)] = create_piece!(black PieceTypes::Bishop);
    board.grid[to_board_index(TILE_NUMBER - 2, TILE_NUMBER - 1)] =
        create_piece!(white PieceTypes::Bishop);
    board.grid[to_board_index(1, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Bishop);

    board.grid[to_board_index(2, 0)] = create_piece!(black PieceTypes::Knight);
    board.grid[to_board_index(TILE_NUMBER - 3, 0)] = create_piece!(black PieceTypes::Knight);
    board.grid[to_board_index(TILE_NUMBER - 3, TILE_NUMBER - 1)] =
        create_piece!(white PieceTypes::Knight);
    board.grid[to_board_index(2, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Knight);

    board.grid[to_board_index(3, 0)] = create_piece!(black PieceTypes::King);
    board.grid[to_board_index(3, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::King);

    board.grid[to_board_index(4, 0)] = create_piece!(black PieceTypes::Queen);
    board.grid[to_board_index(4, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Queen);

    board.grid[to_board_index(5, 0)] = create_piece!(black PieceTypes::Jester);
    board.grid[to_board_index(5, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Jester);

    board.grid[to_board_index(6, 0)] = create_piece!(black PieceTypes::GrandCommander);
    board.grid[to_board_index(6, TILE_NUMBER - 1)] =
        create_piece!(white PieceTypes::GrandCommander);

    board.grid[to_board_index(7, 0)] = create_piece!(black PieceTypes::Amazon);
    board.grid[to_board_index(7, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Amazon);

    board.grid[to_board_index(8, 0)] = create_piece!(black PieceTypes::Abbess);
    board.grid[to_board_index(8, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::Abbess);

    board.grid[to_board_index(9, 0)] = create_piece!(black PieceTypes::ShortRook);
    board.grid[to_board_index(9, TILE_NUMBER - 1)] = create_piece!(white PieceTypes::ShortRook);
}

pub fn to_board_index(x: usize, y: usize) -> usize {
    TILE_NUMBER * y + x
}

pub fn to_cord_index(index: usize) -> (usize, usize) {
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

#[derive(Debug, Clone, Copy)]
pub enum PieceResult {
    None(),
    Moved(usize),
    Captured(),
    Promoted(usize, PieceTypes, bool),
    Changed(PieceTypes, bool),
}

impl BoardRecource {
    pub fn move_piece(&mut self, src: usize, dst: usize) -> Vec<(Entity, PieceResult)> {
        let mut new_pieces_position = Vec::new();
        let (move_check, move_result) = self.handle_move(src, dst);

        if !move_check {
            return new_pieces_position;
        }
        if self.will_be_in_check(src, dst) {
            return new_pieces_position;
        }
        self.grid[src].as_mut().unwrap().set_has_moved();
        self.change_turn();

        let src_id = self.grid[src].unwrap().get_id().unwrap();
        new_pieces_position.push((src_id, move_result[0]));
        if let Some(piece) = self.grid[dst] {
            let dst_id = piece.get_id().unwrap();
            new_pieces_position.push((dst_id, move_result[1]));
            self.grid[dst] = None;
        }
        self.grid.swap(src, dst);

        new_pieces_position
    }

    pub fn get_possible_moves(&mut self, src: usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();
        for i in 0..(TILE_NUMBER * TILE_NUMBER) {
            if self.is_move_legal(src, i) && !self.will_be_in_check(src, i) {
                possible_moves.push(i);
            }
        }
        possible_moves
    }

    fn handle_move(&mut self, src: usize, dst: usize) -> (bool, [PieceResult; 2]) {
        let legal_move = self.is_move_legal(src, dst);
        let mut src_result = PieceResult::Moved(dst);
        let mut dst_result = PieceResult::Captured();
        let (src_col, src_row) = tuple_as!(to_cord_index(src), i32);
        let (dst_col, dst_row) = tuple_as!(to_cord_index(dst), i32);
        if let Some(piece) = &self.grid[src] {
            match piece.get_type() {
                PieceTypes::Pawn => {
                    if dst_row == 0 || dst_row == (TILE_NUMBER as i32 - 1) {
                        src_result =
                            PieceResult::Promoted(dst, PieceTypes::Queen, piece.get_color());
                        self.grid[src].unwrap().promote(PieceTypes::Queen);
                    }
                }
                PieceTypes::Rook => {}
                PieceTypes::Knight => {}
                PieceTypes::Bishop => {}
                PieceTypes::Queen => {}
                PieceTypes::King => {}
                PieceTypes::Jester => {}
                PieceTypes::Amazon => {}
                PieceTypes::GrandCommander => {}
                PieceTypes::Abbess => {}
                PieceTypes::ShortRook => {}
                _ => {}
            };
        }
        (legal_move, [src_result, dst_result])
    }

    #[allow(clippy::collapsible_if)]
    fn is_move_legal(&self, src: usize, dst: usize) -> bool {
        let (src_col, src_row) = tuple_as!(to_cord_index(src), i32);
        let (dst_col, dst_row) = tuple_as!(to_cord_index(dst), i32);
        if src == dst {
            return false;
        }
        if let Some(piece) = self.grid[src] {
            if self.turn != piece.get_color() {
                return false;
            }
            let rules = piece.get_rules();
            let movement_rules = rules.movment_rules;
            let delta_x = dst_col - src_col;
            let delta_y = dst_row - src_row;
            let max_distance = rules.max_distance.unwrap_or(i32::MAX);
            let distance = delta_x.pow(2) + delta_y.pow(2);

            if let Some(target) = self.grid[dst] {
                if target.get_color() == piece.get_color() {
                    return false;
                }
            }

            if !rules.multiple_direction_rule
                && ((piece.get_color() && delta_y > 0) || (!piece.get_color() && delta_y < 0))
            {
                return false;
            }

            if !rules.step_over_rule && !can_step_over(self.grid.clone(), src, dst, true) {
                return false;
            }

            if movement_rules.contains(MovementsRules::PAWN_MOVMENT) {
                let mut pawn_max_distance = max_distance;
                if piece.has_moved() {
                    pawn_max_distance = max_distance - 1;
                }
                if (delta_y).abs() <= pawn_max_distance {
                    if let Some(target) = self.grid[dst] {
                        return (target.get_color() != piece.get_color())
                            && (delta_x.abs() == 1)
                            && (delta_y.abs() == 1);
                    }
                    return delta_x == 0;
                }
            }
            if movement_rules.contains(MovementsRules::HORIZONTAL_MOVMENT) {
                if delta_y == 0 && (delta_x).abs() <= max_distance {
                    return true;
                }
            }
            if movement_rules.contains(MovementsRules::VERTICAL_MOVMENT) {
                if delta_x == 0 && (delta_y).abs() <= max_distance {
                    return true;
                }
            }
            if movement_rules.contains(MovementsRules::DIAGONAL_MOVMENT) {
                if delta_x.abs() == delta_y.abs() && delta_x.abs() < max_distance {
                    return true;
                }
            }
            if movement_rules.contains(MovementsRules::SHIFT_STEP_MOVMENT) {
                if distance % 5 == 0 && distance <= max_distance {
                    return true;
                }
            }
        }
        false
    }

    fn will_be_in_check(&self, src: usize, dst: usize) -> bool {
        let mut temp_board = self.clone();
        if !temp_board.is_move_legal(src, dst) {
            return false;
        }
        temp_board.grid[src].as_mut().unwrap().set_has_moved();

        if let Some(piece) = temp_board.grid[dst] {
            temp_board.grid[dst] = None;
        }
        temp_board.grid.swap(src, dst);
        let king_pos = temp_board.grid.iter().position(|p| {
            if let Some(piece) = p {
                (piece.get_color() == temp_board.turn) && (piece.get_type() == PieceTypes::King)
            } else {
                false
            }
        });
        temp_board.change_turn();
        // println!("king pos: {:?}", king_pos);
        if let Some(king) = king_pos {
            for i in 0..(TILE_NUMBER * TILE_NUMBER) {
                if temp_board.is_move_legal(i, king) {
                    return true;
                }
            }
            return false;
        }
        true
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

pub fn can_step_over(board: Vec<Option<Piece>>, from: usize, to: usize, start: bool) -> bool {
    if from == to {
        return true;
    }
    if board[from].is_some() && !start {
        return false;
    }
    let (src_col, src_row) = tuple_as!(to_cord_index(from), i32);
    let (dst_col, dst_row) = tuple_as!(to_cord_index(to), i32);
    let delta_x = dst_col - src_col;
    let delta_y = dst_row - src_row;
    let (added_x, added_y) = (
        delta_x
            .checked_div(delta_y.abs())
            .unwrap_or(delta_x.clamp(-1, 1))
            .clamp(-1, 1),
        delta_y
            .checked_div(delta_x.abs())
            .unwrap_or(delta_y.clamp(-1, 1))
            .clamp(-1, 1),
    );
    can_step_over(
        board,
        to_board_index((src_col + added_x) as usize, (src_row + added_y) as usize),
        to,
        false,
    )
}

#[macro_export]
macro_rules! create_piece {
    (white $piece_type: expr) => {
        Some(Piece::new($piece_type, true, Rules::get_rules($piece_type)))
    };
    (black $piece_type: expr) => {
        Some(Piece::new(
            $piece_type,
            false,
            Rules::get_rules($piece_type),
        ))
    };
    ($color: expr, $piece_type: expr) => {
        Some(Piece::new(
            $piece_type,
            $color,
            Rules::get_rules($piece_type),
        ))
    };
}
