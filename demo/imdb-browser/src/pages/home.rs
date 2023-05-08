use dioxus::prelude::*;

use lazy_static::lazy_static;
use fermi::use_read;

use dioxus_daisyui::prelude::*;

use crate::components::*;
use crate::app;

pub fn view(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            "TODO"
        }
    ))
}