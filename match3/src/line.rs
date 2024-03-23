use crate::{BoardMatch, MatchColor};
use nohash_hasher::{IntMap, IsEnabled};
use smallvec::SmallVec;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

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
    pub fn new(line_size: usize, min_group_size: Option<usize>, merge_neighbours: bool) -> Self {
        Self {
            line_size,
            min_group_size,
            merge_neighbours,
        }
    }

    /// Common match-3 settings, with 3-in-a-row matches and no neighboring
    /// groups merging
    pub fn common_match3() -> Self {
        Self::new(3, None, false)
    }

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
    ) -> Vec<BoardMatch<Color>> {
        let state = LineMatcherState::new(self.clone(), cells, lines, neighbours);
        state.find_matches()
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

    matches: Vec<Option<BoardMatch<Color>>>,
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

    fn find_matches(mut self) -> Vec<BoardMatch<Color>> {
        for line in self.lines {
            self.match_line(line.as_ref());
        }
        self.matches.into_iter().flatten().collect()
    }

    fn close_match(&mut self, mut group: BoardMatch<Color>) {
        if group.cells.len() < self.settings.line_size {
            group.cells.clear();
            self.match_cells_cache = Some(group.cells);
            return;
        }

        /// Check for intersection with other groups and merge them
        fn check_merge_groups_at_cell<Color: MatchColor>(
            matches: &mut [Option<BoardMatch<Color>>],
            match_board: &IntMap<usize, SmallVec<[MatchIndex; 1]>>,
            group: &mut BoardMatch<Color>,
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
                        continue;
                    }
                    if let Some(group) = merge_group {
                        if group == &intersecting {
                            continue;
                        }
                        // We already found a group to merge into, so add this matching group to merge in at a later stage
                        groups_to_merge.insert(intersecting);
                    } else {
                        // We intersect with the first matching group, so merge the current group into that one
                        other_group.cells.extend(&group.cells);
                        *merge_group = Some(intersecting);
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
                &self.match_board,
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
                        &self.match_board,
                        &mut group,
                        neighbour,
                        &mut merge_group,
                        &mut groups_to_merge,
                    );
                }
            }
        }

        if let Some(merged) = merge_group {
            for &x in &group.cells {
                let groups = self.match_board.entry(x).or_default();
                if !groups.contains(&merged) {
                    groups.push(merged)
                }
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

                    #[cfg(debug_assertions)]
                    if groups.is_empty() {
                        panic!(
                            "Something gone extremely wrong\ngroup: {:?}\nmatch_groups: {:?}\ngroup: {:?}",
                            merged, self.match_board, group.cells
                        );
                    }

                    let mut remove_after = groups.len() - 1;
                    for (i, x) in groups.iter_mut().enumerate() {
                        if x == &other_group_idx {
                            *x = merged;
                            remove_after = i;
                        }
                    }
                    for i in ((remove_after + 1)..groups.len()).rev() {
                        if groups[i] == merged {
                            groups.remove(i);
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

            #[cfg(debug_assertions)]
            {
                for (idx, groups) in self.match_board.iter() {
                    for i in 0..(groups.len() - 1) {
                        if groups[(i + 1)..].contains(&groups[i]) {
                            panic!("Cell {} has bad groups composition: {:?}", idx, groups)
                        }
                    }
                }
            }

            group.cells.clear();
            self.match_cells_cache = Some(group.cells);
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
        let mut current_match: Option<BoardMatch<Color>> = None;
        let mut was_wildcard = false;
        for i in 0..line.len() {
            let pos = line[i];
            let gem = self.cells[pos].as_ref();
            let can_start_match = gem.can_start_match();
            let can_be_matched = !gem.hint_is_unmatchable();

            if let Some(ref mut match_group) = current_match {
                if !match_group.color.matches(gem) {
                    let group = std::mem::take(&mut current_match)
                        .expect("Should have a match group to close");
                    self.close_match(group);
                } else {
                    match_group.cells.push(pos);
                }
            }

            if current_match.is_none() && can_start_match && can_be_matched {
                let mut group = BoardMatch::<Color>::new(
                    gem.clone(),
                    std::mem::take(&mut self.match_cells_cache).unwrap_or_default(),
                );

                if was_wildcard {
                    for i in (0..i).rev() {
                        let back_pos = line[i];
                        let back_gem = self.cells[back_pos].as_ref();
                        if !group.color.matches(back_gem) {
                            break;
                        }
                        group.cells.insert(0, back_pos);
                    }
                }

                group.cells.push(pos);

                current_match = Some(group);
            }
            was_wildcard = !can_start_match && can_be_matched;
        }

        if let Some(group) = current_match {
            self.close_match(group);
        }
    }
}
