use std::fmt::Debug;
use derive_builder::Builder;

use crate::prelude::{Uuid, CoreTag, Tag};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ValTag<V> {
    pub uuid: Uuid,
    #[builder(setter(into, strip_option), default)]
    pub parent: Option<Uuid>,
    pub val: V,
}

impl<V> CoreTag for ValTag<V>
    where V: Debug
{
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl<V> Tag for ValTag<V>
    where V: Debug
{
    fn parent(&self) -> Option<Uuid> {
        self.parent.clone()
    }
}
