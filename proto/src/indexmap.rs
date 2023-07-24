use crate::prelude::ValTag;

pub use indexmap::{IndexSet, IndexMap};

pub type IndexSetTag<V> = ValTag<IndexSet<V>>;
pub type IndexMapTag<K, V> = ValTag<IndexMap<K, V>>;

