use nohash_hasher::{IntMap, IntSet, IsEnabled};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[cfg(test)]
mod tests;

mod line;

mod rect_board;

/// Match colors are assumed to be cheap to clone and instantiate my matching
/// algorithms
pub trait MatchColor: Clone + Default {
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
        return false;
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
