use std::sync::Arc;
use std::fmt::Debug;
use std::future::Future;
use derive_builder::Builder;

use async_trait::async_trait;

use super::prelude::{Uuid, Hash, IndexMap, LoadResult, Item, Tag,ModelVolume};

#[derive(Clone, Debug, Builder)]
pub struct Volume<TD, ID, VD, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug,
        ID: Debug,
        VD: Debug,
        Loader: Fn(&Hash) -> LoadResult<Body>,
        AsyncLoader: Fn(&Hash) -> TF,
        TF: Future<Output = LoadResult<Body>>
{
    pub uuid: Uuid,
    pub data: VD,
    pub root: Arc<Tag<TD, ID>>,
    #[builder(default)]
    pub items: IndexMap<Uuid, Arc<Item<TD, ID>>>,

    loader: Loader,
    async_loader: AsyncLoader,
}

#[async_trait]
impl<TD, ID, VD, Body, Loader, AsyncLoader, TF> ModelVolume for Volume<TD, ID, VD, Body, Loader, AsyncLoader, TF>
    where
        TD: Debug + Send + Sync,
        ID: Debug + Send + Sync,
        VD: Debug + Send + Sync,
        Body: Send + Sync,
        Loader: Fn(&Hash) -> LoadResult<Body> + Send + Sync,
        AsyncLoader: Fn(&Hash) -> TF + Send + Sync,
        TF: Future<Output = LoadResult<Body>> + Send + Sync,
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

    fn get_item(&self, uuid: &Uuid) -> Option<&Self::Item> {
        self.items.get(uuid).map(|x| x.as_ref())
    }

    fn each_item<F: Fn(&Self::Item) -> bool>(&self, callback: &F) -> bool {
        for item in self.items.values() {
            if callback(item) {
                return true;
            }
        }
        false
    }

    fn load_body(&self, hash: &Hash) -> LoadResult<Self::Body> {
        (self.loader)(hash)
    }

    async fn load_body_async(&self, hash: &Hash) -> LoadResult<Self::Body> {
        (self.async_loader)(hash).await
    }
}