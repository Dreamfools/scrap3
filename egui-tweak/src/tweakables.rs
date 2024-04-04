use duplicate::duplicate_item;
use egui::Ui;

use crate::Tweakable;

impl Tweakable for bool {
    fn draw(&mut self, ui: &mut Ui, name: &str) {
        ui.checkbox(self, name);
    }
}

#[duplicate_item(
    int_type;
    [ u8 ];
    [ u16 ];
    [ u32 ];
    [ u64 ];
    [ i8 ];
    [ i16 ];
    [ i32 ];
    [ i64 ];
    [ f32 ];
    [ f64 ];
)]
impl Tweakable for int_type {
    fn draw(&mut self, ui: &mut Ui, name: &str) {
        ui.horizontal(|ui| {
            ui.label(name);
            ui.add(egui::DragValue::new(self));
        });
    }
}
