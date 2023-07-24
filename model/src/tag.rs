use crate::prelude::{Uuid, Hash, ProtoTag, Item};

pub trait Tag : ProtoTag {
    type Data;
    type Item: Item;

    fn data(&self) -> &Self::Data;
    fn body(&self) -> Option<&Hash> { None }

    fn children_count(&self) -> usize;
    fn each_child<F: Fn(&Self) -> bool>(&self, callback: &F) -> bool;
    fn with_child<O, F: Fn(Option<&Self>) -> O>(&self, uuid: &Uuid, callback: &F) -> O;

    fn items_count(&self) -> usize;
    fn each_item<F: Fn(&Self::Item) -> bool>(&self, callback: &F) -> bool;
    fn with_item<O, F: Fn(Option<&Self::Item>) -> O>(&self, uuid: &Uuid, callback: &F) -> O;

    fn each_child_deep<F: Fn(&Self) -> bool>(&self, callback: &F) -> bool {
        self.each_child(&|child| {
            if callback(child) {
                return true;
            }
            if child.each_child_deep(callback) {
                return true;
            }
            false
        })
    }
    fn each_item_deep<F: Fn(&Self::Item) -> bool>(&self, callback: &F) -> bool {
        if self.each_item(callback) {
            return true;
        }
        self.each_child(&|child| {
            child.each_item_deep(callback)
        })
    }
}