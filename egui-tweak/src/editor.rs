use crate::{Name, Tweakable};
use egui::Ui;
use std::iter::Peekable;

pub fn edit_tweakables(ui: &mut Ui) {
    let mut tweakables = crate::get_all_tweakables();
    ui.vertical(|ui| edit_items(ui, "", &mut tweakables.iter_mut().peekable()));
}

/// Recursively draw tree of tweekable items
fn edit_items<'a, Iter: Iterator<Item = (&'a Name, &'a mut Box<dyn Tweakable>)>>(
    ui: &mut Ui,
    prefix: &str,
    items: &mut Peekable<Iter>,
) {
    while let Some((path, _)) = items.peek() {
        if !path.starts_with(prefix) {
            return;
        }
        let mut segments: Vec<&str> = path.split('.').collect();
        let name = segments
            .pop()
            .expect("Tweakable paths should not be empty")
            .to_string();
        let new_prefix = segments.join(".");
        if prefix != new_prefix {
            let collapsed = new_prefix.trim_start_matches(prefix);
            ui.collapsing(collapsed, |ui| {
                edit_items(ui, &new_prefix, items);
            });
            while items
                .peek()
                .is_some_and(|(name, _)| name.starts_with(&new_prefix))
            {
                items.next();
            }
        } else {
            let (_, item) = items.next().expect("Item was peeked at before");
            item.draw(ui, &name);
        }
    }
}
