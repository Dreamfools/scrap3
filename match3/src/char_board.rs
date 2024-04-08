use crate::rect_board::RectBoard;
use crate::refilling::{GravityRefill, GravityRefillAction, RefillableGem};
use crate::{BoardGem, BoardMatch, MatchColor};
use colored::Colorize;
use itertools::Itertools;
use nohash_hasher::IntSet;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct CharGem(pub char);

impl MatchColor for CharGem {
    fn matches(&self, other: &Self) -> bool {
        self.0 == other.0 || self.0 == '*' || other.0 == '*'
    }

    fn can_start_match(&self) -> bool {
        self.0 != '*' && self.0 != '-' && self.0 != ' '
    }

    fn hint_is_unmatchable(&self) -> bool {
        self.0 == '-' || self.0 == ' '
    }
}

impl BoardGem for CharGem {
    type Color = CharGem;

    fn color(&self) -> Self::Color {
        *self
    }
}

impl RefillableGem for CharGem {
    fn is_empty(&self) -> bool {
        self.0 == ' ' || self.0 == '-'
    }
}

impl AsRef<CharGem> for CharGem {
    fn as_ref(&self) -> &CharGem {
        self
    }
}

impl From<char> for CharGem {
    fn from(c: char) -> Self {
        CharGem(c)
    }
}

pub type CharBoard = RectBoard<CharGem>;

pub fn color_char(c: char) -> String {
    match c {
        'r' => "r".red().on_red().to_string(),
        'g' => "g".bright_green().on_bright_green().to_string(),
        'b' => "b".bright_blue().on_bright_blue().to_string(),
        'w' => "w".bright_white().on_bright_white().to_string(),
        'y' => "y".bright_yellow().on_bright_yellow().to_string(),
        'p' => "p".bright_purple().on_bright_purple().to_string(),
        'o' => "o".yellow().on_yellow().to_string(),
        '-' => "-".black().on_black().to_string(),
        _ => c.to_string(),
    }
}

pub fn board_from_str(board: &str) -> CharBoard {
    let lines: Vec<&str> = board.lines().map(|l| l.trim()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let board = lines
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            assert_eq!(
                line.len(),
                width,
                "All lines must have the same length, at line {i}"
            );
            line.chars().map(CharGem::from)
        })
        .collect();
    CharBoard::new(width, height, board)
}

pub fn display_board(board: &CharBoard, colored: bool) -> String {
    board
        .board
        .iter()
        .map(|c| {
            if colored {
                color_char(c.0)
            } else {
                c.0.to_string()
            }
        })
        .chunks(board.width())
        .into_iter()
        .map(|mut line| line.join("").to_string())
        .join("\n")
}

pub fn pretty_print_board(board: &CharBoard, colored: bool) -> String {
    let body = board
        .board
        .iter()
        .map(|c| {
            if colored {
                color_char(c.0)
            } else {
                c.0.to_string()
            }
        })
        .chunks(board.width())
        .into_iter()
        .map(|mut line| format!("│{}│", line.join("")))
        .join("\n");

    format!(
        "╭{:─>width$}╮\n{}\n╰{:─>width$}╯",
        "",
        body,
        "",
        width = board.width()
    )
}

pub fn visualize_and_apply_matches(
    name: String,
    board: &mut CharBoard,
    mut matches: Vec<BoardMatch<CharGem>>,
    colored: bool,
) -> String {
    let mut text = format!("{name}\nBoard: \n");

    text += &pretty_print_board(&board, colored);

    // Result order is not guaranteed, so we sort it
    matches.sort_by(|a, b| a.cells.cmp(&b.cells).then(a.color.0.cmp(&b.color.0)));

    if matches.is_empty() {
        text += "\nNo matches"
    } else {
        let mut to_remove: IntSet<usize> = Default::default();
        for (i, m) in matches.iter().enumerate() {
            let mut cloned = board.clone();
            for (i, x) in cloned.board.iter_mut().enumerate() {
                if !m.cells.contains(&i) {
                    *x = CharGem(' ');
                }
            }
            text += &format!(
                "\nMatch #{i} - {}:\n{}",
                m.color.0,
                pretty_print_board(&cloned, colored)
            );
            to_remove.extend(m.cells.iter());
        }
        for (i, x) in board.board.iter_mut().enumerate() {
            if to_remove.contains(&i) {
                *x = CharGem(' ');
            }
        }
        text += &format!(
            "\nRemaining cells:\n{}",
            pretty_print_board(&board, colored)
        );
    }
    text
}

pub fn visualise_and_apply_gravity(board: &mut CharBoard) -> String {
    let actions = GravityRefill::refill(&board.board, board.vertical_lines());
    let (falling, refilling) = actions
        .iter()
        .partition::<Vec<_>, _>(|a| matches!(a, GravityRefillAction::Fall(_)));
    for fall in falling {
        fall.apply(&mut board.board, |_| unreachable!())
    }
    let mut result = format!("\nAfter Gravity:\n{}", pretty_print_board(board, false));
    for refill in refilling {
        refill.apply(&mut board.board, |_| '#'.into())
    }
    result += &format!("\nAfter Refill:\n{}", pretty_print_board(board, false));
    result
}
