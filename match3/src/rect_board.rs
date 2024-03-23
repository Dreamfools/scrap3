use crate::line::{LineMatcherSettings, LinesBoardMatch};
use crate::MatchColor;
use ndshape::{RuntimeShape, Shape};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Clone)]
pub struct RectBoard<T: AsRef<Color>, Color: MatchColor> {
    pub shape: RuntimeShape<usize, 2>,
    pub board: Vec<T>,
    pub lines: Arc<Vec<Vec<usize>>>,
    pub neighbours: Arc<Vec<Vec<usize>>>,
    _color: PhantomData<Color>,
}

impl<T: Debug + AsRef<Color>, Color: MatchColor> Debug for RectBoard<T, Color> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CharBoard")
            .field("width", &self.shape.as_array()[0])
            .field("height", &self.shape.as_array()[1])
            .field("board", &self.board)
            .finish()
    }
}

impl<T: Copy + AsRef<Color>, Color: MatchColor> RectBoard<T, Color> {
    pub fn from_element(width: usize, height: usize, filler: impl Into<T>) -> Self {
        let shape = RuntimeShape::<usize, 2>::new([width, height]);
        let board = vec![filler.into(); shape.size()];
        Self::new(shape, board)
    }
}

impl<T: AsRef<Color>, Color: MatchColor> RectBoard<T, Color> {
    pub fn from_fn(width: usize, height: usize, filler: impl Fn(usize) -> T) -> Self {
        let shape = RuntimeShape::<usize, 2>::new([width, height]);
        let board = (0..shape.size()).map(|i| filler(i)).collect();
        Self::new(shape, board)
    }

    pub fn new(shape: RuntimeShape<usize, 2>, board: Vec<T>) -> Self {
        let [width, height] = shape.as_array();
        let mut lines: Vec<Vec<usize>> = vec![];
        let mut neighbours: Vec<Vec<usize>> = vec![];
        for y in 0..height {
            lines.push((0..width).map(|x| shape.linearize([x, y])).collect());
        }
        for x in 0..width {
            lines.push((0..height).map(|y| shape.linearize([x, y])).collect());
        }

        for i in 0..shape.size() {
            let [x, y] = shape.delinearize(i);
            let mut cell_neighbours = vec![];
            if x > 0 {
                cell_neighbours.push(shape.linearize([x - 1, y]));
            }
            if x < width - 1 {
                cell_neighbours.push(shape.linearize([x + 1, y]));
            }
            if y > 0 {
                cell_neighbours.push(shape.linearize([x, y - 1]));
            }
            if y < height - 1 {
                cell_neighbours.push(shape.linearize([x, y + 1]));
            }
            neighbours.push(cell_neighbours);
        }

        Self {
            shape,
            board,
            lines: Arc::new(lines),
            neighbours: Arc::new(neighbours),
            _color: Default::default(),
        }
    }

    pub fn find_matches(&self, settings: LineMatcherSettings) -> Vec<LinesBoardMatch<Color>> {
        settings.find_matches(&self.board, &self.lines, &self.neighbours)
    }

    pub fn width(&self) -> usize {
        self.shape.as_array()[0]
    }

    pub fn height(&self) -> usize {
        self.shape.as_array()[1]
    }
}
