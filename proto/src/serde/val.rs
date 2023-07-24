use crate::prelude::{SerdeTag, ValTag, VecTag};

use crate::impl_serde_v_tag;

macro_rules! impl_serde_value_tags {
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
        impl_serde_v_tag!($type, f32);
        impl_serde_v_tag!($type, f64);
        impl_serde_v_tag!($type, String);
    }
}

impl_serde_value_tags!(ValTag);
impl_serde_value_tags!(VecTag);
