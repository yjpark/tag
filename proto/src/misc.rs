use crate::prelude::{Uuid, Tag, ValTag};

impl dyn Tag {
    pub fn root(uuid: Uuid) -> impl Tag {
        ValTag::<()> {
            uuid,
            parent: None,
            val: ()
        }
    }
}