use yakui::widgets::Pad;

pub trait PadExt {
    fn left(x: f32) -> Self;
    fn right(x: f32) -> Self;
    fn top(x: f32) -> Self;
    fn bottom(x: f32) -> Self;
}

impl PadExt for Pad {
    fn left(x: f32) -> Self {
        let mut p = Self::ZERO;
        p.left = x;
        p
    }

    fn right(x: f32) -> Self {
        let mut p = Self::ZERO;
        p.right = x;
        p
    }

    fn top(x: f32) -> Self {
        let mut p = Self::ZERO;
        p.top = x;
        p
    }

    fn bottom(x: f32) -> Self {
        let mut p = Self::ZERO;
        p.bottom = x;
        p
    }
}
