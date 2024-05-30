use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use glamour::{point, size, Box2, Point2, Rect, Size2, Unit, Vector2};
use ndshape::{RuntimeShape, Shape};

pub struct MicroCell;
impl Unit for MicroCell {
    type Scalar = u64;

    fn name() -> Option<&'static str> {
        Some("MicroCell")
    }
}

pub struct Cell;
impl Unit for Cell {
    type Scalar = u64;

    fn name() -> Option<&'static str> {
        Some("Cell")
    }
}

pub struct Screen;
impl Unit for Screen {
    type Scalar = f32;

    fn name() -> Option<&'static str> {
        Some("Screen Unit")
    }
}

pub type GridMicroPos = Point2<MicroCell>;
pub type GridPos = Point2<Cell>;
pub type GridSize = Size2<Cell>;

pub type ScreenPos = Point2<Screen>;
pub type ScreenRect = Rect<Screen>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CellIndex(pub usize);

impl Hash for CellIndex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[derive(Clone)]
pub struct GridMath<ViewUnits: Unit<Scalar = f32>, const MICROCELLS: u64 = 1000> {
    rect: Rect<ViewUnits>,
    shape: RuntimeShape<usize, 2>,
    cols: usize,
    rows: usize,
    cell_width: f32,
    cell_height: f32,
    swap_y: bool,
}

impl<ViewUnits: Unit<Scalar = f32>, const MICROCELLS: u64> GridMath<ViewUnits, MICROCELLS> {
    pub fn new(rect: Rect<ViewUnits>, cols: usize, rows: usize, swap_y: bool) -> Self {
        let cell_width = rect.width() / cols as f32;
        let cell_height = rect.height() / rows as f32;
        Self {
            rect,
            shape: RuntimeShape::<usize, 2>::new([cols, rows]),
            cols,
            rows,
            cell_width,
            cell_height,
            swap_y,
        }
    }

    fn linearize(&self, x: usize, y: usize) -> usize {
        if self.swap_y {
            self.shape.linearize([x, self.rows - y - 1])
        } else {
            self.shape.linearize([x, y])
        }
    }

    fn delinearize(&self, pos: usize) -> (usize, usize) {
        let [x, mut y] = self.shape.delinearize(pos);
        if self.swap_y {
            y = self.rows - y - 1;
        }

        (x, y)
    }

    pub fn micropos_to_pos(&self, pos: GridMicroPos) -> GridPos {
        GridPos {
            x: pos.x / MICROCELLS,
            y: pos.y / MICROCELLS,
        }
    }

    pub fn micropos_to_index(&self, pos: GridMicroPos) -> CellIndex {
        let pos = self.micropos_to_pos(pos);
        CellIndex(self.linearize(pos.x as usize, pos.y as usize))
    }

    /// Returns the closest micro position to the given view position
    pub fn view_to_micropos(&self, pos: Vector2<ViewUnits>) -> GridMicroPos {
        let x = ((pos.x - self.rect.origin.x) / self.cell_width * MICROCELLS as f32).floor() as u64;
        let y =
            ((pos.y - self.rect.origin.y) / self.cell_height * MICROCELLS as f32).floor() as u64;
        point!(x, y)
    }

    /// Returns a rect for a cell in a grid
    pub fn rect_at_index(&self, index: CellIndex) -> Rect<ViewUnits> {
        let (x, y) = self.delinearize(index.0);
        let minx = self.rect.origin.x + x as f32 * self.cell_width;
        let miny = self.rect.origin.y + y as f32 * self.cell_height;
        let maxx = minx + self.cell_width;
        let maxy = miny + self.cell_height;

        Box2::new((minx, miny), (maxx, maxy)).into()
    }

    /// Returns the center of a cell in a grid
    pub fn center_at_index(&self, index: CellIndex) -> Point2<ViewUnits> {
        let (x, y) = self.delinearize(index.0);
        let pos_x = self.rect.origin.x + (x as f32 + 0.5) * self.cell_width;
        let pos_y = self.rect.origin.y + (y as f32 + 0.5) * self.cell_height;
        point!(pos_x, pos_y)
    }

    /// Returns a single cell-sized rect with minx and miny at zero
    pub fn unit_cell(&self) -> Rect<ViewUnits> {
        Rect::from_size(size!(self.cell_width, self.cell_height))
    }

    /// Returns the squared euclidean distance between two cells
    pub fn distance2(&self, a: CellIndex, b: CellIndex) -> f32 {
        let (ax, ay) = self.delinearize(a.0);
        let (bx, by) = self.delinearize(b.0);
        let dx = ax as f32 - bx as f32;
        let dy = ay as f32 - by as f32;
        dx * dx + dy * dy
    }

    /// Returns the euclidean distance between two cells
    pub fn distance(&self, a: CellIndex, b: CellIndex) -> f32 {
        self.distance2(a, b).sqrt()
    }

    /// Returns the grid (manhattan) distance between two cells
    pub fn grid_distance(&self, a: CellIndex, b: CellIndex) -> usize {
        let (ax, ay) = self.delinearize(a.0);
        let (bx, by) = self.delinearize(b.0);
        let dx = (ax as isize - bx as isize).abs();
        let dy = (ay as isize - by as isize).abs();
        dx.max(dy) as usize
    }

    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    pub fn cell_height(&self) -> f32 {
        self.cell_height
    }

    pub fn shape(&self) -> &RuntimeShape<usize, 2> {
        &self.shape
    }
}

impl<ViewUnits: Unit<Scalar = f32>, const MICROCELLS: u64> Debug
    for GridMath<ViewUnits, MICROCELLS>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GridMath")
            .field("MICROCELLS", &MICROCELLS)
            .field("rect", &self.rect)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .field("cell_width", &self.cell_width)
            .field("cell_height", &self.cell_height)
            .finish()
    }
}
