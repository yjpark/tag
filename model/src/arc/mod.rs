pub mod tag;
pub mod item;
pub mod volume;
pub mod loader;

pub use dashmap;

pub mod prelude {
    #[doc(hidden)]
    pub use dashmap::DashMap;

    #[doc(hidden)]
    pub use crate::prelude::{*,
        Tag as ModelTag,
        Item as ModelItem,
        ItemData as ModelItemData,
        Volume as ModelVolume,
    };

    #[doc(hidden)]
    pub use super::tag::Tag;

    #[doc(hidden)]
    pub use super::item::{Item, ItemData};

    #[doc(hidden)]
    pub use super::volume::Volume;

    #[doc(hidden)]
    pub use super::loader::load_volume;
}