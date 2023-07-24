use std::sync::Arc;
use std::fmt::Debug;
use derive_builder::Builder;

use super::prelude::{Uuid, IndexMap, CoreTag, ProtoTag, ModelTag, Item};

#[derive(Clone, Debug, Builder)]
pub struct Tag<TD: Debug, ID: Debug> {
    pub data: TD,
    pub proto: Arc<dyn ProtoTag + Send + Sync>,
    #[builder(setter(into, strip_option), default)]
    pub parent: Option<Arc<Tag<TD, ID>>>,
    #[builder(default)]
    pub children: IndexMap<Uuid, Arc<Tag<TD, ID>>>,
    #[builder(default)]
    pub items: IndexMap<Uuid, Arc<Item<TD, ID>>>,
}

impl<TD: Debug, ID: Debug> CoreTag for Tag<TD, ID> {
    fn uuid(&self) -> &Uuid {
        self.proto.uuid()
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl<TD: Debug, ID: Debug> ProtoTag for Tag<TD, ID> {
    fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref().map(|x| { x.uuid() })
    }
}

impl<TD: Debug, ID: Debug> ModelTag for Tag<TD, ID> {
    type Data = TD;
    type Item = Item<TD, ID>;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn children_count(&self) -> usize {
        self.children.len()
    }

    fn each_child<F: Fn(&Self) -> bool>(&self, callback: &F) -> bool {
        for child in self.children.values() {
            if callback(child) {
                return true;
            }
        }
        false
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
}
