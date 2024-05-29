use crate::assets::sprite::{SpriteData, SpriteDataSerialized};
use crate::GemColorId;
use ahash::AHashMap;
use nohash_hasher::IntMap;
use scrapcore_serialization::derive::DatabaseModel;
use scrapcore_serialization::ItemId;

#[derive(Debug, DatabaseModel)]
pub struct GemColorModel {
    pub sprite: SpriteData,
}

/// Modifiers for gems, like "enchanted"
#[derive(Debug, DatabaseModel)]
pub struct GemModifierModel {
    /// Sprite overlay for the gem
    pub overlay: SpriteData,
    /// Custom sprite replacements for each color
    ///
    /// If sprite replacement is found, `overlay` will not be added
    #[model_attr(schemars(with = "std::collections::HashMap<String, String>"))]
    #[model(ty = "AHashMap<ItemId, SpriteDataSerialized>")]
    pub sprite_changes: IntMap<GemColorId, SpriteData>,
}
