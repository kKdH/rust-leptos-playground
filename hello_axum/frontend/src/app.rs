use leptos::*;
use leptos_router::*;
use leptos::ev::MouseEvent;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

use crate::charts::{BarChart, ColorPallet, Dataset, PieChart, PieChartData, PieChartItem, Record};

#[derive(thiserror::Error, Clone, Debug)]
pub enum FetchError {
    #[error("Error loading data from serving.")]
    Request,
    #[error("Error deserializing cat data from request.")]
    Json,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Counter {
    value: i32
}

async fn fetch_counter() -> Result<Counter, FetchError> {
    Request::get(&format!("/api/plant"))
        .send()
        .await
        .map_err(|_| FetchError::Request)?
        .json::<Counter>()
        .await
        .map_err(|_| FetchError::Json)
}

async fn inc_counter() -> Result<(), FetchError> {
    Request::post(&format!("/api/plant"))
        .send()
        .await
        .map_err(|_| FetchError::Request)
        .map(|_| ())
}

async fn reset_counter() -> Result<(), FetchError> {
    Request::delete(&format!("/api/plant"))
        .send()
        .await
        .map_err(|_| FetchError::Request)
        .map(|_| ())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    create_signal(cx, "");
    let count = create_action(cx, |_| async move {
        fetch_counter().await.unwrap_or(Counter { value: 0 })
    });

    let fetch_callback = move |_: MouseEvent| {
        spawn_local(async move { fetch_counter().await.unwrap(); });
        count.dispatch(());
    };

    let inc_callback = move |_: MouseEvent| {
        spawn_local(async move { inc_counter().await.unwrap(); });
        count.dispatch(());
    };

    let reset_callback = move |_: MouseEvent| {
        spawn_local(async move { reset_counter().await.unwrap(); });
        count.dispatch(());
    };

    let items = vec![
        PieChartItem {
            name: String::from("A"),
            value: 50.0,
        },
        PieChartItem {
            name: String::from("B"),
            value: 30.0,
        },
        PieChartItem {
            name: String::from("C"),
            value: 80.0,
        },
        PieChartItem {
            name: String::from("C"),
            value: 10.0,
        },
        PieChartItem {
            name: String::from("C"),
            value: 35.0,
        },
        PieChartItem {
            name: String::from("C"),
            value: 50.0,
        },
        PieChartItem {
            name: String::from("C"),
            value: 78.0,
        }
    ];

    let pie_chart_data_1 = create_rw_signal(cx, PieChartData {
        items: Clone::clone(&items),
        caption: String::from("Examples PieChart (Material)"),
        color_pallet: ColorPallet::Material,
    });

    let pie_chart_data_2 = create_rw_signal(cx, PieChartData {
        items: Clone::clone(&items),
        caption: String::from("Examples PieChart (OrangeFire)"),
        color_pallet: ColorPallet::OrangeFire,
    });

    let pie_chart_data_3 = create_rw_signal(cx, PieChartData {
        items: Clone::clone(&items),
        caption: String::from("Examples PieChart (DarkBlue)"),
        color_pallet: ColorPallet::DarkBlue,
    });

    let pie_chart_data_4 = create_rw_signal(cx, PieChartData {
        items: Clone::clone(&items),
        caption: String::from("Examples PieChart (Default)"),
        color_pallet: ColorPallet::Default,
    });

    let dataset = create_rw_signal(cx, Dataset {
        records: vec![
            Record {
                value: 5.0
            },
            Record {
                value: 10.0
            },
            Record {
                value: 4.6
            },
            Record {
                value: 1.0
            },
            Record {
                value: 2.0
            },
        ]
    });

    view! { cx,
        // <h1>"Hello Leptos"</h1>
        // <button on:click=fetch_callback>"Fetch"</button>
        // <button on:click=inc_callback>"Inc"</button>
        // <button on:click=reset_callback>"Reset"</button>
        // <p>"Counter: " { move || format!("{}", count.value().get().map(|counter| counter.value).unwrap_or(0)) }</p>
        <Router>
            <Routes>
                <Route
                    path="/piechart"
                    view=move |cx| view! { cx,
                            <PieChart data=pie_chart_data_1.read_only() />
                            <PieChart data=pie_chart_data_2.read_only() />
                            <PieChart data=pie_chart_data_3.read_only() />
                            <PieChart data=pie_chart_data_4.read_only() />
                        }
                />
                <Route
                    path="/barchart"
                    view=move |cx| view! { cx,
                            <BarChart data=dataset.read_only() />
                        }
                />
            </Routes>
        </Router>
    }
}
