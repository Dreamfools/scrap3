//! Taken from on https://github.com/nsmryan/rectcut-rs/tree/058a45bbb9dc66ffa04bf60cb6d3a1a9b59cdf24
//! Courtesy of MIT license

/// The Rect struct represents an area, such as an area of the screen,
/// by its minimum x and y and maximum x and y.
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
}

impl Rect {
    /// Create a new Rect.
    pub fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Rect {
        Rect {
            minx,
            miny,
            maxx,
            maxy,
        }
    }

    /// Cut out the left of the rect, returning the left piece and modifying the original Rect.
    pub fn cut_left(&mut self, a: f32) -> Rect {
        let minx: f32 = self.minx;
        if self.maxx < self.minx + a {
            self.minx = self.maxx;
        } else {
            self.minx += a;
        }
        Rect::new(minx, self.miny, self.minx, self.maxy)
    }

    /// Cut out the right of the rect, returning the right piece and modifying the original Rect.
    pub fn cut_right(&mut self, a: f32) -> Rect {
        let maxx: f32 = self.maxx;
        if self.minx > self.maxx - a {
            self.maxx = self.minx;
        } else {
            self.maxx -= a;
        }
        Rect::new(self.maxx, self.miny, maxx, self.maxy)
    }

    /// Cut out the top of the rect, returning the top piece and modifying the original Rect.
    pub fn cut_top(&mut self, a: f32) -> Rect {
        let miny: f32 = self.miny;
        if self.maxy < self.miny + a {
            self.miny = self.maxy;
        } else {
            self.miny += a;
        }
        Rect::new(self.minx, miny, self.maxx, self.miny)
    }

    /// Cut out the bottom of the rect, returning the bottom piece and modifying the original Rect.
    pub fn cut_bottom(&mut self, a: f32) -> Rect {
        let maxy: f32 = self.maxy;
        if self.miny > self.maxy - a {
            self.maxy = self.miny;
        } else {
            self.maxy -= a;
        }
        Rect::new(self.minx, self.maxy, self.maxx, maxy)
    }

    /// Cut out the left of the rect, leaving the original unmodified.
    pub fn get_left(&self, a: f32) -> Rect {
        let maxx;
        if self.maxx < self.minx + a {
            maxx = self.maxx;
        } else {
            maxx = self.minx + a;
        }
        Rect::new(self.minx, self.miny, maxx, self.maxy)
    }

    /// Cut out the right of the rect, leaving the original unmodified.
    pub fn get_right(&self, a: f32) -> Rect {
        let minx;
        if self.minx > self.maxx - a {
            minx = self.minx;
        } else {
            minx = self.maxx - a;
        }
        Rect::new(minx, self.miny, self.maxx, self.maxy)
    }

    /// Cut out the top of the rect, leaving the original unmodified.
    pub fn get_top(&self, a: f32) -> Rect {
        let maxy;
        if self.maxy < self.miny + a {
            maxy = self.maxy;
        } else {
            maxy = self.miny + a;
        }
        Rect::new(self.minx, self.miny, self.maxx, maxy)
    }

    /// Cut out the bottom of the rect, leaving the original unmodified.
    pub fn get_bottom(&self, a: f32) -> Rect {
        let miny;
        if self.miny > self.maxy - a {
            miny = self.miny;
        } else {
            miny = self.maxy - a;
        }
        Rect::new(self.minx, miny, self.maxx, self.maxy)
    }

    /// Create a rect that extends the given rect out to the left,
    /// leaving the original unmodified.
    pub fn add_left(&self, a: f32) -> Rect {
        Rect::new(self.minx - a, self.miny, self.minx, self.maxy)
    }

    /// Create a rect that extends the given rect out to the right,
    /// leaving the original unmodified.
    pub fn add_right(&self, a: f32) -> Rect {
        Rect::new(self.maxx, self.miny, self.maxx + a, self.maxy)
    }

    /// Create a rect that extends the given rect out to the top,
    /// leaving the original unmodified.
    pub fn add_top(&self, a: f32) -> Rect {
        Rect::new(self.minx, self.miny - a, self.maxx, self.miny)
    }

    /// Create a rect that extends the given rect out to the bottom,
    /// leaving the original unmodified.
    pub fn add_bottom(&self, a: f32) -> Rect {
        Rect::new(self.minx, self.maxy, self.maxx, self.maxy + a)
    }

