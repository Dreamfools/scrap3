use crate::char_board::{board_from_str, display_board, visualize_match, CharBoard, CharGem};
use crate::line::LineMatcherSettings;
use crate::MatchColor;
use insta::assert_snapshot;
use itertools::Itertools;
use ndshape::Shape;
use proptest::prelude::{Just, Strategy};
use proptest::test_runner::{TestError, TestRunner};
use rstest::rstest;
use serde::Deserialize;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

type S = LineMatcherSettings;

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
            let matches = board.find_matches_linear(&settings);
            (
                settings_name,
                visualize_match(name.to_string(), board, matches, false),
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

#[rstest]
fn common_line3_file_tests(#[files("src/cases/common/*.txt")] path: PathBuf) {
    check_path("common", path);
}

#[rstest]
fn wildcard_line3_file_tests(#[files("src/cases/wildcard/*.txt")] path: PathBuf) {
    check_path("wildcard", path);
}

#[rstest]
fn sizing_line3_file_tests(#[files("src/cases/sizing/*.txt")] path: PathBuf) {
    check_path("sizing", path);
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
    let b = board_from_str("rr*g");
    let settings = S::common_match3();
    // settings.merge_neighbours = false;
    // settings.line_size = 2;
    let matches = b.find_matches_linear(&settings);
    visualize_match("DEV".to_string(), b, matches, false);
}

#[test]
fn random_tests() {
    let mut runner = TestRunner::default();
    let run_result = runner.run(&prop_board(64), |board| {
        let mut settings = S::common_match3();
        settings.line_size = 3;
        let matches = board.find_matches_linear(&settings);
        let mut cloned = board.clone();
        for m in &matches {
            for &x in m.cells() {
                cloned.board[x].0 = ' '
            }
        }
        if let Some(err) = check_straight_matches(&cloned) {
            let visualized = visualize_match(err, board, matches, false);
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
            panic!("{}:\n{}", reason, display_board(&board, false))
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
