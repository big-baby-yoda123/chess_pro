use bevy::{app::App, math::vec2, prelude::*};

use crate::{states::AppState, tuple_as};

use super::{
    piece::{MovementsRules, Piece, PieceTypes, Rules},
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
        board.grid[to_board_index(i, TILE_NUMBER - 2)] = Some(Piece::new(
            PieceTypes::Pawn,
            true,
            Rules::new(MovementsRules::PAWN_MOVMENT, false, Some(3), false),
        ));
        board.grid[to_board_index(i, 1)] = Some(Piece::new(
            PieceTypes::Pawn,
            false,
            Rules::new(MovementsRules::PAWN_MOVMENT, false, Some(3), false),
        ));
    }
    board.grid[to_board_index(0, 0)] = Some(Piece::new(
        PieceTypes::Rook,
        false,
        Rules::new(
            MovementsRules::HORIZONTAL_MOVMENT | MovementsRules::VERTICAL_MOVMENT,
            false,
            None,
            true,
        ),
    ));
    board.grid[to_board_index(TILE_NUMBER - 1, 0)] = Some(Piece::new(
        PieceTypes::Rook,
        false,
        Rules::new(
            MovementsRules::HORIZONTAL_MOVMENT | MovementsRules::VERTICAL_MOVMENT,
            false,
            None,
            true,
        ),
    ));
    board.grid[to_board_index(TILE_NUMBER - 1, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Rook,
        true,
        Rules::new(
            MovementsRules::HORIZONTAL_MOVMENT | MovementsRules::VERTICAL_MOVMENT,
            false,
            None,
            true,
        ),
    ));
    board.grid[to_board_index(0, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Rook,
        true,
        Rules::new(
            MovementsRules::HORIZONTAL_MOVMENT | MovementsRules::VERTICAL_MOVMENT,
            false,
            None,
            true,
        ),
    ));

    board.grid[to_board_index(1, 0)] = Some(Piece::new(
        PieceTypes::Knight,
        false,
        Rules::new(MovementsRules::SHIFT_STEP_MOVMENT, false, Some(5), true),
    ));
    board.grid[to_board_index(TILE_NUMBER - 2, 0)] = Some(Piece::new(
        PieceTypes::Knight,
        false,
        Rules::new(MovementsRules::SHIFT_STEP_MOVMENT, false, Some(5), true),
    ));
    board.grid[to_board_index(TILE_NUMBER - 2, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Knight,
        true,
        Rules::new(MovementsRules::SHIFT_STEP_MOVMENT, false, Some(5), true),
    ));
    board.grid[to_board_index(1, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Knight,
        true,
        Rules::new(MovementsRules::SHIFT_STEP_MOVMENT, false, Some(5), true),
    ));

    board.grid[to_board_index(2, 0)] = Some(Piece::new(
        PieceTypes::Bishop,
        false,
        Rules::new(MovementsRules::DIAGONAL_MOVMENT, false, None, true),
    ));
    board.grid[to_board_index(TILE_NUMBER - 3, 0)] = Some(Piece::new(
        PieceTypes::Bishop,
        false,
        Rules::new(MovementsRules::DIAGONAL_MOVMENT, false, None, true),
    ));
    board.grid[to_board_index(TILE_NUMBER - 3, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Bishop,
        true,
        Rules::new(MovementsRules::DIAGONAL_MOVMENT, false, None, true),
    ));
    board.grid[to_board_index(2, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Bishop,
        true,
        Rules::new(MovementsRules::DIAGONAL_MOVMENT, false, None, true),
    ));

    board.grid[to_board_index(3, 0)] = Some(Piece::new(
        PieceTypes::King,
        false,
        Rules::new(
            MovementsRules::DIAGONAL_MOVMENT
                | MovementsRules::HORIZONTAL_MOVMENT
                | MovementsRules::VERTICAL_MOVMENT,
            false,
            Some(1),
            true,
        ),
    ));
    board.grid[to_board_index(3, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::King,
        true,
        Rules::new(
            MovementsRules::DIAGONAL_MOVMENT
                | MovementsRules::HORIZONTAL_MOVMENT
                | MovementsRules::VERTICAL_MOVMENT,
            false,
            Some(1),
            true,
        ),
    ));

    board.grid[to_board_index(4, 0)] = Some(Piece::new(
        PieceTypes::Queen,
        false,
        Rules::new(
            MovementsRules::DIAGONAL_MOVMENT
                | MovementsRules::HORIZONTAL_MOVMENT
                | MovementsRules::VERTICAL_MOVMENT,
            false,
            None,
            true,
        ),
    ));
    board.grid[to_board_index(4, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Queen,
        true,
        Rules::new(
            MovementsRules::DIAGONAL_MOVMENT
                | MovementsRules::HORIZONTAL_MOVMENT
                | MovementsRules::VERTICAL_MOVMENT,
            false,
            None,
            true,
        ),
    ));

    board.grid[to_board_index(5, 0)] = Some(Piece::new(
        PieceTypes::Jester,
        false,
        Rules::new(MovementsRules::PAWN_MOVMENT, true, Some(2), true),
    ));
    board.grid[to_board_index(5, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Jester,
        true,
        Rules::new(MovementsRules::PAWN_MOVMENT, true, Some(2), true),
    ));

    board.grid[to_board_index(6, 0)] = Some(Piece::new(
        PieceTypes::Amazon,
        false,
        Rules::new(
            MovementsRules::DIAGONAL_MOVMENT
                | MovementsRules::HORIZONTAL_MOVMENT
                | MovementsRules::VERTICAL_MOVMENT,
            false,
            Some(2),
            true,
        ),
    ));
    board.grid[to_board_index(6, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Amazon,
        true,
        Rules::new(
            MovementsRules::DIAGONAL_MOVMENT
                | MovementsRules::HORIZONTAL_MOVMENT
                | MovementsRules::VERTICAL_MOVMENT,
            false,
            Some(2),
            true,
        ),
    ));

    board.grid[to_board_index(7, 0)] = Some(Piece::new(
        PieceTypes::Abbess,
        false,
        Rules::new(MovementsRules::PAWN_MOVMENT, true, Some(2), true),
    ));
    board.grid[to_board_index(7, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::Abbess,
        true,
        Rules::new(MovementsRules::PAWN_MOVMENT, true, Some(2), true),
    ));

    board.grid[to_board_index(8, 0)] = Some(Piece::new(
        PieceTypes::GrandCommander,
        false,
        Rules::new(MovementsRules::SHIFT_STEP_MOVMENT, true, Some(24), true),
    ));
    board.grid[to_board_index(8, TILE_NUMBER - 1)] = Some(Piece::new(
        PieceTypes::GrandCommander,
        true,
        Rules::new(MovementsRules::SHIFT_STEP_MOVMENT, true, Some(24), true),
    ));
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
        if src == dst {
            return false;
        }
        if let Some(piece) = self.grid[src] {
            // if self.turn != piece.get_color() {
            //     return false;
            // }
            let rules = piece.get_rules();
            let movement_rules = rules.movment_rules;
            let delta_x = dst_col - src_col;
            let delta_y = dst_row - src_row;
            let max_distance = rules.max_distance.unwrap_or(i32::MAX);

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
                        return (target.get_color() != piece.get_color()) && (delta_x.abs() == 1);
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
                if delta_x.abs() == delta_y.abs() && (delta_x).abs() <= max_distance {
                    return true;
                }
            }
            if movement_rules.contains(MovementsRules::SHIFT_STEP_MOVMENT) {
                let distance = delta_x.pow(2) + delta_y.pow(2);
                if distance % 5 == 0 && distance <= max_distance {
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