    /// Extend the rect out in all directions, leaving the original unmodified.
    pub fn extend(&self, a: f32) -> Rect {
        Rect::new(self.minx - a, self.miny - a, self.maxx + a, self.maxy + a)
    }

    /// Contract the rect in all directions, leaving the original unmodified.
    pub fn contract(&self, a: f32) -> Rect {
        Rect::new(self.minx + a, self.miny + a, self.maxx - a, self.maxy - a)
    }

    /// Splits current rect into a list of columns, leaving the original unmodified.
    pub fn cols(&self, n_cols: usize, gap: f32) -> Vec<Rect> {
        let mut cols = Vec::with_capacity(n_cols);
        let col_width = (self.maxx - self.minx - gap * (n_cols - 1) as f32) / n_cols as f32;
        for i in 0..n_cols {
            let x = self.minx + i as f32 * (col_width + gap);
            let col = Rect::new(x, self.miny, x + col_width, self.maxy);
            cols.push(col);
        }
        cols
    }

    /// Splits current rect into a list of rows, leaving the original unmodified.
    pub fn rows(&self, n_rows: usize, gap: f32) -> Vec<Rect> {
        let mut rows = Vec::with_capacity(n_rows);
        let row_height = (self.maxy - self.miny - gap * (n_rows - 1) as f32) / n_rows as f32;
        for i in 0..n_rows {
            let y = self.miny + i as f32 * (row_height + gap);
            let row = Rect::new(self.minx, y, self.maxx, y + row_height);
            rows.push(row);
        }
        rows
    }

    /// Splits current rect into a grid of columns and rows, leaving the original unmodified.
    pub fn grid(&self, n_cols: usize, n_rows: usize, gap_x: f32, gap_y: f32) -> Vec<Rect> {
        let mut grid = Vec::with_capacity(n_cols * n_rows);
        let col_width = (self.maxx - self.minx - gap_x * (n_cols - 1) as f32) / n_cols as f32;
        let row_height = (self.maxy - self.miny - gap_y * (n_rows - 1) as f32) / n_rows as f32;
        for j in 0..n_rows {
            for i in 0..n_cols {
                let x = self.minx + i as f32 * (col_width + gap_x);
                let y = self.miny + j as f32 * (row_height + gap_y);
                let rect = Rect::new(x, y, x + col_width, y + row_height);
                grid.push(rect);
            }
        }
        grid
    }

    /// Trims the rect to the given aspect ratio and centers it, leaving the original unmodified.
    pub fn trim_to_aspect_ratio(&self, aspect_ratio: f32) -> Rect {
        let width = self.width();
        let height = self.height();
        let current_aspect_ratio = width / height;
        if current_aspect_ratio > aspect_ratio {
            let new_width = height * aspect_ratio;
            let diff = width - new_width;
            Rect::new(
                self.minx + diff / 2.0,
                self.miny,
                self.maxx - diff / 2.0,
                self.maxy,
            )
        } else {
            let new_height = width / aspect_ratio;
            let diff = height - new_height;
            Rect::new(
                self.minx,
                self.miny + diff / 2.0,
                self.maxx,
                self.maxy - diff / 2.0,
            )
        }
    }

    /// Returns the width of the rect.
    pub fn width(&self) -> f32 {
        self.maxx - self.minx
    }

    /// Returns the height of the rect.
    pub fn height(&self) -> f32 {
        self.maxy - self.miny
    }

    pub fn center(&self) -> (f32, f32) {
        ((self.minx + self.maxx) / 2.0, (self.miny + self.maxy) / 2.0)
    }
}

#[test]
pub fn test_cut_left() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let left = rect.cut_left(1.0);

    assert_eq!(0.0, left.minx);
    assert_eq!(1.0, left.maxx);
    assert_eq!(rect.miny, left.miny);
    assert_eq!(rect.maxy, left.maxy);

    assert_eq!(1.0, rect.minx);
    assert_eq!(10.0, rect.maxx);
    assert_eq!(0.0, rect.miny);
    assert_eq!(10.0, rect.maxy);
}

#[test]
pub fn test_cut_right() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let right = rect.cut_right(1.0);

    assert_eq!(9.0, right.minx);
    assert_eq!(10.0, right.maxy);
    assert_eq!(rect.miny, right.miny);
    assert_eq!(rect.maxy, right.maxy);

    assert_eq!(0.0, rect.minx);
    assert_eq!(9.0, rect.maxx);
    assert_eq!(0.0, rect.miny);
    assert_eq!(10.0, rect.maxy);
}

