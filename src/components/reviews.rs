use crate::components::layout::Layout;
use crate::models::Review;
use dioxus::prelude::*;

struct Props {
    reviews: Vec<Review>,
}

// Take a Vec<User> and create an HTML table.
pub fn reviews(reviews: Vec<Review>) -> String {
    // Inner function to create our rsx! component
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {    // <-- Use our layout
                title: "Reviews Table",
                table {
                    class: "table-auto border-collapse w-full text-sm rounded-xl",
                    thead {
                        tr {
                            th { class: "font-medium text-left p-2", "ID" }
                            th { class: "font-medium text-left p-2", "Salario Semanal" }
                            th { class: "font-medium text-left p-2", "Días de trabajo" }
                            th { class: "font-medium text-left p-2", "Duración del turno" }
                        }
                    }
                    tbody {
                        cx.props.reviews.iter().map(|review| rsx!(
                            tr {
                                td {
                                    class: "p-2 border-b border-slate-100 text-slate-500",
                                    strong {
                                        "{review.id}"
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
