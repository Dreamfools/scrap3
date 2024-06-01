use yakui::widgets::{List, ListResponse};
use yakui::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize, Response};

// with_direction: Direction,
// with_item_spacing: f32,
// with_main_axis_size: MainAxisSize,
// with_main_axis_alignment: MainAxisAlignment,
// with_cross_axis_alignment: CrossAxisAlignment,

pub trait ListExt {
    fn with_item_spacing(self, x: f32) -> Self;
    fn with_main_axis_size(self, x: MainAxisSize) -> Self;
    fn with_main_axis_alignment(self, x: MainAxisAlignment) -> Self;
    fn with_cross_axis_alignment(self, x: CrossAxisAlignment) -> Self;
}

impl ListExt for List {
    fn with_item_spacing(mut self, x: f32) -> Self {
        self.item_spacing = x;
        self
    }

    fn with_main_axis_size(mut self, x: MainAxisSize) -> Self {
        self.main_axis_size = x;
        self
    }

    fn with_main_axis_alignment(mut self, x: MainAxisAlignment) -> Self {
        self.main_axis_alignment = x;
        self
    }

    fn with_cross_axis_alignment(mut self, x: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = x;
        self
    }
}

pub fn column_shrink(children: impl FnOnce()) -> Response<ListResponse> {
    List::column()
        .with_main_axis_size(MainAxisSize::Min)
        .show(children)
}

pub fn row_shrink(children: impl FnOnce()) -> Response<ListResponse> {
    List::row()
        .with_main_axis_size(MainAxisSize::Min)
        .show(children)
}
