use crate::prelude::{SerdeTag, SetTag, IndexSetTag, MapTag, IndexMapTag};

use crate::{impl_serde_kv_tag, impl_serde_v_tag};

macro_rules! impl_serde_key_tags {
    ($type: ident, $key_type: ident) => {
        impl_serde_kv_tag!($type, $key_type, bool);
        impl_serde_kv_tag!($type, $key_type, u8);
        impl_serde_kv_tag!($type, $key_type, i8);
        impl_serde_kv_tag!($type, $key_type, u16);
        impl_serde_kv_tag!($type, $key_type, i16);
        impl_serde_kv_tag!($type, $key_type, u32);
        impl_serde_kv_tag!($type, $key_type, i32);
        impl_serde_kv_tag!($type, $key_type, u64);
        impl_serde_kv_tag!($type, $key_type, i64);
        impl_serde_kv_tag!($type, $key_type, u128);
        impl_serde_kv_tag!($type, $key_type, i128);
        impl_serde_kv_tag!($type, $key_type, f32);
        impl_serde_kv_tag!($type, $key_type, f64);
        impl_serde_kv_tag!($type, $key_type, String);
    }
}

macro_rules! impl_serde_map_tags {
    ($type: ident) => {
        impl_serde_key_tags!($type, bool);
        impl_serde_key_tags!($type, u8);
        impl_serde_key_tags!($type, i8);
        impl_serde_key_tags!($type, u16);
        impl_serde_key_tags!($type, i16);
        impl_serde_key_tags!($type, u32);
        impl_serde_key_tags!($type, i32);
        impl_serde_key_tags!($type, u64);
        impl_serde_key_tags!($type, i64);
        impl_serde_key_tags!($type, u128);
        impl_serde_key_tags!($type, i128);
        impl_serde_key_tags!($type, String);
    }
}

macro_rules! impl_serde_set_tags {
    ($type: ident) => {
        impl_serde_v_tag!($type, bool);
        impl_serde_v_tag!($type, u8);
        impl_serde_v_tag!($type, i8);
        impl_serde_v_tag!($type, u16);
        impl_serde_v_tag!($type, i16);
        impl_serde_v_tag!($type, u32);
        impl_serde_v_tag!($type, i32);
        impl_serde_v_tag!($type, u64);
        impl_serde_v_tag!($type, i64);
        impl_serde_v_tag!($type, u128);
        impl_serde_v_tag!($type, i128);
        impl_serde_v_tag!($type, String);
    }
}

impl_serde_set_tags!(SetTag);
impl_serde_set_tags!(IndexSetTag);

impl_serde_map_tags!(MapTag);
impl_serde_map_tags!(IndexMapTag);
