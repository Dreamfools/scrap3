use nohash_hasher::{IntMap, IntSet, IsEnabled};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

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
}

#[derive(Debug)]
pub struct LinesBoardMatch<Color: MatchColor> {
    pub color: Color,
    pub cells: Vec<usize>,
}

impl<Color: MatchColor> LinesBoardMatch<Color> {
    pub fn new(color: Color, cells: Vec<usize>) -> Self {
        Self { color, cells }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct MatchIndex(usize);

impl Hash for MatchIndex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl IsEnabled for MatchIndex {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BoardIndex(usize);
#[derive(Debug, Clone)]
pub struct LineMatcherSettings {
    /// Minimum line size
    pub line_size: usize,
    /// Minimum size of a match group
    pub min_group_size: Option<usize>,
    /// Whenever neighbouring valid groups of matching types should be merged
    /// regardless of the [line_size]
    pub merge_neighbours: bool,
}

impl LineMatcherSettings {
    pub fn find_matches<
        'a,
        Color: MatchColor,
        Gem: AsRef<Color>,
        Line: AsRef<[usize]>,
        Neighbours: AsRef<[usize]>,
    >(
        &self,
        cells: &'a [Gem],
        lines: &'a [Line],
        neighbours: &'a [Neighbours],
    ) -> Vec<LinesBoardMatch<Color>> {
        let state = LineMatcherState::new(self.clone(), cells, lines, neighbours);
        state.find_matches()
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

#[derive(Debug)]
struct LineMatcherState<
    'a,
    Color: MatchColor,
    Gem: AsRef<Color>,
    Line: AsRef<[usize]>,
    Neighbours: AsRef<[usize]>,
> {
    settings: LineMatcherSettings,

    cells: &'a [Gem],
    lines: &'a [Line],
    neighbours: &'a [Neighbours],

    matches: Vec<Option<LinesBoardMatch<Color>>>,
    match_board: IntMap<usize, SmallVec<[MatchIndex; 1]>>,

    match_cells_cache: Option<Vec<usize>>,
}

impl<
        'a,
        Color: MatchColor,
        Gem: AsRef<Color>,
        Line: AsRef<[usize]>,
        Neighbours: AsRef<[usize]>,
    > LineMatcherState<'a, Color, Gem, Line, Neighbours>
{
    fn new(
        settings: LineMatcherSettings,
        cells: &'a [Gem],
        lines: &'a [Line],
        neighbours: &'a [Neighbours],
    ) -> Self {
        Self {
            settings,
            cells,
            lines,
            neighbours,
            matches: Default::default(),
            match_board: Default::default(),
            match_cells_cache: None,
        }
    }

    fn find_matches(mut self) -> Vec<LinesBoardMatch<Color>> {
        for line in self.lines {
            self.match_line(line.as_ref());
        }
        self.matches.into_iter().flatten().collect()
    }

    fn close_match(&mut self, mut group: LinesBoardMatch<Color>) {
        if group.cells.len() < self.settings.line_size {
            group.cells.clear();
            self.match_cells_cache = Some(group.cells);
            return;
        }
        /// Check for intersection with other groups and merge them
        fn check_merge_groups_at_cell<Color: MatchColor>(
            matches: &mut [Option<LinesBoardMatch<Color>>],
            match_board: IntMap<usize, SmallVec<[MatchIndex; 1]>>,
            group: &mut LinesBoardMatch<Color>,
            cell: usize,
            merge_group: &mut Option<MatchIndex>,
            groups_to_merge: &mut BTreeSet<MatchIndex>,
        ) {
            if let Some(other_groups) = match_board.get(&cell) {
                for &intersecting in other_groups {
                    // Resolve the other group ID
                    let other_group = &mut matches[intersecting.0]
                        .as_mut()
                        .expect("All dead groups should be inaccessible from the board");
                    if !other_group.color.matches(&group.color) {
                        return;
                    }
                    if merge_group.is_none() {
                        // We intersect with the first matching group, so merge the current group into that one
                        other_group.cells.extend(&group.cells);
                        *merge_group = Some(intersecting);
                    } else {
                        // We already found a group to merge into, so add this matching group to merge in at a later stage
                        groups_to_merge.insert(intersecting);
                    }
                }
            }
        }
        let mut merge_group: Option<MatchIndex> = None;
        let mut groups_to_merge = BTreeSet::<MatchIndex>::new();

        // Check for intersection with other groups
        for i in 0..group.cells.len() {
            let cell = group.cells[i];
            check_merge_groups_at_cell(
                &mut self.matches,
                self.match_board.clone(),
                &mut group,
                cell,
                &mut merge_group,
                &mut groups_to_merge,
            );
        }

        if merge_group.is_none() && group.cells.len() < self.settings.min_group_size.unwrap_or(0) {
            group.cells.clear();
            self.match_cells_cache = Some(group.cells);
            return;
        }

        // Check for merging with neighbours
        if self.settings.merge_neighbours {
            for i in 0..group.cells.len() {
                let cell = group.cells[i];
                for &neighbour in self.neighbours[cell].as_ref() {
                    check_merge_groups_at_cell(
                        &mut self.matches,
                        self.match_board.clone(),
                        &mut group,
                        neighbour,
                        &mut merge_group,
                        &mut groups_to_merge,
                    );
                }
            }
        }

        if let Some(merged) = merge_group {
            group.cells.clear();
            self.match_cells_cache = Some(group.cells);
            for &x in &self.matches[merged.0]
                .as_ref()
                .expect("Merge group was checked for already")
                .cells
            {
                self.match_board.entry(x).or_default().push(merged)
            }

            // Merge group exists, which mean the current group is already
            // merged, now just clean up the other groups
            for &other_group_idx in &groups_to_merge {
                let mut other = std::mem::take(&mut self.matches[other_group_idx.0])
                    .expect("Merge group was checked for already");
                let main = self.matches[merged.0]
                    .as_mut()
                    .expect("Merge group was checked for already");

                // Remap the cells of the other group to the main group
                for cell in &other.cells {
                    let groups = self
                        .match_board
                        .get_mut(cell)
                        .expect("Cell should already be initialized by the previous group");
                    if groups[groups.len() - 1] == merged {
                        groups.pop();
                    }
                    for x in groups {
                        if x == &other_group_idx {
                            *x = merged;
                        }
                    }
                }
                main.cells.append(&mut other.cells);
            }

            #[cfg(debug_assertions)]
            for (i, cell) in self.match_board.iter() {
                for dead_group in &groups_to_merge {
                    for existing in cell {
                        debug_assert!(
                            existing != dead_group,
                            "Should not have consumed group `{}` on the board (at position `{}`)",
                            dead_group.0,
                            i,
                        );
                    }
                }
            }
        } else {
            debug_assert!(
                groups_to_merge.is_empty(),
                "should not have extra merge groups when the main merge group is None"
            );

            let index = MatchIndex(self.matches.len());

            for &cell in &group.cells {
                self.match_board.entry(cell).or_default().push(index);
            }

            self.matches.push(Some(group));
        }
    }

    fn match_line(&mut self, line: &[usize]) {
        let mut current_match: Option<LinesBoardMatch<Color>> = None;
        for i in 0..line.len() {
            let pos = line[i];
            let gem = self.cells[pos].as_ref();
            if let Some(ref mut match_group) = current_match {
                if !match_group.color.matches(gem) {
                    let group = std::mem::take(&mut current_match)
                        .expect("Should have a match group to close");
                    self.close_match(group);
                    continue;
                }
                match_group.cells.push(pos);
            } else {
                if !gem.can_start_match() {
                    continue;
                }

                let mut group = LinesBoardMatch::<Color>::new(
                    gem.clone(),
                    std::mem::take(&mut self.match_cells_cache).unwrap_or_default(),
                );
                todo!("Only backtrack when the previous cell was a wildcard");
                for i in (0..=i).rev() {
                    let back_pos = line[i];
                    let back_gem = self.cells[back_pos].as_ref();
                    if !group.color.matches(back_gem) {
                        break;
                    }
                    group.cells.insert(0, i);
                }

                current_match = Some(group);
            }
        }

        if let Some(group) = current_match {
            self.close_match(group);
        }
    }
}
