use std::sync::Arc;
use std::fmt::Debug;
use std::future::Future;

use async_trait::async_trait;

use super::prelude::{Uuid, Hash, DashMap, LoadBodyResult, Item, ProtoTag, Tag, ModelVolume};

#[derive(Clone, Debug)]
pub struct Volume<VD, TD, ID, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug,
        ID: Debug ,
        VD: Debug,
        Loader: Fn(&VD, &Hash) -> LoadBodyResult<Body>,
        AsyncLoader: Fn(&VD, &Hash) -> TF,
        TF: Future<Output = LoadBodyResult<Body>>
{
    loader: Loader,
    async_loader: AsyncLoader,

    pub uuid: Uuid,
    pub data: VD,
    pub root: Arc<Tag<TD, ID>>,
    pub items: DashMap<Uuid, Arc<Item<TD, ID>>>,
}

#[async_trait]
impl<VD, TD, ID, Body, Loader, AsyncLoader, TF> ModelVolume for Volume<VD, TD, ID, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug + Send + Sync + 'static,
        ID: Debug  + Send + Sync + 'static,
        VD: Debug + Send + Sync + 'static,
        Body: Send + Sync,
        Loader: Fn(&VD, &Hash) -> LoadBodyResult<Body> + Send + Sync,
        AsyncLoader: Fn(&VD, &Hash) -> TF + Send + Sync,
        TF: Future<Output = LoadBodyResult<Body>> + Send + Sync,
{
    type Data = VD;
    type Tag = Tag<TD, ID>;
    type Item = Item<TD, ID>;
    type Body = Body;

    fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn root(&self) -> &Self::Tag {
        self.root.as_ref()
    }

    fn items_count(&self) -> usize {
        self.items.len()
    }

    fn each_item<F: Fn(&Self::Item) -> bool>(&self, callback: &F) -> bool {
        for kv in self.items.iter() {
            if callback(kv.value()) {
                return true;
            }
        }
        false
    }

    fn with_item<O, F: Fn(Option<&Self::Item>) -> O>(&self, uuid: &Uuid, callback: &F) -> O {
        match self.items.get(uuid) {
            None => callback(None),
            Some(kv) => callback(Some(kv.value().as_ref())),
        }
    }

    fn load_body(&self, hash: &Hash) -> LoadBodyResult<Self::Body> {
        (self.loader)(&self.data, hash)
    }

    async fn load_body_async(&self, hash: &Hash) -> LoadBodyResult<Self::Body> {
        (self.async_loader)(&self.data, hash).await
    }
}

impl<VD, TD, ID, Body, Loader, AsyncLoader, TF> Volume<VD, TD, ID, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug + Send + Sync,
        ID: Debug  + Send + Sync,
        VD: Debug + Send + Sync,
        Body: Send + Sync,
        Loader: Fn(&VD, &Hash) -> LoadBodyResult<Body> + Send + Sync,
        AsyncLoader: Fn(&VD, &Hash) -> TF + Send + Sync,
        TF: Future<Output = LoadBodyResult<Body>> + Send + Sync,
{
    // loader and async_loader are private
    pub fn new(
        loader: Loader,
        async_loader: AsyncLoader,
        uuid: Uuid,
        data: VD,
        root_data: TD,
    ) -> Self {
        let root_proto = Arc::new(<dyn ProtoTag>::root(uuid.clone()));
        let root = Tag::<TD, ID>::new_arc(root_proto, root_data, None);
        Self {
            loader,
            async_loader,
            uuid,
            data,
            root,
            items: DashMap::new(),
        }
    }

    pub fn new_item(&self,
        item_uuid: Uuid,
        item_data: ID,
    ) -> Arc<Item<TD, ID>> {
        let item = Item::<TD, ID>::new_arc(item_uuid, item_data);
        self.add_item(item.clone());
        item
    }

    pub fn add_item(&self, item: Arc<Item<TD, ID>>) {
        self.items.insert(item.uuid.clone(), item);
    }

    pub fn get_tag(&self, uuid: &Uuid) -> Option<Arc<Tag<TD, ID>>> {
        self.root.get_child_deep(uuid)
    }
}