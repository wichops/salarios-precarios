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
                    thead {
                        tr {
                            th { "ID" }
                            th { "Salario Semanal" }
                            th { "Días de trabajo" }
                            th { "Duración del turno" }
                        }
                    }
                    tbody {
                        cx.props.reviews.iter().map(|review| rsx!(
                            tr {
                                td {
                                    strong {
                                        "{review.id}"
                                    }
                                }
                                td {
                                    "{review.weekly_salary}"
                                }
                                td {
                                    "{review.shift_days_count}"
                                }
                                td {
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
