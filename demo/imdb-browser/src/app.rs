#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::prelude::*;
use lazy_static::lazy_static;
use simsearch::SimSearch;

use crate::pages::home;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", home::view {} }
        }
    })
}