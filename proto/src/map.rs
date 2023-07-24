use crate::prelude::ValTag;

use std::collections::{HashSet, HashMap};

pub type SetTag<V> = ValTag<HashSet<V>>;
pub type MapTag<K, V> = ValTag<HashMap<K, V>>;
