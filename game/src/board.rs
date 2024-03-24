use crate::{screen_rect, CutRect};
use comfy::{draw_circle, world, BLACK, BLUE, DARKPURPLE, GREEN, RED, YELLOW};
use match3::rect_board::RectBoard;
use match3::{Gem as MatchGem, MatchColor, Shape, SimpleGem};
use tinyrand::{RandRange, StdRand};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum GemColor {
    #[default]
    Empty,
    Red,
    Green,
    Blue,
    Light,
    Dark,
}

pub static GEM_COLORS: [GemColor; 5] = [
    GemColor::Red,
    GemColor::Green,
    GemColor::Blue,
    GemColor::Light,
    GemColor::Dark,
];

impl MatchColor for GemColor {
    fn matches(&self, other: &Self) -> bool {
        self == other
    }

    fn can_start_match(&self) -> bool {
        self != &GemColor::Empty
    }
}

pub type Gem = SimpleGem<GemColor>;

pub type GemBoard = RectBoard<Gem>;

pub fn draw_gem(gem: &Gem, rect: CutRect) {
    let rect = rect.trim_to_aspect_ratio(1.0);
    let center = rect.center().into();
    let radius = rect.width() / 2.0;
    let color = match gem.color() {
        GemColor::Empty => BLACK,
        GemColor::Red => RED,
        GemColor::Green => GREEN,
        GemColor::Blue => BLUE,
        GemColor::Light => YELLOW,
        GemColor::Dark => DARKPURPLE,
    };
    // info!(
    //     "Drawing gem; gem={:?} center={:?}, r={:?}, color={:?}",
    //     gem, center, radius, color,
    // );
    draw_circle(center, radius, color, 1)
}

#[derive(Debug, Copy, Clone)]
pub enum BoardState {
    Idle,
    Moving,
    Refilling,
    Matching,
}

#[inline_tweak::tweak_fn]
pub fn draw_board() {
    for (_e, board) in world().query::<&GemBoard>().iter() {
        let [width, height] = board.shape.as_array();

        let grid = screen_rect()
            .trim_to_aspect_ratio(width as f32 / height as f32)
            .grid(width, height, 0.25, 0.25);

        for (i, gem) in board.board.iter().enumerate() {
            let rect = grid[i];
            draw_gem(gem, rect)
        }
    }
}

pub fn random_board(width: usize, height: usize) -> (RectBoard<Gem>, BoardState) {
    let mut rand = StdRand::default();
    let mut board = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        let i = rand.next_range(0..GEM_COLORS.len());
        board.push(SimpleGem(GEM_COLORS[i]));
    }
    let board = GemBoard::new(width, height, board);
    (board, BoardState::Idle)
}
