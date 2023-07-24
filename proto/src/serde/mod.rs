use tag_core::prelude::Tag;

pub mod val;
pub mod map;

#[typetag::serde(tag = "type")]
pub trait SerdeTag : Tag {
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SerdeTags(Vec<Box<dyn SerdeTag>>);

// typetag doesn't support generic impl yet
// https://github.com/dtolnay/typetag/issues/1
#[macro_export]
macro_rules! impl_serde_v_tag {
    ($type: ident, $value_type: ident) => {
        #[typetag::serde]
        impl SerdeTag for $type<$value_type> {
        }
    }
}

#[macro_export]
macro_rules! impl_serde_kv_tag {
    ($type: ident, $key_type: ident, $value_type: ident) => {
        #[typetag::serde]
        impl SerdeTag for $type<$key_type, $value_type> {
        }
    }
}