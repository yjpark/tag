use std::fmt::Debug;
use std::sync::Arc;
use std::future::Future;
use snafu::prelude::*;
use super::prelude::{Uuid, Hash, DashMap, Item, ItemData, Tag, LoadBodyResult, Volume};

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
    loader: Loader,
    async_loader: AsyncLoader,
    tag_data_factory: TagDataFactory,
    item_data_factory: ItemDataFactory,
    item_uuids: ItemUuidIterator,
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
        ItemUuidIterator: Iterator<Item = Uuid>,
{
    let mut root = Tag::<TD, ID>::root_arc(uuid.clone(), tag_data_factory(&uuid));
    let mut items = DashMap::new();
    for item_uuid in item_uuids {
        let item_data = item_data_factory(&item_uuid);
        let mut item = Item::<TD, ID> {
            uuid: item_uuid.clone(),
            data: item_data,
            tags: DashMap::new(),
        };
        items.insert(item_uuid.clone(), Arc::new(item));
    }
    Ok(Volume::<TD, ID, VD, Body, Loader, AsyncLoader, TF>::new(
        uuid, data, root, items, loader, async_loader,
    ))
}
