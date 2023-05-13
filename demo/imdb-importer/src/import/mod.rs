pub mod utils;
pub mod title;
pub mod name;
pub mod principal;

pub mod table {
    pub const TITLE: &'static str = "title";
    pub const NAME: &'static str = "name";
}

pub mod relation {
    pub const KNOWN_FOR: &'static str = "known_for";
    pub const HAS_PRINCIPAL: &'static str = "has_principal";
}
