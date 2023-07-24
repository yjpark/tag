use std::fmt::Debug;
use std::future::Future;
use super::prelude::{Uuid, Hash, Tag, ItemData, LoadBodyResult, Volume};

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
        let item = volume.new_item(item_uuid.clone(), item_data_factory(&item_uuid));
        for proto_tag in item.data.iter_tags() {
            let tag_uuid = proto_tag.uuid();
            let tag = volume.get_tag(tag_uuid)
                .unwrap_or_else(|| {
                    let parent = proto_tag.parent()
                        .and_then(|x| volume.get_tag(&x))
                        .unwrap_or_else(|| volume.root.clone() );
                    Tag::<TD, ID>::new_child(&parent, proto_tag.clone(), tag_data_factory(tag_uuid))
                });
            tag.add_item(item.clone());
        }
    }
    volume
}
