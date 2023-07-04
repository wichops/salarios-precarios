#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct HeroProps<'a> {
    title: &'a str,
    subtitle: &'a str,
}

pub fn Hero<'a>(cx: Scope<'a, HeroProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "isolate bg-white px-6 pt-14",
            div {
                class: "mx-auto max-w-2xl py-56",
                div {
                    class: "text-center",
                    h1 {
                        class: "text-5xl font-bold text-sky-600",
                        cx.props.title
                    }
                    p {
                        class: "text-lg leading-8 mt-8 text-gray-700",
                        cx.props.subtitle
                    }
                }
            }
        }
    })
}
