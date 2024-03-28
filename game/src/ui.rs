use crate::CutRect;
use comfy::{vec2, Itertools, Vec2};
use ndshape::{RuntimeShape, Shape};
use rect::Rect;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct Ui {
    pub rect: CutRect,
    pub z: i32,
}

impl Ui {
    pub fn new(rect: CutRect, z: i32) -> Self {
        Self { rect, z }
    }

    pub fn with_rect(mut self, rect: CutRect) -> Self {
        self.rect = rect;
        self
    }

    pub fn map_rect(mut self, f: impl FnOnce(CutRect) -> CutRect) -> Self {
        self.rect = f(self.rect);
        self
    }

    /// Returns a new Ui with the same rect but the next z layer
    pub fn next_layer(&self) -> Self {
        Self::new(self.rect, self.z + 1)
    }

    /// Splits the Ui into a grid
    pub fn grid(self, cols: usize, rows: usize) -> Vec<Self> {
        self.rect
            .grid(cols, rows, 0.0, 0.0)
            .into_iter()
            .map(|cell| self.clone().with_rect(cell))
            .collect_vec()
    }

    /// Splits the Ui into rows
    pub fn rows(self, rows: usize) -> Vec<Self> {
        self.rect
            .rows(rows, 0.0)
            .into_iter()
            .map(|cell| self.clone().with_rect(cell))
            .collect_vec()
    }

    /// Splits the Ui into columns
    pub fn cols(self, cols: usize) -> Vec<Self> {
        self.rect
            .cols(cols, 0.0)
            .into_iter()
            .map(|cell| self.clone().with_rect(cell))
            .collect_vec()
    }

    /// Shrinks the available space by a percentage on each side.
    ///
    /// Percentage is a value between 0.0 and 1.0.
    pub fn shrink(self, percentage: f32) -> Self {
        let ox = self.rect.width() * percentage / 2.0;
        let oy = self.rect.height() * percentage / 2.0;
        self.map_rect(|r| CutRect::new(r.minx + ox, r.miny + oy, r.maxx - ox, r.maxy - oy))
    }

    /// See [CutRect::trim_to_aspect_ratio]
    pub fn trim_to_aspect_ratio(self, aspect_ratio: f32) -> Self {
        self.map_rect(|r| r.trim_to_aspect_ratio(aspect_ratio))
    }

    /// See [CutRect::cut_left]
    pub fn cut_left(&mut self, a: f32) -> Self {
        let r = self.rect.cut_left(a);
        Self::new(r, self.z)
    }

    /// See [CutRect::cut_right]
    pub fn cut_right(&mut self, a: f32) -> Self {
        let r = self.rect.cut_right(a);
        Self::new(r, self.z)
    }

    /// See [CutRect::cut_top]
    pub fn cut_top(&mut self, a: f32) -> Self {
        let r = self.rect.cut_top(a);
        Self::new(r, self.z)
    }

    /// See [CutRect::cut_bottom]
    pub fn cut_bottom(&mut self, a: f32) -> Self {
        let r = self.rect.cut_bottom(a);
        Self::new(r, self.z)
    }
}

impl Ui {
    /// Recenter the Ui around a position
    pub fn recenter(self, pos: Vec2) -> Ui {
        self.map_rect(|rect| rect.recenter(pos))
    }

    /// Returns true if the position is inside the Ui's rect
    pub fn contains(&self, pos: Vec2) -> bool {
        self.rect.contains(pos)
    }

    /// Clamps a position to the Ui's rect
    pub fn clamp_pos(&self, mut pos: Vec2) -> Vec2 {
        pos.x = pos.x.clamp(self.rect.minx, self.rect.maxx);
        pos.y = pos.y.clamp(self.rect.miny, self.rect.maxy);
        pos
    }
}

#[derive(Clone)]
pub struct GridMath {
    rect: Rect,
    shape: RuntimeShape<usize, 2>,
    cols: usize,
    rows: usize,
    cell_width: f32,
    cell_height: f32,
}

impl GridMath {
    pub fn new(rect: Rect, cols: usize, rows: usize) -> Self {
        let cell_width = rect.width() / cols as f32;
        let cell_height = rect.height() / rows as f32;
        Self {
            rect,
            shape: RuntimeShape::<usize, 2>::new([cols, rows]),
            cols,
            rows,
            cell_width,
            cell_height,
        }
    }

    /// Returns the index of a cell that contains a position
    pub fn pos_to_index(&self, pos: Vec2) -> usize {
        let x = ((pos.x - self.rect.minx) / self.cell_width).floor() as usize;
        let y = ((pos.y - self.rect.miny) / self.cell_height).floor() as usize;
        self.shape.linearize([x, y])
    }

    /// Returns a rect for a cell in a grid
    pub fn rect_at_index(&self, index: usize) -> Rect {
        let [x, y] = self.shape.delinearize(index);
        let minx = self.rect.minx + x as f32 * self.cell_width;
        let miny = self.rect.miny + y as f32 * self.cell_height;
        let maxx = minx + self.cell_width;
        let maxy = miny + self.cell_height;
        CutRect::new(minx, miny, maxx, maxy)
    }

    /// Returns the center of a cell in a grid
    pub fn center_at_index(&self, index: usize) -> Vec2 {
        let [x, y] = self.shape.delinearize(index);
        let pos_x = self.rect.minx + (x as f32 + 0.5) * self.cell_width;
        let pos_y = self.rect.miny + (y as f32 + 0.5) * self.cell_height;
        vec2(pos_x, pos_y)
    }

    /// Returns a single cell-sized rect with minx and miny at zero
    pub fn unit_cell(&self) -> Rect {
        CutRect::new(0.0, 0.0, self.cell_width, self.cell_height)
    }

    /// Returns the squared distance between two cells
    pub fn distance2(&self, a: usize, b: usize) -> f32 {
        let [ax, ay] = self.shape.delinearize(a);
        let [bx, by] = self.shape.delinearize(b);
        let dx = ax as f32 - bx as f32;
        let dy = ay as f32 - by as f32;
        dx * dx + dy * dy
    }

    /// Returns the distance between two cells
    pub fn distance(&self, a: usize, b: usize) -> f32 {
        self.distance2(a, b).sqrt()
    }
}

impl Debug for GridMath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridMath")
            .field("rect", &self.rect)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .field("cell_width", &self.cell_width)
            .field("cell_height", &self.cell_height)
            .finish()
    }
}
