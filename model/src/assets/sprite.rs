use crate::assets::color::{ColorData, ColorDataSerialized};
use ahash::AHashMap;
use assets_manager::SharedString;
use atomic_refcell::{AtomicRef, AtomicRefCell};
use macroquad::color::{Color, WHITE};
use macroquad::miniquad::TextureId;
use macroquad::texture::Texture2D;
use schemars::JsonSchema;
use scrapcore_serialization::registry::{AssetsHolder, PartialRegistry};
use scrapcore_serialization::serialization::error::{
    DeserializationError, DeserializationErrorKind,
};
use scrapcore_serialization::serialization::{DeserializeModel, SerializationFallback};
use scrapcore_serialization::{AssetNameRef, ItemId};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static HANDLES: OnceLock<AtomicRefCell<AHashMap<String, Texture2D>>> = OnceLock::new();

pub(crate) fn texture_handles() -> &'static AtomicRefCell<AHashMap<String, Texture2D>> {
    HANDLES.get_or_init(|| AtomicRefCell::new(AHashMap::new()))
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct SpriteId(pub(crate) SharedString);

/// Sprite data, currently contains texture and an optional tint
///
/// Can be deserialized from various formats
/// - A plain string with texture name
/// - An object with `sprite` and `tint` fields
#[derive(Debug)]
pub struct SpriteData {
    pub sprite: SpriteId,
    pub tint: Option<Color>,
}

impl SpriteData {
    /// Returns tint color, or white color if none
    pub fn tint_or_white(&self) -> Color {
        self.tint.unwrap_or(WHITE)
    }

    pub fn texture(&self) -> AtomicRef<Texture2D> {
        AtomicRef::map(texture_handles().borrow(), |m| {
            m.get(self.sprite.0.as_str()).unwrap()
        })
    }

    pub fn texture_id(&self) -> TextureId {
        texture_handles()
            .borrow()
            .get(self.sprite.0.as_str())
            .unwrap()
            .raw_miniquad_id()
    }
}

// <================================>
// <===== Deserialization code =====>
// <================================>

// TODO: this is boileplate-y, make into a macro
impl<'a, Registry: PartialRegistry> DeserializeModel<SpriteId, Registry> for AssetNameRef<'a>
where
    Registry: AssetsHolder<SpriteId>,
{
    fn deserialize(
        self,
        registry: &mut Registry,
    ) -> Result<SpriteId, DeserializationError<Registry>> {
        let name = self.to_ascii_lowercase();
        if let Some(handle) = registry.get_assets().get(&name) {
            Ok(handle.0.clone())
        } else {
            Err(DeserializationErrorKind::MissingAsset(name, Registry::asset_kind()).into())
        }
    }
}

impl SerializationFallback for SpriteData {
    type Fallback = SpriteDataSerialized;
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SpriteDataSerialized {
    JustTexture(ItemId),
    Data {
        sprite: ItemId,
        tint: Option<ColorDataSerialized>,
    },
}

impl<Registry: PartialRegistry> DeserializeModel<SpriteData, Registry> for SpriteDataSerialized
where
    Registry: AssetsHolder<SpriteId>,
    ColorDataSerialized: DeserializeModel<ColorData, Registry>,
{
    fn deserialize(
        self,
        registry: &mut Registry,
    ) -> Result<SpriteData, DeserializationError<Registry>> {
        Ok(match self {
            SpriteDataSerialized::JustTexture(id) => {
                let texture: SpriteId = DeserializeModel::deserialize(id, registry)?;

                SpriteData {
                    sprite: texture,
                    tint: None,
                }
            }
            SpriteDataSerialized::Data { sprite, tint } => {
                let sprite: SpriteId = DeserializeModel::deserialize(sprite, registry)?;
                let tint: Option<ColorData> = DeserializeModel::deserialize(tint, registry)?;

                SpriteData {
                    sprite,
                    tint: tint.map(Into::into),
                }
            }
        })
    }
}