#[test]
pub fn test_cut_top() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let top = rect.cut_top(1.0);

    assert_eq!(0.0, top.minx);
    assert_eq!(10.0, top.maxx);
    assert_eq!(0.0, top.miny);
    assert_eq!(1.0, top.maxy);

    assert_eq!(0.0, rect.minx);
    assert_eq!(10.0, rect.maxx);
    assert_eq!(1.0, rect.miny);
    assert_eq!(10.0, rect.maxy);
}

#[test]
pub fn test_cut_bottom() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let bottom = rect.cut_bottom(1.0);

    assert_eq!(0.0, bottom.minx);
    assert_eq!(10.0, bottom.maxx);
    assert_eq!(9.0, bottom.miny);
    assert_eq!(10.0, bottom.maxy);

    assert_eq!(0.0, rect.minx);
    assert_eq!(10.0, rect.maxx);
    assert_eq!(0.0, rect.miny);
    assert_eq!(9.0, rect.maxy);
}

#[test]
pub fn test_get_left() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let left = rect.get_left(1.0);

    assert_eq!(0.0, left.minx);
    assert_eq!(1.0, left.maxx);
    assert_eq!(0.0, left.miny);
    assert_eq!(10.0, left.maxy);
}

#[test]
pub fn test_get_right() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let right = rect.get_right(1.0);

    assert_eq!(9.0, right.minx);
    assert_eq!(10.0, right.maxx);
    assert_eq!(0.0, right.miny);
    assert_eq!(10.0, right.maxy);
}

#[test]
pub fn test_get_top() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let top = rect.get_top(1.0);

    assert_eq!(0.0, top.minx);
    assert_eq!(10.0, top.maxx);
    assert_eq!(0.0, top.miny);
    assert_eq!(1.0, top.maxy);
}

#[test]
pub fn test_get_bottom() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let bottom = rect.get_bottom(1.0);

    assert_eq!(0.0, bottom.minx);
    assert_eq!(10.0, bottom.maxx);
    assert_eq!(9.0, bottom.miny);
    assert_eq!(10.0, bottom.maxy);
}

#[test]
pub fn test_add_left() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let left = rect.add_left(1.0);

    assert_eq!(-1.0, left.minx);
    assert_eq!(0.0, left.maxx);
    assert_eq!(0.0, left.miny);
    assert_eq!(10.0, left.maxy);
}

#[test]
pub fn test_add_right() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let right = rect.add_right(1.0);

    assert_eq!(10.0, right.minx);
    assert_eq!(11.0, right.maxx);
    assert_eq!(0.0, right.miny);
    assert_eq!(10.0, right.maxy);
}

#[test]
pub fn test_add_top() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let top = rect.add_top(1.0);

    assert_eq!(0.0, top.minx);
    assert_eq!(10.0, top.maxx);
    assert_eq!(-1.0, top.miny);
    assert_eq!(0.0, top.maxy);
}

#[test]
pub fn test_add_bottom() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let bottom = rect.add_bottom(1.0);

    assert_eq!(0.0, bottom.minx);
    assert_eq!(10.0, bottom.maxx);
    assert_eq!(10.0, bottom.miny);
    assert_eq!(11.0, bottom.maxy);
}

#[test]
pub fn test_extend() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);

    let extended = rect.extend(1.0);

    assert_eq!(-1.0, extended.minx);
    assert_eq!(11.0, extended.maxx);
    assert_eq!(-1.0, extended.miny);
    assert_eq!(11.0, extended.maxy);
}

#[test]
pub fn test_contract() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);

    let contract = rect.contract(1.0);

    assert_eq!(1.0, contract.minx);
    assert_eq!(9.0, contract.maxx);
    assert_eq!(1.0, contract.miny);
    assert_eq!(9.0, contract.maxy);
}

