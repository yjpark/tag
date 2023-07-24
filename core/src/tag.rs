use std::fmt::Debug;

pub use uuid::Uuid;

pub trait Tag : Debug {
    fn uuid(&self) -> &Uuid;
    fn has_parent(&self) -> bool;
}