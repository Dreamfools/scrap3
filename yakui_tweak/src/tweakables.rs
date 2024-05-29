use duplicate::duplicate_item;
use std::fmt::Display;
use std::str::FromStr;
use yakui::widgets::List;
use yakui::{checkbox, label, textbox, use_state, MainAxisSize};
use yakui_widgets_plus::list_ext::{row_shrink, ListExt};

use crate::Tweakable;

impl Tweakable for bool {
    fn draw(&mut self, name: &str) {
        row_shrink(|| {
            label(name.to_string());
            *self = checkbox(*self).checked;
        });
    }
}

impl Tweakable for String {
    fn draw(&mut self, name: &str) {
        row_shrink(|| {
            label(name.to_string());
            if let Some(res) = textbox(self.clone()).into_inner().text {
                *self = res;
            }
        });
    }
}

pub trait StringEditable: 'static + Sync + Send + Clone + Display + FromStr {}

impl<T: StringEditable> Tweakable for T {
    fn draw(&mut self, name: &str) {
        row_shrink(|| {
            label(name.to_string());
            let text = self.to_string();
            let text_data = use_state(|| text);

            let res = textbox(text_data.borrow().to_string());
            let update = res.lost_focus || res.activated;
            if let Some(text) = res.into_inner().text {
                *text_data.borrow_mut() = text;
            }
            if update {
                if let Ok(res) = text_data.borrow().parse() {
                    *self = res;
                }
                *text_data.borrow_mut() = self.to_string();
            }
        });
    }
}

#[duplicate_item(
    num_type;
    [ u8 ];
    [ u16 ];
    [ u32 ];
    [ u64 ];
    [ u128 ];
    [ usize ];
    [ i8 ];
    [ i16 ];
    [ i32 ];
    [ i64 ];
    [ i128 ];
    [ isize ];
    [ f32 ];
    [ f64 ];
)]
impl StringEditable for num_type {}
