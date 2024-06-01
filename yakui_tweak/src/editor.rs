use std::iter::Peekable;

use yakui::widgets::Pad;
use yakui::{colored_box_container, draggable, offset, pad, use_state, Color, Vec2};

use yakui_widgets_plus::collapsing::collapsing;
use yakui_widgets_plus::list_ext::column_shrink;

use crate::{Name, Tweakable};

pub fn edit_tweakables() {
    let debug_window_pos = use_state(|| Vec2::ZERO);
    let offset_pos = *debug_window_pos.borrow();
    offset(offset_pos, || {
        column_shrink(|| {
            colored_box_container(Color::GRAY, || {
                if let Some(drag) = draggable(|| {
                    pad(Pad::all(3.0), || {
                        collapsing("Tweakables", || {
                            let mut tweakables = crate::get_all_tweakables();
                            column_shrink(|| {
                                edit_items(&[], &mut tweakables.iter_mut().peekable())
                            });
                        });
                    });
                })
                .dragging
                {
                    let x = drag.current.x.max(0.0);
                    let y = drag.current.y.max(0.0);
                    *debug_window_pos.borrow_mut() = Vec2::new(x, y);
                }
            });
        });
    });
}

/// Recursively draw tree of tweekable items
fn edit_items<'a, Iter: Iterator<Item = (&'a Name, &'a mut Box<dyn Tweakable>)>>(
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
            collapsing(next_level.last().unwrap().to_string(), || {
                edit_items(&next_level, items);
            });
            while items
                .peek()
                .is_some_and(|(name, _)| name.starts_with(&next_level_str))
            {
                items.next();
            }
        } else {
            let (_, item) = items.next().expect("Item was peeked at before");
            item.draw(&name);
        }
    }
}
