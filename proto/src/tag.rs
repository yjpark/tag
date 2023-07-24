use downcast_rs::impl_downcast;
#[cfg(feature = "arc")]
use downcast_rs::DowncastSync;
#[cfg(not(feature = "arc"))]
use downcast_rs::Downcast;

use crate::prelude::{Uuid, CoreTag};

#[cfg(feature = "arc")]
pub trait Tag : CoreTag + DowncastSync {
    fn parent(&self) -> Option<Uuid>;
}
#[cfg(feature = "arc")]
impl_downcast!(sync Tag);

#[cfg(not(feature = "arc"))]
pub trait Tag : CoreTag + Downcast {
    fn parent(&self) -> Option<Uuid>;
}
#[cfg(not(feature = "arc"))]
impl_downcast!(Tag);
