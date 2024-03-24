use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use match3::char_board::{CharBoard, CharGem};
use match3::line::LineMatcherSettings;
use match3::rect_board::RectBoard;
use tinyrand::{RandRange, StdRand};

pub fn criterion_benchmark(c: &mut Criterion) {
    let settings = &LineMatcherSettings::common_match3();
    bench_board("common", c, 6, 6, 1, false, settings);
    bench_board("common", c, 6, 6, 2, false, settings);
    bench_board("common", c, 6, 6, 3, false, settings);
    bench_board("common", c, 6, 6, 5, false, settings);
    bench_board("common", c, 10, 10, 5, false, settings);
    bench_board("common", c, 25, 25, 5, false, settings);
    let settings = &LineMatcherSettings::common_match3().with_merge_neighbours(true);
    bench_board("match_neighbours", c, 6, 6, 1, false, settings);
    bench_board("match_neighbours", c, 6, 6, 2, false, settings);
    bench_board("match_neighbours", c, 6, 6, 3, false, settings);
    bench_board("match_neighbours", c, 6, 6, 5, false, settings);
    bench_board("match_neighbours", c, 10, 10, 5, false, settings);
    bench_board("match_neighbours", c, 25, 25, 5, false, settings);
}

fn bench_board(
    name: &str,
    c: &mut Criterion,
    width: usize,
    height: usize,
    alphabet: usize,
    wildcard: bool,
    settings: &LineMatcherSettings,
) {
    c.bench_function(
        &format!(
            "{name} - {width}x{height} {alphabet}-color{}",
            if wildcard { " with wildcards" } else { "" }
        ),
        |b| {
            let board = &make_board(width, height);
            let mut rand = StdRand::default();
            b.iter_batched(
                || populate_board(board, &mut rand, alphabet, wildcard),
                |board| black_box(board).find_matches_linear(black_box(settings)),
                BatchSize::SmallInput,
            );
        },
    );
}

fn make_board(width: usize, height: usize) -> RectBoard<CharGem, CharGem> {
    CharBoard::from_element(width, height, '-')
}

static ALPHABET: &[char] = &['*', 'r', 'g', 'b', 'y', 'p'];

fn populate_board(
    board: &CharBoard,
    rand: &mut StdRand,
    alphabet: usize,
    wildcard: bool,
) -> CharBoard {
    let mut board = board.clone();
    let range = if wildcard {
        0..(alphabet + 1)
    } else {
        1..(alphabet + 1)
    };
    for gem in board.board.iter_mut() {
        gem.0 = ALPHABET[rand.next_range(range.clone())];
    }
    board
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
