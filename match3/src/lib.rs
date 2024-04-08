use lockfree_object_pool::{LinearObjectPool, LinearReusable};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::OnceLock;

pub use ndshape::Shape;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "char-board")]
pub mod char_board;

pub mod line;

pub mod rect_board;

/// Match colors are assumed to be cheap to clone and instantiate my matching
/// algorithms
pub trait MatchColor: Debug + Clone + Default {
    /// Checks whenever two colors are matching
    ///
    /// Implementations of this method must ensure commutative property
    ///
    /// Aka, `a.matches(b) == b.matches(a)`
    fn matches(&self, other: &Self) -> bool;

    /// Checks whenever match can start with the gem of this color
    ///
    /// Should ideally be set to false for "wildcard" colors
    fn can_start_match(&self) -> bool;

    /// Hints to the matcher that the color should not be checked for matching
    ///
    /// This method is mainly for performance optimization and matchers may
    /// ignore it, so the actual matching logic should be handled by the
    /// [matches] and [can_start_match] methods
    fn hint_is_unmatchable(&self) -> bool {
        false
    }
}

pub trait BoardGem {
    type Color: MatchColor;

    fn color(&self) -> Self::Color;
}
pub type BoardMatchPool = LinearObjectPool<Vec<usize>>;
pub type BoardMatchCells = LinearReusable<'static, Vec<usize>>;

static BOARD_MATCH_POOL: OnceLock<BoardMatchPool> = OnceLock::new();

#[inline]
pub fn get_board_match_pool() -> &'static BoardMatchPool {
    BOARD_MATCH_POOL.get_or_init(|| BoardMatchPool::new(Default::default, |v| v.clear()))
}

pub struct BoardMatch<Color: MatchColor> {
    color: Color,
    cells: BoardMatchCells,
}

impl<Color: MatchColor> Debug for BoardMatch<Color> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoardMatch")
            .field("color", &self.color)
            .field("cells", self.cells.deref())
            .finish()
    }
}

impl<Color: MatchColor> BoardMatch<Color> {
    #[inline]
    pub fn new(color: Color) -> Self {
        Self::from_cells(color, get_board_match_pool().pull())
    }
    #[inline]
    pub fn from_cells(color: Color, cells: BoardMatchCells) -> Self {
        Self { color, cells }
    }
    #[inline]
    pub fn color(&self) -> &Color {
        &self.color
    }
    #[inline]
    pub fn cells(&self) -> &Vec<usize> {
        &self.cells
    }
    #[inline]
    pub fn cells_mut(&mut self) -> &mut Vec<usize> {
        &mut self.cells
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct SimpleGem<C: MatchColor>(pub C);

impl<C: MatchColor> BoardGem for SimpleGem<C> {
    type Color = C;

    fn color(&self) -> Self::Color {
        self.0.clone()
    }
}

/// Get two mutable references to two elements in a slice
///
/// # Panics
/// Panics if `i1 == i2` or if either index is out of bounds
pub fn get_two_mut<T>(slice: &mut [T], i1: usize, i2: usize) -> (&mut T, &mut T) {
    match i1.cmp(&i2) {
        Ordering::Less => {
            let (l, r) = slice.split_at_mut(i2);
            (&mut l[i1], &mut r[0])
        }
        Ordering::Greater => {
            let (l, r) = slice.split_at_mut(i1);
            (&mut r[0], &mut l[i2])
        }
        Ordering::Equal => {
            panic!("Cannot get two elements at the same index")
        }
    }
}
