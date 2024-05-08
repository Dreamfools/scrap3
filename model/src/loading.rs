use crate::assets::sprite::texture_handles;
use crate::registry::{PartialRegistry, Registry, RegistryItemSerialized};
use crate::SpriteId;
use assets_manager::loader::{BytesLoader, LoadFrom};
use assets_manager::{loader, Asset, AssetCache, Compound, Handle, RecursiveDirectory};
use itertools::Itertools;
use macroquad::prelude::Texture2D;
use miette::Report;
use scrapcore_serialization::registry::insert::asset_insert;
use scrapcore_serialization::registry::path_identifier::PathIdentifier;
use scrapcore_serialization::registry::PartialRegistry as _;
use scrapcore_serialization::serialization::error::{
    DeserializationError, DeserializationErrorKind, DeserializationErrorStackItem,
};

struct ImageBytes(Vec<u8>);

impl From<Vec<u8>> for ImageBytes {
    fn from(bytes: Vec<u8>) -> ImageBytes {
        ImageBytes(bytes)
    }
}

impl Asset for ImageBytes {
    const EXTENSIONS: &'static [&'static str] = &["png", "jpg", "jpeg"];
    type Loader = LoadFrom<Vec<u8>, BytesLoader>;
}

impl Asset for RegistryItemSerialized {
    const EXTENSION: &'static str = "json";
    type Loader = loader::JsonLoader;
}

type ItemsHandle = Handle<RecursiveDirectory<RegistryItemSerialized>>;
type ImagesHandle = Handle<RecursiveDirectory<ImageBytes>>;

#[derive(Debug)]
pub struct LoadedMod<'a> {
    registry: Registry,
    cache: &'a AssetCache,
    items: &'a ItemsHandle,
    images: &'a ImagesHandle,

    want_full_reload: bool,
}

impl<'a> LoadedMod<'a> {
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Loads a mod given the asset cache
    ///
    /// Errors can be safely handed, and should not affect any global state
    pub fn load_mod(cache: &'a AssetCache) -> Result<Self, Report> {
        Self::load_mod_inner(cache, None).map_err(diagnostic)
    }

    /// Performs hot-reload, updating the mod accordingly, returning `true` if
    /// data files were reloaded (not images or other assets)
    ///
    /// Hot reload is guaranteed to not alter existing loaded item IDs
    ///
    /// Errors can be safely handed, and should not affect any global state
    ///
    /// Some changes can't be safely hot-reloaded, call
    /// [LoadedMod::want_full_reload] to check if a full reload is required
    pub fn hot_reload(&mut self) -> Result<bool, Report> {
        self.cache.hot_reload();

        self.update_images::<false>().map_err(err_m)?;

        if self.want_files_hot_reload().map_err(err_m)? {
            // TODO: stagger loading by a couple of frames, in case of multiple file updates
            println!("[Scrap3 Model]: Hot reloading data files");
            match Self::load_mod_inner(self.cache, Some(&self.registry)) {
                Ok(loaded) => {
                    // Essentially a full reload but we keep IDs
                    self.registry = loaded.registry;
                    self.items = loaded.items;
                    self.images = loaded.images;
                    self.want_full_reload = false;
                }
                Err(err) => {
                    if err.is_hot_reload_blocker() {
                        self.want_full_reload = true;
                    } else {
                        return Err(diagnostic(err));
                    }
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Indicates that a full reload is required, because some changes are not
    /// supported by hot-reload
    ///
    /// This should be called after [LoadedMod::hot_reload]
    pub fn want_full_reload(&self) -> bool {
        self.want_full_reload
    }
}

impl<'a> LoadedMod<'a> {
    fn load_mod_inner(
        cache: &'a AssetCache,
        full_registry: Option<&Registry>,
    ) -> Result<Self, DeserializationError<PartialRegistry>> {
        let image_handles = cache.load_rec_dir::<ImageBytes>("").map_err(err_mr)?;
        let item_handles = cache
            .load_rec_dir::<RegistryItemSerialized>("")
            .map_err(err_mr)?;

        let images = load_items(cache, image_handles).map_err(err_mr)?;
        let items = load_items(cache, item_handles).map_err(err_mr)?;

        let mut reg = PartialRegistry::default();

        if let Some(full_registry) = full_registry {
            reg.reserve_ids(full_registry)?;
        };

        for (path, data) in items {
            reg.insert(path, data.cloned())?;
        }

        let mut sprites = Vec::with_capacity(images.len());
        for (path, data) in images {
            asset_insert(&mut reg, path.clone(), SpriteId(data.id().clone()))?;
            sprites.push(data);
        }

        let registry = reg.into_registry()?;

        let mod_data = Self {
            registry,
            cache,
            items: item_handles,
            images: image_handles,
            want_full_reload: false,
        };

        mod_data.update_images::<true>().map_err(err_mr)?;

        Ok(mod_data)
    }

    /// Updates all images. If error happens, no changes are performed
    ///
    /// If `FORCE` is true, all textures would be updated, otherwise only hot
    /// reloaded textures would
    fn update_images<const FORCE: bool>(&self) -> Result<(), assets_manager::Error> {
        // if !FORCE && !self.images.reloaded_global() {
        //     return Ok(());
        // }
        let mut changes = vec![];
        for data in self.images.read().iter(self.cache) {
            let handle = data?;
            if FORCE || handle.reloaded_global() {
                if !FORCE {
                    println!("[Scrap3 Model]: Updating image `{}`", handle.id());
                }
                changes.push(handle);
            }
        }

        // Confine static handles borrow into a block
        {
            let mut handles = texture_handles().borrow_mut();
            for image in changes {
                let texture2d = Texture2D::from_file_with_format(&image.read().0, None);
                handles.insert(image.id().to_string(), texture2d);
            }
        }

        Ok(())
    }

    fn want_files_hot_reload(&self) -> Result<bool, assets_manager::Error> {
        let mut any_reloaded = false;
        for data in self.items.read().iter(self.cache) {
            let handle = data?;
            if handle.reloaded_global() {
                any_reloaded = true;
            }
        }
        Ok(any_reloaded)
    }
}

fn load_items<'a, T: Compound>(
    cache: &'a AssetCache,
    input: &'a Handle<RecursiveDirectory<T>>,
) -> Result<Vec<(PathIdentifier, &'a Handle<T>)>, assets_manager::Error> {
    let data: Vec<_> = input
        .read()
        .iter(cache.as_any_cache())
        .map(|h| {
            h.map(|h| {
                let ident = PathIdentifier::from_components(h.id().split('.'));
                (ident, h)
            })
        })
        .try_collect()?;

    Ok(data)
}

fn err_mr(e: assets_manager::Error) -> DeserializationError<PartialRegistry> {
    let id = PathIdentifier::from_components(e.id().split('.'));
    let err = e.into_inner();
    DeserializationErrorKind::<PartialRegistry>::LoadingError(err.to_string())
        .into_err()
        .context(DeserializationErrorStackItem::File(id))
}

fn err_m(e: assets_manager::Error) -> Report {
    diagnostic(err_mr(e))
}

fn diagnostic(e: DeserializationError<PartialRegistry>) -> Report {
    Report::from(e.diagnostic())
}
