
pub use tag_core;

pub mod tag;

pub mod val;
pub mod vec;
pub mod map;

pub mod indexmap;

pub mod tags;

#[cfg(feature = "serde")]
pub mod serde;


pub mod prelude {
    #[doc(hidden)]
    pub use tag_core::prelude::{*, Tag as CoreTag};

    #[doc(hidden)]
    pub use crate::tag::Tag;

    #[doc(hidden)]
    pub use crate::val::ValTag;

    #[doc(hidden)]
    pub use crate::vec::VecTag;

    #[doc(hidden)]
    pub use crate::map::{SetTag, MapTag};

    #[doc(hidden)]
    pub use crate::indexmap::{IndexSet, IndexMap, IndexSetTag, IndexMapTag};

    #[doc(hidden)]
    pub use crate::tags::Tags;

    #[cfg(feature = "serde")]
    #[doc(hidden)]
    pub use crate::serde::*;
}
