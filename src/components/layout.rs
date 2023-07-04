#![allow(non_snake_case)]

use crate::components::hero::Hero;
use dioxus::prelude::*;

#[derive(Props)]
pub struct AppLayoutProps<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Layout<'a>(cx: Scope<'a, AppLayoutProps<'a>>) -> Element {
    cx.render(rsx!(
        head {
            title {
                "{cx.props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            link {
                href: "public/output.css",
                rel: "stylesheet",
                title: "style"
            }
        }
        body {
            header {
                class: "w-full mb-5 text-gray-700 bg-white border-b border-gray-900/10 shadow-sm",
                div {
                    class: "container flex justify-between p-6 mx-auto",
                    nav {
                        class: "flex text-base space-x-5",
                        a {
                            href: "#",
                            class: "font-medium hover:text-gray-900",
                            "Home"
                        }
                        a {
                            href: "#",
                            class: "font-medium text-sky-600 hover:text-gray-900",
                            "Sueldos"
                        }
                        a {
                            href: "#",
                            class: "font-medium hover:text-gray-900",
                            "Acerca de"
                        }
                    }

                }
            }
            Hero {
                title: "Salarios qleros",
                subtitle: "Análisis de los salarios del servicio en México"
            }
            main {
                class: "px-20",

                h1 {
                    class: "font-bold mb-4 text-2xl",
                    "Salarios"
                }

                &cx.props.children

            }
        }
    ))
}
