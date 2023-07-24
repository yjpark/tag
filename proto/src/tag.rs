use downcast_rs::{DowncastSync, impl_downcast};

use crate::prelude::{Uuid, CoreTag};

pub trait Tag : CoreTag + DowncastSync {
    fn parent(&self) -> Option<Uuid>;
}
impl_downcast!(sync Tag);