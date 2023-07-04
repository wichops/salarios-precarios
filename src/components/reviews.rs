use crate::components::layout::Layout;
use crate::models::{Place, Review};
use dioxus::prelude::*;

struct Props {
    reviews: Vec<(Review, Place)>,
}

pub fn reviews(reviews: Vec<(Review, Place)>) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                title: "Salarios qleros",
                table {
                    class: "table-auto border-collapse w-full text-sm rounded-xl",
                    thead {
                        tr {
                            th { class: "font-bold text-left p-2", "Lugar" }
                            th { class: "font-bold text-left p-2", "Salario Semanal" }
                            th { class: "font-bold text-left p-2", "Días de trabajo" }
                            th { class: "font-bold text-left p-2", "Duración del turno" }
                        }
                    }
                    tbody {
                        cx.props.reviews.iter().map(|(review, place)| rsx!(
                            tr {
                                td {
                                    class: "p-2 border-b border-slate-100 text-sky-600",
                                    strong {
                                        "{place.name}"
                                    }
                                }
                                td {
                                    class: "p-2 border-b border-slate-100 text-slate-500",
                                    "{review.weekly_salary}"
                                }
                                td {
                                    class: "p-2 border-b border-slate-100 text-slate-500",
                                    "{review.shift_days_count}"
                                }
                                td {
                                    class: "p-2 border-b border-slate-100 text-slate-500",
                                    "{review.shift_duration} hrs"
                                }
                            }
                        ))
                    }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(app, Props { reviews });
    let _ = app.rebuild();

    dioxus_ssr::render(&app)
}
