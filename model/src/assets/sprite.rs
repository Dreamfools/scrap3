use crate::assets::color::{ColorData, ColorDataSerialized};
use ahash::AHashMap;
use assets_manager::{SharedBytes, SharedString};
use atomic_refcell::{AtomicRefCell, AtomicRefMut};
use macroquad::color::{Color, WHITE};
use macroquad::texture::Texture2D;
use schemars::JsonSchema;
use scrapcore_serialization::registry::{AssetsHolder, PartialRegistry};
use scrapcore_serialization::serialization::error::{
    DeserializationError, DeserializationErrorKind,
};
use scrapcore_serialization::serialization::{DeserializeModel, SerializationFallback};
use scrapcore_serialization::{AssetNameRef, ItemId};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::sync::OnceLock;
use yakui::paint::{TextureFilter, TextureFormat};
use yakui::UVec2;

#[derive(Debug)]
struct ImageHandle {
    bytes: SharedBytes,
    macroquad: Option<Texture2D>,
    yakui: Option<yakui::TextureId>,
}

impl ImageHandle {
    pub fn new(bytes: SharedBytes) -> Self {
        Self {
            bytes,
            macroquad: None,
            yakui: None,
        }
    }
}

static HANDLES: OnceLock<AtomicRefCell<AHashMap<String, ImageHandle>>> = OnceLock::new();

fn texture_handles() -> &'static AtomicRefCell<AHashMap<String, ImageHandle>> {
    HANDLES.get_or_init(|| AtomicRefCell::new(AHashMap::new()))
}

#[derive(Debug)]
pub struct TextureCache(AtomicRefMut<'static, AHashMap<String, ImageHandle>>);

impl TextureCache {
    pub fn open() -> Self {
        Self(texture_handles().borrow_mut())
    }
    pub fn add_texture(&mut self, k: String, bytes: SharedBytes) {
        match self.0.entry(k) {
            Entry::Occupied(mut entry) => {
                // Bytes are the same, so skip replacing
                if entry.get().bytes == bytes {
                    return;
                }
                let old = entry.insert(ImageHandle::new(bytes));

                // Macroquad does texture GC by itself, so only worry about yakui
                if let Some(_id) = old.yakui {
                    // TODO: delete old yakui texture
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(ImageHandle::new(bytes));
            }
        }
    }
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

    pub fn texture2d(&self) -> Texture2D {
        let mut textures = texture_handles().borrow_mut();

        let texture = textures.get_mut(self.sprite.0.as_str()).unwrap();
        texture
            .macroquad
            .get_or_insert_with(|| Texture2D::from_file_with_format(texture.bytes.as_ref(), None))
            .weak_clone()
    }

    pub fn yakui_texture(&self) -> yakui::TextureId {
        let mut textures = texture_handles().borrow_mut();

        let texture = textures.get_mut(self.sprite.0.as_str()).unwrap();
        *texture.yakui.get_or_insert_with(|| {
            let image = image::load_from_memory(texture.bytes.as_ref())
                .unwrap()
                .into_rgba8();
            let size = UVec2::new(image.width(), image.height());

            let mut texture =
                yakui::paint::Texture::new(TextureFormat::Rgba8Srgb, size, image.into_raw());
            texture.mag_filter = TextureFilter::Linear;
            yakui::TextureId::Managed(yakui_macroquad::cfg(|yak| yak.add_texture(texture)))
        })
    }

    // pub fn texture_id(&self) -> TextureId {
    //     texture_handles()
    //         .borrow()
    //         .get(self.sprite.0.as_str())
    //         .unwrap()
    //         .raw_miniquad_id()
    // }
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