#[test]
pub fn test_cols_no_gap() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let cols = rect.cols(2, 0.0);

    assert_eq!(2, cols.len());
    assert_eq!(0.0, cols[0].minx);
    assert_eq!(5.0, cols[0].maxx);
    assert_eq!(10.0, cols[0].height());
    assert_eq!(5.0, cols[1].minx);
    assert_eq!(10.0, cols[1].maxx);
    assert_eq!(10.0, cols[1].height());

    let rect = Rect::new(0.0, 0.0, 15.0, 10.0);
    let cols = rect.cols(3, 0.0);

    assert_eq!(3, cols.len());
    assert_eq!(0.0, cols[0].minx);
    assert_eq!(5.0, cols[0].maxx);
    assert_eq!(10.0, cols[0].height());
    assert_eq!(5.0, cols[1].minx);
    assert_eq!(10.0, cols[1].maxx);
    assert_eq!(10.0, cols[1].height());
    assert_eq!(10.0, cols[2].minx);
    assert_eq!(15.0, cols[2].maxx);
    assert_eq!(10.0, cols[2].height());
}

#[test]
pub fn test_cols_gap() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let cols = rect.cols(2, 1.0);

    assert_eq!(2, cols.len());
    assert_eq!(0.0, cols[0].minx);
    assert_eq!(4.5, cols[0].maxx);
    assert_eq!(10.0, cols[0].height());
    assert_eq!(5.5, cols[1].minx);
    assert_eq!(10.0, cols[1].maxx);
    assert_eq!(10.0, cols[1].height());

    let rect = Rect::new(0.0, 0.0, 14.0, 10.0);
    let cols = rect.cols(3, 1.0);

    assert_eq!(3, cols.len());
    assert_eq!(0.0, cols[0].minx);
    assert_eq!(4.0, cols[0].maxx);
    assert_eq!(10.0, cols[0].height());
    assert_eq!(5.0, cols[1].minx);
    assert_eq!(9.0, cols[1].maxx);
    assert_eq!(10.0, cols[1].height());
    assert_eq!(10.0, cols[2].minx);
    assert_eq!(14.0, cols[2].maxx);
    assert_eq!(10.0, cols[2].height());
}

#[test]
pub fn test_rows_no_gap() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let rows = rect.rows(2, 0.0);

    assert_eq!(2, rows.len());
    assert_eq!(0.0, rows[0].miny);
    assert_eq!(5.0, rows[0].maxy);
    assert_eq!(10.0, rows[0].width());
    assert_eq!(5.0, rows[1].miny);
    assert_eq!(10.0, rows[1].maxy);
    assert_eq!(10.0, rows[1].width());

    let rect = Rect::new(0.0, 0.0, 10.0, 15.0);
    let rows = rect.rows(3, 0.0);

    assert_eq!(3, rows.len());
    assert_eq!(0.0, rows[0].miny);
    assert_eq!(5.0, rows[0].maxy);
    assert_eq!(10.0, rows[0].width());
    assert_eq!(5.0, rows[1].miny);
    assert_eq!(10.0, rows[1].maxy);
    assert_eq!(10.0, rows[1].width());
    assert_eq!(10.0, rows[2].miny);
    assert_eq!(15.0, rows[2].maxy);
    assert_eq!(10.0, rows[2].width());
}

#[test]
pub fn test_rows_gap() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let rows = rect.rows(2, 1.0);

    assert_eq!(2, rows.len());
    assert_eq!(0.0, rows[0].miny);
    assert_eq!(4.5, rows[0].maxy);
    assert_eq!(10.0, rows[0].width());
    assert_eq!(5.5, rows[1].miny);
    assert_eq!(10.0, rows[1].maxy);
    assert_eq!(10.0, rows[1].width());

    let rect = Rect::new(0.0, 0.0, 10.0, 14.0);
    let rows = rect.rows(3, 1.0);

    assert_eq!(3, rows.len());
    assert_eq!(0.0, rows[0].miny);
    assert_eq!(4.0, rows[0].maxy);
    assert_eq!(10.0, rows[0].width());
    assert_eq!(5.0, rows[1].miny);
    assert_eq!(9.0, rows[1].maxy);
    assert_eq!(10.0, rows[1].width());
    assert_eq!(10.0, rows[2].miny);
    assert_eq!(14.0, rows[2].maxy);
    assert_eq!(10.0, rows[2].width());
}

