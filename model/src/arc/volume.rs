use std::sync::Arc;
use std::fmt::Debug;
use std::future::Future;

use async_trait::async_trait;

use super::prelude::{Uuid, Hash, DashMap, LoadBodyResult, Item, ItemData, Tag,ModelVolume};

#[derive(Clone, Debug)]
pub struct Volume<TD, ID, VD, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug,
        ID: Debug + ItemData,
        VD: Debug,
        Loader: Fn(&VD, &Hash) -> LoadBodyResult<Body>,
        AsyncLoader: Fn(&VD, &Hash) -> TF,
        TF: Future<Output = LoadBodyResult<Body>>
{
    pub uuid: Uuid,
    pub data: VD,
    pub root: Arc<Tag<TD, ID>>,
    pub items: DashMap<Uuid, Arc<Item<TD, ID>>>,

    loader: Loader,
    async_loader: AsyncLoader,
}

#[async_trait]
impl<TD, ID, VD, Body, Loader, AsyncLoader, TF> ModelVolume for Volume<TD, ID, VD, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug + Send + Sync,
        ID: Debug + ItemData + Send + Sync,
        VD: Debug + Send + Sync,
        Body: Send + Sync,
        Loader: Fn(&VD, &Hash) -> LoadBodyResult<Body> + Send + Sync,
        AsyncLoader: Fn(&VD, &Hash) -> TF + Send + Sync,
        TF: Future<Output = LoadBodyResult<Body>> + Send + Sync,
{
    type Tag = Tag<TD, ID>;
    type Data = VD;
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
