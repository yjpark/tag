use std::fmt::Debug;
use std::future::Future;
use super::prelude::{Uuid, Hash, ItemData, LoadBodyResult, Volume};

pub fn load_volume<
        VD, TD, ID, Body, Loader, AsyncLoader, TF,
        TagDataFactory, ItemDataFactory, ItemUuidIterator> (
    uuid: Uuid,
    data: VD,
    loader: Loader,
    async_loader: AsyncLoader,
    tag_data_factory: TagDataFactory,
    item_data_factory: ItemDataFactory,
    item_uuids: ItemUuidIterator,
) -> Volume<VD, TD, ID, Body, Loader, AsyncLoader, TF>
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
    let volume = Volume::<VD, TD, ID, Body, Loader, AsyncLoader, TF>::new(
        loader, async_loader, uuid, data, tag_data_factory(&uuid),
    );
    for item_uuid in item_uuids {
        let item_data = item_data_factory(&item_uuid);
    }
    volume
}
