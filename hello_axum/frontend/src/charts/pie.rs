
use core::f32::consts::PI;
use std::fmt::Display;

use leptos::*;
use crate::charts::{ColorPallet, Degree, IntoColorCode};

#[derive(Clone)]
pub struct PieChartData {
    pub items: Vec<PieChartItem>,
    pub caption: String,
    pub color_pallet: ColorPallet,
}

#[derive(Clone)]
pub struct PieChartItem {
    pub name: String,
    pub value: f32,
}

#[component]
pub fn PieChart(cx: Scope, data: ReadSignal<PieChartData>) -> impl IntoView {

    let width = move || 300.0;
    let height = move || 300.0;
    let view_box = move || format!("{} {} {} {}", width() * -0.5, height() * -0.5, width(), height());

    let caption = move || data.get().caption;
    let caption_y = move || width() * 0.45;

    let selected_item_value: RwSignal<Option<String>> = create_rw_signal(cx, None);
    let selected_item_percentage: RwSignal<Option<String>> = create_rw_signal(cx, None);
    let selected_item_text: RwSignal<Option<String>> = create_rw_signal(cx, None);

    let wedges = move || {

        data.with(move |chart_data| {

            let sum = chart_data.items.iter().map(|item| item.value).sum::<f32>() as f32;

            let radius = width() * 0.4;

            let (wedges, _, _, _) = chart_data.items.iter()
                .map(|item| (item.value, (item.value * 100.0) / sum, Clone::clone(&item.name)))
                .zip(chart_data.color_pallet.colors().iter().cycle())
                .fold((Vec::new(), 0f32, -radius, 0f32), move |(mut result, x0, y0, s), ((value, percentage, name), fill)| {
                    let (x1, y1) = compute_coordinates((value + s) / sum, radius);
                    let (wedge_data, _) = create_signal(cx, WedgeData {
                        value,
                        percentage,
                        outer_radius: radius,
                        inner_radius: radius * 0.5,
                        x0, y0,
                        x1, y1,
                        x2: x1 * 0.5,
                        y2: y1 * 0.5,
                        x3: x0 * 0.5,
                        y3: y0 * 0.5,
                        color: *fill
                    });
                    let selected = create_rw_signal(cx, false);
                    result.push(view! { cx,
                        <Wedge
                            data={wedge_data}
                            selected={selected.read_only()}
                            on_mouse_enter=move || {
                                selected_item_value.set(Some(format!("{}", value)));
                                selected_item_percentage.set(Some(format!("{:.2}%", percentage)));
                                selected_item_text.set(Some(Clone::clone(&name)));
                                selected.set(true);
                            }
                            on_mouse_exit=move || {
                                selected_item_value.set(None);
                                selected_item_percentage.set(None);
                                selected_item_text.set(None);
                                selected.set(false);
                            }>
                        </Wedge>
                    });
                    (result, x1, y1, s + value)
                });

            wedges
        })
    };

    view! { cx,
        <svg width=width height=height viewBox=view_box>
            {wedges}
            <text text-anchor="middle" alignment-baseline="central" style="font-family: sans-serif;">
                <tspan x="0" dy="0em" font-size="16pt" font-weight="bold">{move || selected_item_value.get()}</tspan>
                <tspan x="0" dy="1.75em" font-size="8pt">{move || selected_item_percentage.get()}</tspan>
            </text>
            <text y=caption_y text-anchor="middle" alignment-baseline="central" font-family="sans-serif" font-size="10pt" font-weight="bold">{move || caption()}</text>
        </svg>
    }
}

#[derive(Clone, PartialEq)]
struct WedgeData<A>
    where A: Clone + Display + 'static {
    value: A,
    percentage: f32,
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
    color: u32,
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

    let WedgeData { value, percentage, x0, y0, x1, y1, x2, y2, x3, y3, outer_radius: r1, inner_radius: r2, color} = data.get();

    let path_commands = move || {
        const SCALE: f32 = 1.075;
        let (x0, y0, x1, y1, r1) = if selected.get() {
            (x0 * SCALE, y0 * SCALE, x1 * SCALE, y1 * SCALE, r1 * SCALE)
        }
        else {
            (x0, y0, x1, y1, r1)
        };

        let large_arc = if percentage <= 50.0 {
            String::from("0")
        }
        else {
            String::from("1")
        };

        format!("\
            M {x0} {y0} \
            A {r1} {r1} 0 {large_arc} 1 {x1} {y1} \
            L {x2} {y2} \
            A {r2} {r2} 0 {large_arc} 0 {x3} {y3}"
        )
    };

    view! { cx,
        <path
            on:mouseenter=move |_| on_mouse_enter()
            on:mouseleave=move |_| on_mouse_exit()
            fill=move || color.into_color_code()
            stroke="none"
            d=path_commands>
        </path>
    }
}

fn compute_coordinates(percent: f32, radius: f32) -> (f32, f32) {
    let x = (2.0 * PI * (percent - 0.25)).cos() * radius;
    let y = (2.0 * PI * (percent - 0.25)).sin() * radius;

    (x, y)
}
