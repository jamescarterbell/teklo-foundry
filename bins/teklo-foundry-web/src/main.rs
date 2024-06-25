#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use manganis::*;
use tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/editor")]
    Editor,
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

enum EditableText {
    Editing(RenderableText),
    Rendering(RenderableText),
}

impl EditableText {
    fn set_editing(&mut self) {
        if let Self::Rendering(val) = self {
            let val = std::mem::take(val);
            *self = Self::Editing(val)
        }
    }

    fn set_rendering(&mut self) {
        if let Self::Editing(val) = self {
            let val = std::mem::take(val);
            *self = Self::Rendering(val)
        }
    }

    fn is_editing(&self) -> bool {
        if let Self::Editing(_) = self {
            return true;
        }
        false
    }
}

impl Deref for EditableText {
    type Target = RenderableText;

    fn deref(&self) -> &Self::Target {
        match self {
            EditableText::Editing(val) => val,
            EditableText::Rendering(val) => val,
        }
    }
}

impl DerefMut for EditableText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            EditableText::Editing(val) => val,
            EditableText::Rendering(val) => val,
        }
    }
}

impl Default for EditableText {
    fn default() -> Self {
        Self::Rendering(RenderableText::default())
    }
}

impl Display for EditableText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Editing(text) => text.raw.fmt(f),
            Self::Rendering(text) => text.rendered.fmt(f),
        }
    }
}

#[derive(Default)]
struct RenderableText {
    raw: String,
    rendered: String,
}

impl RenderableText {
    fn render(&mut self, raw: String) {
        self.raw = raw;
        self.rendered = self
            .raw
            .replace(":pow:", "<img src=\"icons/pow.png\" class=\"inline-img\"/>")
            .replace(":int:", "<img src=\"icons/int.png\" class=\"inline-img\"/>")
            .replace(":def:", "<img src=\"icons/def.png\" class=\"inline-img\"/>")
            .replace(":lif:", "<img src=\"icons/lif.png\" class=\"inline-img\"/>")
            .replace(":res:", "<img src=\"icons/res.png\" class=\"inline-img\"/>");
    }
}

#[component]
fn Editor() -> Element {
    const BASE_CARD: manganis::ImageAsset =
        manganis::mg!(image("./assets/frames/generic_empty_frame.png"));

    let mut rules_text = use_signal(EditableText::default);

    rsx! {
        div {
            class: "editor",
            img { src: "{BASE_CARD}" },
            div {
                class: "rules",
                onfocusin: move |_event| {
                    let mut text = rules_text.write();
                    text.set_editing();
                },
                contenteditable: !rules_text.read().is_editing(),
                if !rules_text.read().is_editing() {
                    div {
                        class: "rules-editor",
                        dangerous_inner_html: rules_text.read().to_string(),
                    }
                } else {
                    textarea {
                        onmounted: move |event| { spawn(async move { event.set_focus(true).await.unwrap() }); },
                        onfocusout: move |_event| {rules_text.write().set_rendering()},
                        onchange: move |event| {
                            let mut text = rules_text.write();
                            text.render(event.data.value());
                        },
                        class: "rules-editor",
                        initial_value: "{rules_text}",
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Link {
            to: Route::Editor,
            "Go to editor"
        }
    }
}
