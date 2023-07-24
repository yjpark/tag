use std::fmt::Debug;
use std::sync::Arc;
use std::future::Future;
use snafu::prelude::*;
use super::prelude::{Uuid, Hash, IndexMap, ValTag, ProtoTag, ItemData, Tag, LoadBodyResult, Volume};

#[derive(Debug, Snafu)]
pub enum LoadVolumeError {
    #[snafu(display("Not implemented"))]
    NotImplemented,
    #[snafu(display("IO failed: {} -> {}", info, error))]
    IoFailed { error: std::io::Error, info: String },
}

pub fn load_volume<
        TD, ID, VD, Body, Loader, AsyncLoader, TF,
        TagDataFactory, ItemDataFactory, ItemUuidIterator> (
    uuid: Uuid,
    data: VD,
    root_data: TD,
    loader: Loader,
    async_loader: AsyncLoader,
    tag_data_factory: TagDataFactory,
    item_data_factory: ItemDataFactory,
    items: ItemUuidIterator,
) -> Result<Volume<TD, ID, VD, Body, Loader, AsyncLoader, TF>, LoadVolumeError>
    where
        TD: Debug + Send + Sync,
        ID: Debug + ItemData + Send + Sync,
        VD: Debug + Send + Sync,
        Body: Send + Sync,
        Loader: Fn(&VD, &Hash) -> LoadBodyResult<Body> + Send + Sync,
        AsyncLoader: Fn(&VD, &Hash) -> TF + Send + Sync,
        TF: Future<Output = LoadBodyResult<Body>> + Send + Sync,
        TagDataFactory: Fn(&Uuid) -> TD,
        ItemDataFactory: Fn(&Uuid) -> ID,
        ItemUuidIterator: Iterator<Item = ID>,
{
    let root = Tag::<TD, ID>::root(uuid.clone(), root_data);
    let items = IndexMap::new();
    Ok(Volume::<TD, ID, VD, Body, Loader, AsyncLoader, TF>::new(
        uuid,
        data,
        Arc::new(root),
        items,
        loader,
        async_loader,
    ))
}
