use std::fmt::Debug;
use crate::prelude::{Uuid, Hash, Tag, ProtoTag};

pub trait Item : Debug {
    type Data: ItemData;
    type Tag: Tag;

    fn uuid(&self) -> &Uuid;
    fn data(&self) -> &Self::Data;
    fn body(&self) -> Option<&Hash> { None }

    fn tags_count(&self) -> usize;
    fn each_tag<F: Fn(&Self::Tag) -> bool>(&self, callback: &F) -> bool;
}

pub trait ItemData {
    fn tags_count(&self) -> usize;
    fn each_tag<F: Fn(&dyn ProtoTag) -> bool>(&self, callback: &F) -> bool;
}