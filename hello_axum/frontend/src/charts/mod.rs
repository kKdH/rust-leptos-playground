use core::f32::consts::PI;
use std::fmt::Display;
use leptos::*;

#[component]
pub fn PieChart(cx: Scope) -> impl IntoView {

    let data = vec![10, 25, 25, 10, 10, 40, 100, 75];
    let sum = data.iter().sum::<i32>() as f32;

    let selected: RwSignal<Option<f32>> = create_rw_signal(cx, None);
    let text: RwSignal<Option<String>> = create_rw_signal(cx, None);
    let subtext: RwSignal<Option<String>> = create_rw_signal(cx, None);

    let radius = 40f32;

    let (mut wedges, _, _, _) = data.iter()
        .fold((Vec::new(), 0f32, -radius, 0f32), |(mut result, x0, y0, s), value| {
            let value = *value as f32;
            let percentage = (value * 100.0) / sum;
            let (x1, y1) = compute_coordinates((value + s) / sum, radius);
            let (data, _) = create_signal(cx, WedgeData {
                value,
                outer_radius: radius,
                inner_radius: radius * 0.5,
                x0, y0,
                x1, y1,
                x2: x1 * 0.5,
                y2: y1 * 0.5,
                x3: x0 * 0.5,
                y3: y0 * 0.5,
            });
            let selected = create_rw_signal(cx, false);
            result.push(view! { cx,
                <Wedge
                    data={data}
                    selected={selected.read_only()}
                    on_mouse_enter=move || {
                        text.set(Some(value.to_string()));
                        subtext.set(Some(format!("{:.2}%", percentage)));
                        selected.set(true);
                    }
                    on_mouse_exit=move || {
                        text.set(None);
                        subtext.set(None);
                        selected.set(false);
                    }>
                </Wedge>
            });
            (result, x1, y1, s + value)
        });

    view! { cx,
        <svg width="400" height="400" viewBox="-50 -50 100 100">
            {wedges}
            <text text-anchor="middle" alignment-baseline="central" style="font-family: sans-serif;">
                <tspan x="0" dy="0em" style="font-size: 12px;">{move || text.get()}</tspan>
                <tspan x="0" dy="1.5em" style="font-size: 5px;">{move || subtext.get()}</tspan>
            </text>
        </svg>
    }
}

#[derive(Clone, PartialEq)]
struct WedgeData<A>
where A: Clone + Display + 'static {
    value: A,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    outer_radius: f32,
    inner_radius: f32,
}

#[component]
fn Wedge<A, B, C>(
    cx: Scope,
    data: ReadSignal<WedgeData<A>>,
    selected: ReadSignal<bool>,
    on_mouse_enter: B,
    on_mouse_exit: C,
) -> impl IntoView
where A: Clone + Display + 'static,
      B: Fn() -> () + 'static,
      C: Fn() -> () + 'static {
    let WedgeData { value, x0, y0, x1, y1, x2, y2, x3, y3, outer_radius: r1, inner_radius: r2} = data.get();
    let fill = move || {
        if selected.get() {
            "white"
        }
        else {
            "red"
        }
    };
    view! { cx,
        <path
            on:mouseenter=move |_| on_mouse_enter()
            on:mouseleave=move |_| on_mouse_exit()
            fill=move || fill()
            stroke="black"
            d={format!("\
                M {x0} {y0} \
                A {r1} {r1} 0 0 1 {x1} {y1} \
                L {x2} {y2} \
                A {r2} {r2} 0 0 0 {x3} {y3}"
            )}>
        </path>
    }
}

fn compute_coordinates(percent: f32, radius: f32) -> (f32, f32) {
    let x = (2.0 * PI * (percent - 0.25)).cos() * radius;
    let y = (2.0 * PI * (percent - 0.25)).sin() * radius;

    (x, y)
}
