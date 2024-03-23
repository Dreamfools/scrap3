use crate::line::LineMatcherSettings;
use crate::rect_board::RectBoard;
use crate::{BoardMatch, MatchColor};
use colored::Colorize;
use insta::assert_snapshot;
use itertools::Itertools;
use ndshape::Shape;
use nohash_hasher::IntSet;
use proptest::prelude::{Just, Strategy};
use proptest::test_runner::{TestError, TestRunner};
use rstest::rstest;
use serde::Deserialize;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct CharGem(char);

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

#[derive(Deserialize)]
struct MatchSettings {
    line_size: Option<usize>,
    min_group_size: Option<usize>,
    merge_neighbours: Option<bool>,
}

fn check_path(prefix: &str, path: impl AsRef<Path>) {
    let path = path.as_ref();
    let test = std::fs::read_to_string(path).unwrap();
    let mut lines = test.lines().peekable();
    let name = lines.next().expect("Should have name").trim();
    let mut settings_variations: Vec<(&'static str, S)> = vec![];
    if let Some(peek) = lines.peek() {
        if peek.starts_with('{') {
            let mut match_settings = S::common_match3();
            let settings = serde_json5::from_str::<MatchSettings>(peek.trim()).unwrap();

            match_settings.line_size = settings.line_size.unwrap_or(match_settings.line_size);
            match_settings.min_group_size =
                settings.min_group_size.or(match_settings.min_group_size);
            match_settings.merge_neighbours = settings
                .merge_neighbours
                .unwrap_or(match_settings.merge_neighbours);

            if settings.merge_neighbours.is_none() {
                match_settings.merge_neighbours = false;
                let mut cloned = match_settings.clone();
                settings_variations.push(("(with merge_neighbours = false)", match_settings));
                cloned.merge_neighbours = true;
                settings_variations.push(("(with merge_neighbours = true)", cloned));
            } else {
                settings_variations.push(("", match_settings));
            }

            let _ = lines.next();
        } else {
            settings_variations.push(("(with merge_neighbours = false)", {
                let mut s = S::common_match3();
                s.merge_neighbours = false;
                s
            }));
            settings_variations.push(("(with merge_neighbours = true)", {
                let mut s = S::common_match3();
                s.merge_neighbours = true;
                s
            }));
        }
    } else {
        panic!("Board is empty")
    }
    let board = board_from_str(&lines.join("\n"));

    let variants = settings_variations
        .into_iter()
        .map(|(settings_name, settings)| {
            let board = board.clone();
            let matches = board.find_matches_linear(settings);
            (
                settings_name,
                visualize_snapshot(name.to_string(), board, matches, false),
            )
        })
        .collect::<Vec<(&'static str, String)>>();

    for i in 1..variants.len() {
        assert_eq!(
            variants[i].1, variants[0].1,
            "Got different results with {} and {}",
            variants[i].0, variants[0].0
        );
    }

    let file_name = path
        .file_name()
        .expect("Cases should have a file name")
        .to_string_lossy()
        .to_string();

    assert_snapshot!(format!("{}__{}", prefix, file_name), variants[0].1);
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

fn display_raw(board: &CharBoard, colored: bool) -> String {
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
    mut matches: Vec<BoardMatch<CharGem>>,
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
            text += &format!(
                "\nMatch #{i} - {}:\n{}",
                m.color.0,
                display(&cloned, colored)
            );
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
fn common_line3_file_tests(#[files("src/cases/common/*.txt")] path: PathBuf) {
    check_path("common", path);
}

#[rstest]
fn wildcard_line3_file_tests(#[files("src/cases/wildcard/*.txt")] path: PathBuf) {
    check_path("wildcard", path);
}

fn prop_board(size: usize) -> impl Strategy<Value = CharBoard> {
    (3..size, 3..size)
        .prop_flat_map(|(width, height)| {
            let size = width * height;
            (
                Just(width),
                Just(height),
                proptest::collection::vec(
                    proptest::char::ranges(Cow::Borrowed(&[
                        'r'..='r',
                        'g'..='g',
                        'b'..='b',
                        'w'..='w',
                        'p'..='p',
                        '*'..='*',
                    ])),
                    size,
                )
                .prop_map(|c| c.into_iter().map(CharGem).collect_vec()),
            )
        })
        .prop_map(|(w, h, board)| CharBoard::new(w, h, board))
}

#[test]
fn test_dev() {
    let b = board_from_str(
        "\
w*b
-bb",
    );
    let mut settings = S::common_match3();
    // settings.merge_neighbours = false;
    // settings.line_size = 2;
    let matches = b.find_matches_linear(settings);
    visualize_snapshot("DEV".to_string(), b, matches, false);
}

#[test]
fn random_tests() {
    let mut runner = TestRunner::default();
    let run_result = runner.run(&prop_board(64), |board| {
        let mut settings = S::common_match3();
        settings.line_size = 3;
        let matches = board.find_matches_linear(settings);
        let mut cloned = board.clone();
        for m in &matches {
            for &x in &m.cells {
                cloned.board[x].0 = ' '
            }
        }
        if let Some(err) = check_straight_matches(&cloned) {
            let visualized = visualize_snapshot(err, board, matches, false);
            panic!("{}", visualized)
        }

        Ok(())
    });
    match run_result {
        Ok(_) => {}
        Err(TestError::Abort(reason)) => {
            panic!("{}", reason)
        }
        Err(TestError::Fail(reason, board)) => {
            panic!("{}:\n{}", reason, display_raw(&board, false))
        }
    }
    // panic!("")
}

fn check_straight_matches(board: &CharBoard) -> Option<String> {
    let s = &board.shape;
    let b = &board.board;
    let [w, h] = s.as_array();
    for y in 0..h {
        for x in 0..w {
            let i = s.linearize([x, y]);
            if !b[i].can_start_match() {
                continue;
            }
            if x < w - 2 && b[i] == b[i + 1] && b[i] == b[i + 2] {
                return Some(format!("Unmatched horizontal group at x={x}, y={y}"));
            }
            if y < h - 2 && b[i] == b[i + w] && b[i] == b[i + w * 2] {
                return Some(format!("Unmatched vertical group at x={x}, y={y}"));
            }
        }
    }
    None
}
