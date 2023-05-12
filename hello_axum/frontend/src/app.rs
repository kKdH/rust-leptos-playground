use leptos::*;
use leptos::ev::MouseEvent;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Clone, Debug)]
pub enum FetchError {
    #[error("Error loading data from serving.")]
    Request,
    #[error("Error deserializaing cat data from request.")]
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

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    let (value, set_value) = create_signal(cx, 0i32);
    let count = create_action(cx, |_| async move {
        fetch_counter().await.unwrap_or(Counter { value: 0 })
    });

    let fetch_callback = move |_: MouseEvent| {
        set_value.update(|value| *value += 1);
        count.dispatch(());
    };

    let reset_callback = move |_: MouseEvent| {
        set_value.update(|value| *value = 0);
    };

    view! { cx,
        <h1>"Hello Leptos"</h1>
        <button on:click=fetch_callback>"Fetch"</button>
        <button on:click=reset_callback>"Reset"</button>
        <p>"Counter: " { move || format!("{}", count.value().get().map(|counter| counter.value).unwrap_or(0)) }</p>
    }
}
