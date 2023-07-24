pub use blake3;

pub use tag_proto;
pub use tag_proto::tag_core;

pub mod item;
pub mod tag;
pub mod volume;

pub mod arc;

pub mod prelude {
    #[doc(hidden)]
    pub use blake3::Hash;

    #[doc(hidden)]
    pub use tag_proto::prelude::{*, Tag as ProtoTag};

    #[doc(hidden)]
    pub use crate::item::Item;

    #[doc(hidden)]
    pub use crate::tag::Tag;

    #[doc(hidden)]
    pub use crate::volume::{Volume, LoadBodyError, LoadBodyResult};
}