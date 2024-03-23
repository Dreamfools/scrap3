use crate::line::{LineMatcherSettings, LinesBoardMatch};
use crate::rect_board::RectBoard;
use crate::MatchColor;
use colored::Colorize;
use insta::assert_snapshot;
use itertools::Itertools;
use ndshape::RuntimeShape;
use nohash_hasher::IntSet;
use rstest::rstest;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct CharGem(char);

impl MatchColor for CharGem {
    fn matches(&self, other: &Self) -> bool {
        self.0 == other.0 || self.0 == '*' || other.0 == '*'
    }

    fn can_start_match(&self) -> bool {
        self.0 != '*' && self.0 != '-'
    }

    fn hint_is_unmatchable(&self) -> bool {
        self.0 == '-'
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

type CharBoard = RectBoard<CharGem, CharGem>;
type S = LineMatcherSettings;

fn board_from_str(board: &str) -> CharBoard {
    let lines: Vec<&str> = board.lines().map(|l| l.trim()).collect();
    let height = lines.len();
    let width = lines[0].len();
    let shape = RuntimeShape::<usize, 2>::new([width, height]);
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
    CharBoard::new(shape, board)
}

fn color_char(c: char) -> String {
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

fn display(board: &CharBoard, colored: bool) -> String {
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

fn visualize_snapshot(
    name: String,
    mut board: CharBoard,
    mut matches: Vec<LinesBoardMatch<CharGem>>,
    colored: bool,
) -> String {
    let mut text = format!("{name}\nBoard: \n");

    text += &display(&board, colored);

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
            text += &format!("\nMatch #{i}:\n{}", display(&cloned, colored));
            to_remove.extend(&m.cells);
        }
        for (i, x) in board.board.iter_mut().enumerate() {
            if to_remove.contains(&i) {
                *x = CharGem(' ');
            }
        }
        text += &format!("\nRemaining cells:\n{}", display(&board, colored));
    }
    text
}

#[rstest]
#[case("Horizontal line", "rrr")]
#[case("Vertical line", "r\nr\nr")]
#[case("Incomplete horizontal line", "rr-")]
#[case("Incomplete vertical line", "r\nr\n-")]
pub fn common_line3_tests(#[case] name: String, #[case] board: String) {
    let board = board_from_str(&board);
    let matches = board.find_matches(S::common_match3());
    assert_snapshot!(
        name.clone(),
        visualize_snapshot(name, board, matches, false)
    );
}
