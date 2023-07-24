use std::sync::Arc;
use std::slice::Iter;
use std::fmt::Debug;

use super::prelude::{Uuid, DashMap, ProtoTag, Tag, ModelItem, ModelItemData};

pub trait ItemData : ModelItemData {
    fn iter_tags<'a>(&self) -> Iter<'a, Arc<dyn ProtoTag + Send + Sync>>;
}

#[derive(Clone, Debug)]
pub struct Item<TD: Debug, ID: Debug + ItemData> {
    pub uuid: Uuid,
    pub data: ID,
    pub tags: DashMap<Uuid, Arc<Tag<TD, ID>>>,
}

impl<TD: Debug, ID: Debug + ItemData> ModelItem for Item<TD, ID> {
    type Data = ID;
    type Tag = Tag<TD, ID>;

    fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn tags_count(&self) -> usize {
        self.tags.len()
    }

    fn each_tag<F: Fn(&Self::Tag) -> bool>(&self, callback: &F) -> bool {
        for kv in self.tags.iter() {
            if callback(kv.value()) {
                return true;
            }
        }
        false
    }

    fn with_tag<O, F: Fn(Option<&Self::Tag>) -> O>(&self, uuid: &Uuid, callback: &F) -> O {
        match self.tags.get(uuid) {
            None => callback(None),
            Some(kv) => callback(Some(kv.value().as_ref())),
        }
    }
}

impl<TD: Debug, ID: Debug + ItemData> Item<TD, ID> {
    pub fn new(uuid: Uuid, data: ID) -> Self {
        Self {
            uuid,
            data,
            tags: DashMap::new(),
        }
    }

    pub fn new_arc(uuid: Uuid, data: ID) -> Arc<Self> {
        Arc::new(Self::new(uuid, data))
    }

    pub fn add_tag(&self, tag: Arc<Tag<TD, ID>>) {
        self.tags.insert(tag.proto.uuid().clone(), tag);
    }
}
