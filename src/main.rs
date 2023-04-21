use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <button
                on:click=move |_| {
                    set_count.update(|counter| *counter += 1);
                }
                class:red=move || count() % 2 == 1
            >
                "Click me: " { count }
            </button>
            <button on:click=move |_| { set_count(0); }>"Reset"</button>
        </div>
        <div>
            <progress max="10" value=count />
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
