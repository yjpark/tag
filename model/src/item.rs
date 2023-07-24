use std::fmt::Debug;
use crate::prelude::{Uuid, Hash, Tag};

pub trait Item : Debug {
    type Data;
    type Tag: Tag;

    fn uuid(&self) -> &Uuid;
    fn data(&self) -> &Self::Data;
    fn body(&self) -> Option<&Hash> { None }

    fn tags_count(&self) -> usize;
    fn each_tag<F: Fn(&Self::Tag) -> bool>(&self, callback: &F) -> bool;
    fn with_tag<O, F: Fn(Option<&Self::Tag>) -> O>(&self, uuid: &Uuid, callback: &F) -> O;
}