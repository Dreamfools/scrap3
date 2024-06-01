use crate::assets::sprite::TextureCache;
use crate::loading::staggerer::{Staggerer, StaggererImpl};
use crate::registry::{PartialRegistry, Registry, RegistryItemSerialized};
use crate::SpriteId;
use assets_manager::loader::{BytesLoader, LoadFrom};
use assets_manager::source::Source;
use assets_manager::{
    loader, Asset, AssetCache, Compound, Handle, RecursiveDirectory, SharedBytes,
};
use itertools::Itertools;
use macroquad::logging::{debug, info, trace, warn};
use miette::Report;
use scrapcore_serialization::registry::insert::asset_insert;
use scrapcore_serialization::registry::path_identifier::PathIdentifier;
use scrapcore_serialization::registry::PartialRegistry as _;
use scrapcore_serialization::serialization::error::{
    DeserializationError, DeserializationErrorKind, DeserializationErrorStackItem,
};

mod staggerer;

struct ImageBytes(SharedBytes);

impl From<SharedBytes> for ImageBytes {
    fn from(bytes: SharedBytes) -> ImageBytes {
        ImageBytes(bytes)
    }
}

impl Asset for ImageBytes {
    const EXTENSIONS: &'static [&'static str] = &["png", "jpg", "jpeg"];
    type Loader = LoadFrom<SharedBytes, BytesLoader>;
}

impl Asset for RegistryItemSerialized {
    const EXTENSION: &'static str = "json";
    type Loader = loader::JsonLoader;
}

type ItemsHandle = Handle<RecursiveDirectory<RegistryItemSerialized>>;
type ImagesHandle = Handle<RecursiveDirectory<ImageBytes>>;

#[derive(Debug)]
pub struct LoadedMod<'a, S> {
    registry: Registry,
    cache: &'a AssetCache<S>,
    items: &'a ItemsHandle,
    images: &'a ImagesHandle,

    hot_reload_stagger: StaggererImpl,
    want_full_reload: bool,
}

impl<'a, S: Source + Sync> LoadedMod<'a, S> {
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Loads a mod given the asset cache
    ///
    /// Errors can be safely handed, and should not affect any global state
    pub fn load_mod(cache: &'a AssetCache<S>) -> Result<Self, Report> {
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
            trace!("[Scrap3 Model]: File reload detected, queueing hot reload");
            self.hot_reload_stagger.trigger();
        }

        if self.hot_reload_stagger.activated() {
            // TODO: stagger loading by a couple of frames, in case of multiple file updates
            info!("[Scrap3 Model]: Hot reloading data files");
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
                        warn!("[Scrap3 Model]: Full reload is required");
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

impl<'a, S: Source> LoadedMod<'a, S> {
    fn load_mod_inner(
        cache: &'a AssetCache<S>,
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
            hot_reload_stagger: Staggerer::new(0.150, 1.0),
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
                    debug!("[Scrap3 Model]: Updating image `{}`", handle.id());
                }
                changes.push(handle);
            }
        }

        // Confine static handles borrow into a block
        {
            let mut cache = TextureCache::open();
            for image in changes {
                let bytes = image.read().0.clone();
                cache.add_texture(image.id().to_string(), bytes);
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

fn load_items<'a, T: Compound, S: Source>(
    cache: &'a AssetCache<S>,
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
