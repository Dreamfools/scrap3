use crate::line::LineMatcherSettings;
use crate::{BoardGem, BoardMatch};
use ndshape::{RuntimeShape, Shape};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct RectBoard<T: BoardGem> {
    pub shape: RuntimeShape<usize, 2>,
    pub board: Vec<T>,
    pub lines: Arc<Vec<Vec<usize>>>,
    pub neighbours: Arc<Vec<Vec<usize>>>,
}

impl<T: Debug + BoardGem> Debug for RectBoard<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CharBoard")
            .field("width", &self.shape.as_array()[0])
            .field("height", &self.shape.as_array()[1])
            .field("board", &self.board)
            .finish()
    }
}

impl<T: Copy + BoardGem> RectBoard<T> {
    pub fn from_element(width: usize, height: usize, filler: impl Into<T>) -> Self {
        let board = vec![filler.into(); width * height];
        Self::new(width, height, board)
    }
}

impl<T: BoardGem> RectBoard<T> {
    pub fn from_fn(width: usize, height: usize, filler: impl Fn(usize) -> T) -> Self {
        let board = (0..(width * height)).map(filler).collect();
        Self::new(width, height, board)
    }

    pub fn new(width: usize, height: usize, board: Vec<T>) -> Self {
        let shape = RuntimeShape::<usize, 2>::new([width, height]);
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
        }
    }

    pub fn find_matches_linear(&self, settings: &LineMatcherSettings) -> Vec<BoardMatch<T::Color>> {
        settings.find_matches(&self.board, &self.lines, &self.neighbours)
    }

    /// Returns the shortest path to move the gem between two positions
    pub fn move_gem(
        &mut self,
        from: usize,
        to: usize,
        strategy: GridMoveStrategy,
    ) -> impl Iterator<Item = usize> {
        let [x_from, y_from] = self.shape.delinearize(from);
        let [x_to, y_to] = self.shape.delinearize(to);
        let dx = ((x_to as isize) - (x_from as isize)).clamp(-1, 1);
        let dy =
            ((y_to as isize) - (y_from as isize)).clamp(-1, 1) * self.shape.as_array()[0] as isize;
        let diagonal = match strategy {
            GridMoveStrategy::VerticalFirst => dy,
            GridMoveStrategy::HorizontalFirst => dx,
            GridMoveStrategy::Diagonals => dy + dx,
        };

        GridMoveIter {
            shape: self.shape.clone(),
            from: from as isize,
            to: to as isize,
            dx,
            dy,
            diagonal,
        }
    }

    pub fn width(&self) -> usize {
        self.shape.as_array()[0]
    }

    pub fn height(&self) -> usize {
        self.shape.as_array()[1]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GridMoveStrategy {
    VerticalFirst,
    HorizontalFirst,
    Diagonals,
}

#[derive(Clone)]
struct GridMoveIter {
    shape: RuntimeShape<usize, 2>,
    from: isize,
    to: isize,
    dx: isize,
    dy: isize,
    diagonal: isize,
}

impl Iterator for GridMoveIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from == self.to {
            return None;
        }

        let [x_from, y_from] = self.shape.delinearize(self.from as usize);
        let [x_to, y_to] = self.shape.delinearize(self.to as usize);
        match (x_from == x_to, y_from == y_to) {
            (false, false) => {
                self.from += self.diagonal;
            }
            (true, false) => {
                self.from += self.dy;
            }
            (false, true) => {
                self.from += self.dx;
            }
            (true, true) => {
                return None;
            }
        }

        Some(self.from as usize)
    }
}
