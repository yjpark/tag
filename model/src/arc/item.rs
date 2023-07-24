use std::sync::Arc;
use std::fmt::Debug;
use derive_builder::Builder;

use super::prelude::{Uuid, IndexMap, Tag, ModelItem};

#[derive(Clone, Debug, Builder)]
pub struct Item<TD: Debug, ID: Debug> {
    pub uuid: Uuid,
    pub data: ID,
    #[builder(default)]
    pub tags: IndexMap<Uuid, Arc<Tag<TD, ID>>>,
}

impl<TD: Debug, ID: Debug> ModelItem for Item<TD, ID> {
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
        for tag in self.tags.values() {
            if callback(tag) {
                return true;
            }
        }
        false
    }
}
