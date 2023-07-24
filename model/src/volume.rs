use snafu::prelude::*;
use async_trait::async_trait;

use crate::prelude::{Uuid, Hash};

#[derive(Debug, Snafu)]
pub enum LoadBodyError {
    #[snafu(display("Not supported: `{}`", hash))]
    NotSupported { hash: Hash },
    #[snafu(display("Not found: `{}`", hash))]
    NotFound { hash: Hash },
    #[snafu(display("IO failed: {} -> {}", info, error))]
    IoFailed { error: std::io::Error, info: String },
}

pub type LoadBodyResult<T> = std::result::Result<T, LoadBodyError>;

#[async_trait]
pub trait Volume {
    type Tag;
    type Data;
    type Item;
    type Body;

    fn uuid(&self) -> &Uuid;
    fn data(&self) -> &Self::Data;
    fn root(&self) -> &Self::Tag;

    fn items_count(&self) -> usize;
    fn get_item(&self, uuid: &Uuid) -> Option<&Self::Item>;
    fn each_item<F: Fn(&Self::Item) -> bool>(&self, callback: &F) -> bool;

    fn load_body(&self, hash: &Hash) -> LoadBodyResult<Self::Body>;
    async fn load_body_async(&self, hash: &Hash) -> LoadBodyResult<Self::Body>;
}