#[test]
pub fn test_grid_no_gap() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let grid = rect.grid(2, 2, 0.0, 0.0);

    assert_eq!(4, grid.len());

    assert_eq!(0.0, grid[0].minx);
    assert_eq!(5.0, grid[0].maxx);
    assert_eq!(0.0, grid[0].miny);
    assert_eq!(5.0, grid[0].maxy);

    assert_eq!(5.0, grid[1].minx);
    assert_eq!(10.0, grid[1].maxx);
    assert_eq!(0.0, grid[1].miny);
    assert_eq!(5.0, grid[1].maxy);

    assert_eq!(0.0, grid[2].minx);
    assert_eq!(5.0, grid[2].maxx);
    assert_eq!(5.0, grid[2].miny);
    assert_eq!(10.0, grid[2].maxy);

    assert_eq!(5.0, grid[3].minx);
    assert_eq!(10.0, grid[3].maxx);
    assert_eq!(5.0, grid[3].miny);
    assert_eq!(10.0, grid[3].maxy);
}

#[test]
pub fn test_grid_gap() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let grid = rect.grid(2, 2, 1.0, 1.0);

    assert_eq!(4, grid.len());

    assert_eq!(0.0, grid[0].minx);
    assert_eq!(4.5, grid[0].maxx);
    assert_eq!(0.0, grid[0].miny);
    assert_eq!(4.5, grid[0].maxy);

    assert_eq!(5.5, grid[1].minx);
    assert_eq!(10.0, grid[1].maxx);
    assert_eq!(0.0, grid[1].miny);
    assert_eq!(4.5, grid[1].maxy);

    assert_eq!(0.0, grid[2].minx);
    assert_eq!(4.5, grid[2].maxx);
    assert_eq!(5.5, grid[2].miny);
    assert_eq!(10.0, grid[2].maxy);

    assert_eq!(5.5, grid[3].minx);
    assert_eq!(10.0, grid[3].maxx);
    assert_eq!(5.5, grid[3].miny);
    assert_eq!(10.0, grid[3].maxy);
}

#[test]
pub fn test_trim_to_aspect_ratio() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let trimmed = rect.trim_to_aspect_ratio(1.0);

    assert_eq!(0.0, trimmed.minx);
    assert_eq!(10.0, trimmed.maxx);
    assert_eq!(0.0, trimmed.miny);
    assert_eq!(10.0, trimmed.maxy);

    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let trimmed = rect.trim_to_aspect_ratio(2.0);

    assert_eq!(0.0, trimmed.minx);
    assert_eq!(10.0, trimmed.maxx);
    assert_eq!(2.5, trimmed.miny);
    assert_eq!(7.5, trimmed.maxy);

    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let trimmed = rect.trim_to_aspect_ratio(0.5);

    assert_eq!(2.5, trimmed.minx);
    assert_eq!(7.5, trimmed.maxx);
    assert_eq!(0.0, trimmed.miny);
    assert_eq!(10.0, trimmed.maxy);
}

/// A RectCutSide represents a side of the rectangle. This allows
/// the user to choose a side dynamically using a RectCut.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum RectCutSide {
    Left,
    Right,
    Top,
    Bottom,
}

/// A RectCut wraps a Rect along with a side. This allows the
/// user to fix the side and pass the rect-and-side together
/// into other code.
#[derive(Copy, Clone, Debug)]
struct RectCut {
    pub rect: Rect,
    pub side: RectCutSide,
}

impl RectCut {
    /// Create a new RectCut.
    pub fn new(rect: Rect, side: RectCutSide) -> RectCut {
        RectCut { rect, side }
    }

    /// Cut out from the RectCut, returning the new Rect
    /// and modifying the internal Rect.
    pub fn cut(&mut self, a: f32) -> Rect {
        match self.side {
            RectCutSide::Left => self.rect.cut_left(a),
            RectCutSide::Right => self.rect.cut_right(a),
            RectCutSide::Top => self.rect.cut_top(a),
            RectCutSide::Bottom => self.rect.cut_bottom(a),
        }
    }
}

#[test]
fn test_rectcut() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let mut rectcut = RectCut::new(rect, RectCutSide::Left);
    let left = rectcut.cut(1.0);
    assert_eq!(0.0, left.minx);
    assert_eq!(0.0, left.miny);
    assert_eq!(1.0, left.maxx);
    assert_eq!(10.0, left.maxy);

    assert_eq!(1.0, rectcut.rect.minx);
    assert_eq!(0.0, rectcut.rect.miny);
    assert_eq!(10.0, rectcut.rect.maxx);
    assert_eq!(10.0, rectcut.rect.maxy);
}
