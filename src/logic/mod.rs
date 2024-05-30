use bevy::{
    app::{App, Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
};

use self::{board::BoardPlugin, piece::PiecePlugin};

pub mod board;
pub mod piece;

const GRID_BLOCK_SIZE: f32 = 16.0;
const TILE_NUMBER: usize = GRID_BLOCK_SIZE as usize;
const GRID_SIZE: f32 = 840.0;
const SQUARE_SIZE: f32 = GRID_SIZE / GRID_BLOCK_SIZE;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BoardPlugin).add_plugins(PiecePlugin);
    }
}

pub fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>> = if b > a {
        Box::new((a..b).skip(1))
    } else {
        Box::new((b..a).skip(1).rev())
    };
    x
}

#[macro_export]
macro_rules! tuple_as {
    ($t: expr, $ty: ident) => {{
        let (a, b) = $t;
        let a = a as $ty;
        let b = b as $ty;
        (a, b)
    }};
    ($t: expr, ($ty: ident)) => {{
        let (a, b) = $t;
        let a = a as $ty;
        let b = b as $ty;
        (a, b)
    }};
    ($t: expr, ($($ty: ident),*)) => {{
        let ($($ty,)*) = $t;
        ($($ty as $ty,)*)
    }}}
