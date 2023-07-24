use crate::prelude::{Uuid, CoreTag};

pub trait Tag : CoreTag {
    fn parent(&self) -> Option<&Uuid>;
}