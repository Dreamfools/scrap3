use crate::assets::sprite::{SpriteData, SpriteDataSerialized};
use crate::registry::GemColorId;
use ahash::AHashMap;
use nohash_hasher::IntMap;
use scrapcore_serialization::derive::DatabaseModel;
use scrapcore_serialization::ItemId;

#[derive(Debug, DatabaseModel)]
pub struct GemColor {
    pub sprite: SpriteData,
}

/// Modifiers for gems, like "enchanted"
#[derive(Debug, DatabaseModel)]
pub struct GemModifier {
    /// Sprite overlay for the gem
    pub overlay: SpriteData,
    /// Custom sprite replacements for each color
    ///
    /// If sprite replacement is found, `overlay` will not be added
    #[model_attr(schemars(with = "std::collections::HashMap<String, String>"))]
    #[model(ty = "AHashMap<ItemId, SpriteDataSerialized>")]
    pub sprite_changes: IntMap<GemColorId, SpriteData>,
}
