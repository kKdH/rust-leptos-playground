use leptos::*;
use leptos_router::*;
use leptos::ev::MouseEvent;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use leptos_charts::{BarChart, BarChartData, ChartOptions, Extent2};
use std::str::FromStr;

use crate::charts::{ColorPallet, Dataset, PieChart, PieChartData, PieChartItem, Record};

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
            value: 500.0,
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
                value: 2.0
            },
            Record {
                value: 6.0
            },
            Record {
                value: 1.6
            },
            Record {
                value: 3.0
            },
            Record {
                value: 5.0
            },
            Record {
                value: 3.0
            },
        ]
    });

    let bar_chart_options = create_rw_signal(cx, ChartOptions {
        extent: Extent2::new(500_f32, 500_f32),
    });

    let bar_chart_data = create_rw_signal(cx, BarChartData::new(
        vec![String::from("A"), String::from("B"), String::from("C"), String::from("D")],
        vec![8.0, 3.5, 10.0, 5.0]
    ));

    // let bar_chart_options = create_rw_signal(cx, BarChartOptions {
    //     title: String::from("Historical revenue")
    // });

    view! { cx,
        // <h1>"Hello Leptos"</h1>
        // <button on:click=fetch_callback>"Fetch"</button>
        // <button on:click=inc_callback>"Inc"</button>
        // <button on:click=reset_callback>"Reset"</button>
        // <p>"Counter: " { move || format!("{}", count.value().get().map(|counter| counter.value).unwrap_or(0)) }</p>
        <div style="margin-bottom: 20px;">
            <a href="/piechart">"PieChart"</a>" | "<a href="/barchart">"BarChart"</a>
        </div>
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
                            <div>"Width: "<input type="range" min="0" max="1000" value="500" on:input=move |event| {
                                let value = event_target_value(&event);
                                bar_chart_options.update(|options| {
                                    options.extent.width = f32::from_str(&value).unwrap();
                                });
                            }/></div>
                            <div>"Height:"<input type="range" min="0" max="800" value="400" on:input=move |event| {
                                let value = event_target_value(&event);
                                bar_chart_options.update(|options| {
                                    options.extent.height = f32::from_str(&value).unwrap();
                                });
                            }/></div>
                            <div>"Values: "<input type="text" value="8.0, 3.5, 10.0, 5.0" on:input=move |event| {
                                let result = event_target_value(&event)
                                    .split(",")
                                    .map(|value| {
                                        f32::from_str(value.trim())
                                    })
                                    .collect::<Result<Vec<f32>, _>>();
                                if let Ok(values) = result {
                                    bar_chart_data.update(|data| {
                                        data.set_domain((0..values.len()).map(|value| value.to_string()).collect());
                                        data.set_values(values);
                                    });
                                }
                            }/></div>
                            <br/>
                            <BarChart options=bar_chart_options.read_only() data=bar_chart_data.read_only() />
                        }
                />
            </Routes>
        </Router>
    }
}
