use crate::{Name, Tweakable};
use egui::Ui;
use std::iter::Peekable;

pub fn edit_tweakables(ui: &mut Ui) {
    let mut tweakables = crate::get_all_tweakables();
    ui.vertical(|ui| edit_items(ui, &[], &mut tweakables.iter_mut().peekable()));
}

/// Recursively draw tree of tweekable items
fn edit_items<'a, Iter: Iterator<Item = (&'a Name, &'a mut Box<dyn Tweakable>)>>(
    ui: &mut Ui,
    prefix: &[&str],
    items: &mut Peekable<Iter>,
) {
    while let Some((path, _)) = items.peek() {
        let mut segments: Vec<&str> = path.split('.').collect();
        if !segments.starts_with(prefix) {
            return;
        }
        let name = segments
            .pop()
            .expect("Tweakable paths should not be empty")
            .to_string();
        if prefix.len() != segments.len() {
            let next_level = segments[..(prefix.len() + 1)].to_vec();
            let next_level_str = next_level.join(".");
            ui.collapsing(next_level.last().unwrap().to_string(), |ui| {
                edit_items(ui, &next_level, items);
            });
            while items
                .peek()
                .is_some_and(|(name, _)| name.starts_with(&next_level_str))
            {
                items.next();
            }
        } else {
            let (_, item) = items.next().expect("Item was peeked at before");
            item.draw(ui, &name);
        }
    }
}
