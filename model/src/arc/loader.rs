use std::fmt::Debug;
use std::sync::Arc;
use std::future::Future;
use snafu::prelude::*;
use tag_proto::{prelude::IndexMap, val::ValTag};

use super::prelude::{Uuid, Hash, ItemData, Tag, LoadBodyResult, Volume};

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
    let root_proto = ValTag::<bool> {
        uuid: Uuid::new_v4(),
        parent: None,
        val: false,
    };
    let root = Tag::<TD, ID> {
        data: root_data,
        proto: Arc::new(root_proto),
        parent: None,
        children: IndexMap::new(),
        items: IndexMap::new(),
    };
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
