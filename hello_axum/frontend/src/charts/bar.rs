use std::fmt::Display;

use color_art::Color;
use leptos::*;
use nalgebra::Point2;

use crate::charts::{ColorPallet, Dimension};

pub struct Dataset {
    pub records: Vec<Record>
}

pub struct Record {
    pub value: f32
}

pub struct BarChartOptions {
    pub title: String,
}

#[component]
pub fn BarChart(cx: Scope, data: ReadSignal<Dataset>, options: ReadSignal<BarChartOptions>) -> impl IntoView {

    let width = move || 400.0f32;
    let height = move || 400.0f32;

    let bar_space = 5.0;

    let (signales, views): (Vec<RwSignal<BarData>>, Vec<_>) = data.with(|data| {
        let (min, max) = data.records.iter()
            .map(|record| record.value)
            .fold((data.records[0].value, data.records[0].value), |(min, max), value| {
                let min = if value < min { value } else { min };
                let max = if value > max { value } else { max };
                (min, max)
            });

        let count = data.records.len();

        let bar_width = move || {
            let count = count as f32;
            (width() - bar_space * (count - 1.0)) / count
        };

        data.records.iter()
            .zip(ColorPallet::Material.colors().iter().cycle())
            .enumerate()
            .map(|(index, (record, color))| {
                let value = record.value;
                let ratio = value / max;
                let rect_height = (height() * ratio) as f32;
                let signal = create_rw_signal(cx, BarData {
                    position: Point2::new(
                        index as f32 * (bar_width() + bar_space),
                        height() - rect_height
                    ),
                    dimension: Dimension::new(
                        bar_width(),
                        rect_height,
                    ),
                    fill: Color::from_num(*color).unwrap(),
                    stroke: Color::new(0, 0, 0, 0.0),
                });
                let view = view! { cx, <Bar data=signal.read_only()></Bar> };
                (signal, view)
            }).unzip()
    });

    view! { cx,
        <svg width=width height=height>
            <rect x="0" y="0" width=width height=height fill="none" stroke="black"></rect>
            {views}
        </svg>
    }
}

#[derive(Clone)]
struct BarData {
    position: Point2<f32>,
    dimension: Dimension,
    fill: Color,
    stroke: Color,
}

impl Default for BarData {
    fn default() -> Self {
        BarData {
            position: Point2::default(),
            dimension: Dimension::default(),
            fill: Color::default(),
            stroke: Color::default(),
        }
    }
}

#[component]
fn Bar(cx: Scope, data: ReadSignal<BarData>) -> impl IntoView {
    data.with(|data| {
        let BarData { position, dimension, fill, stroke } = data;
        let hovered = create_rw_signal(cx, false);
        let fill = {
            let fill = Clone::clone(fill);
            move || {
                if hovered.get() {
                    fill.lighten(0.15).hex()
                }
                else {
                    fill.hex()
                }
            }
        };
        let on_mouse_enter = move |_| {
            hovered.set(true);
        };
        let on_mouse_leave = move |_| {
            hovered.set(false);
        };

        view! { cx,
            <rect
                x=position.x
                y=position.y
                width=dimension.width
                height=dimension.height
                fill=fill
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
                on:click=move |_| { log::info!("click") }>
            </rect>
        }
    })
}
