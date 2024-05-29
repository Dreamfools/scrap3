use match3::SimpleGem;

pub type Gem = SimpleGem<GemColor>;

#[derive(Debug, Clone, Default)]
pub enum GemColor {
    #[default]
    Empty,
    Color(model::GemColorId),
}

impl match3::MatchColor for GemColor {
    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (GemColor::Color(c1), GemColor::Color(c2)) => c1 == c2,
            _ => false,
        }
    }

    fn can_start_match(&self) -> bool {
        match self {
            GemColor::Empty => false,
            GemColor::Color(_) => true,
        }
    }

    fn hint_is_unmatchable(&self) -> bool {
        match self {
            GemColor::Empty => true,
            GemColor::Color(_) => false,
        }
    }
}
