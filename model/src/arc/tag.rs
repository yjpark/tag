use std::sync::{Arc, Weak};
use std::fmt::Debug;

use super::prelude::{Uuid, DashMap, CoreTag, ProtoTag, ModelTag, Item};

#[derive(Clone, Debug)]
pub struct Tag<TD: Debug, ID: Debug > {
    pub proto: Arc<dyn ProtoTag + Send + Sync>,
    pub data: TD,
    pub parent: Option<Weak<Tag<TD, ID>>>,
    pub children: DashMap<Uuid, Arc<Tag<TD, ID>>>,
    pub items: DashMap<Uuid, Arc<Item<TD, ID>>>,
}

impl<TD: Debug, ID: Debug > CoreTag for Tag<TD, ID> {
    fn uuid(&self) -> &Uuid {
        self.proto.uuid()
    }

    fn has_parent(&self) -> bool {
        self.parent.as_ref().and_then(|x| x.upgrade()).is_some()
    }
}

impl<TD, ID > ProtoTag for Tag<TD, ID>
    where
        TD: Debug + Send + Sync + 'static,
        ID: Debug + Send + Sync + 'static,
{
    fn parent(&self) -> Option<Uuid> {
        self.parent.as_ref()
            .and_then(|x| x.upgrade())
            .map(|x| x.uuid().clone())
    }
}

impl<TD, ID> ModelTag for Tag<TD, ID>
    where
        TD: Debug + Send + Sync + 'static,
        ID: Debug + Send + Sync + 'static,
{
    type Data = TD;
    type Item = Item<TD, ID>;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn children_count(&self) -> usize {
        self.children.len()
    }

    fn each_child<F: Fn(&Self) -> bool>(&self, callback: &F) -> bool {
        for kv in self.children.iter() {
            if callback(kv.value()) {
                return true;
            }
        }
        false
    }
    fn with_child<O, F: Fn(Option<&Self>) -> O>(&self, uuid: &Uuid, callback: &F) -> O {
        match self.children.get(uuid) {
            None => callback(None),
            Some(kv) => callback(Some(kv.value().as_ref())),
        }
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
}

impl<TD: Debug, ID: Debug > Tag<TD, ID> {
    pub fn new(proto: Arc<dyn ProtoTag + Send + Sync>, data: TD, parent: Option<Weak<Self>>) -> Self {
        Self {
            proto,
            data,
            parent: parent,
            children: DashMap::new(),
            items: DashMap::new(),
        }
    }

    pub fn new_arc(proto: Arc<dyn ProtoTag + Send + Sync>, data: TD, parent: Option<Weak<Self>>) -> Arc<Self> {
        Arc::new(Self::new(proto, data, parent))
    }

    pub fn new_child(arc_self: &Arc<Self>,
        child_proto: Arc<dyn ProtoTag + Send + Sync>,
        child_data: TD,
    ) -> Arc<Self> {
        let child = Self::new_arc(child_proto,  child_data, Some(Arc::downgrade(arc_self)));
        arc_self.add_child(child.clone());
        child
    }

    pub fn add_child(&self, child: Arc<Self>) {
        self.children.insert(child.uuid().clone(), child);
    }

    pub fn get_child(&self, uuid: &Uuid) -> Option<Arc<Self>> {
        self.children.get(uuid).map(|x| x.value().clone() )
    }

    pub fn get_child_deep(&self, uuid: &Uuid) -> Option<Arc<Self>> {
        if let Some(child) = self.get_child(uuid) {
            return Some(child);
        }
        for kv in self.children.iter() {
            if let Some(child) = kv.value().get_child_deep(uuid) {
                return Some(child);
            }
        }
        None
    }

    pub fn new_item(arc_self: &Arc<Self>,
        item_uuid: Uuid,
        item_data: ID,
    ) -> Arc<Item<TD, ID>> {
        let item = Item::<TD, ID>::new_arc(item_uuid, item_data);
        item.add_tag(arc_self.clone());
        arc_self.add_item(item.clone());
        item
    }

    pub fn add_item(&self, item: Arc<Item<TD, ID>>) {
        self.items.insert(item.uuid.clone(), item);
    }

    pub fn get_item(&self, uuid: &Uuid) -> Option<Arc<Item<TD, ID>>> {
        self.items.get(uuid).map(|x| x.value().clone() )
    }

    pub fn get_item_deep(&self, uuid: &Uuid) -> Option<Arc<Item<TD, ID>>> {
        if let Some(child) = self.get_item(uuid) {
            return Some(child);
        }
        for kv in self.children.iter() {
            if let Some(child) = kv.value().get_item_deep(uuid) {
                return Some(child);
            }
        }
        None
    }
}
