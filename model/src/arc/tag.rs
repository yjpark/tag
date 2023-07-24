use std::sync::Arc;
use std::fmt::Debug;

use super::prelude::{Uuid, IndexMap, CoreTag, ProtoTag, ModelTag, Item, ItemData};

#[derive(Clone, Debug)]
pub struct Tag<TD: Debug, ID: Debug + ItemData> {
    pub proto: Arc<dyn ProtoTag + Send + Sync>,
    pub data: TD,
    pub parent: Option<Arc<Tag<TD, ID>>>,
    pub children: IndexMap<Uuid, Arc<Tag<TD, ID>>>,
    pub items: IndexMap<Uuid, Arc<Item<TD, ID>>>,
}

impl<TD: Debug, ID: Debug + ItemData> CoreTag for Tag<TD, ID> {
    fn uuid(&self) -> &Uuid {
        self.proto.uuid()
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl<TD: Debug, ID: Debug + ItemData> ProtoTag for Tag<TD, ID> {
    fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref().map(|x| { x.uuid() })
    }
}

impl<TD: Debug, ID: Debug + ItemData> ModelTag for Tag<TD, ID> {
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

impl<TD: Debug, ID: Debug + ItemData> Tag<TD, ID> {
    pub fn new(proto: Arc<dyn ProtoTag + Send + Sync>, data: TD, parent: Option<Arc<Self>>) -> Self {
        Self {
            proto,
            data,
            parent: parent,
            children: IndexMap::new(),
            items: IndexMap::new(),
        }
    }

    pub fn new_arc(proto: Arc<dyn ProtoTag + Send + Sync>, data: TD, parent: Option<Arc<Self>>) -> Arc<Self> {
        Arc::new(Self::new(proto, data, parent))
    }

    pub fn root_arc(uuid: Uuid, data: TD) -> Arc<Self> {
        let proto = Arc::new(<dyn ProtoTag>::root(uuid.clone()));
        Self::new_arc(proto, data, None)
    }

    pub fn add_child(arc_self: &mut Arc<Self>,
        child_proto: Arc<dyn ProtoTag + Send + Sync>,
        child_data: TD,
    ) {
        let child = Self::new_arc(child_proto,  child_data, Some(arc_self.clone()));
        arc_self.children.insert(child.uuid().clone(), child);
    }
}
