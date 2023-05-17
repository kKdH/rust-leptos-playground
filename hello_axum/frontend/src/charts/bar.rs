use std::fmt::Display;

use leptos::*;

use crate::charts::{ColorPallet, IntoColorCode};

pub struct Dataset {
    pub records: Vec<Record>
}

pub struct Record {
    pub value: f32
}

#[component]
pub fn BarChart(cx: Scope, data: ReadSignal<Dataset>) -> impl IntoView {

    let width = move || 300.0f32;
    let height = move || 300.0f32;
    let view_box = move || format!("{} {} {} {}", 0, 0, width(), height());

    let bars = move || {
        data.with(|data| {
            let (min, max) = data.records.iter()
                .map(|record| record.value)
                .fold((data.records[0].value, data.records[0].value), |(min, max), value| {
                    let min = if value < min { value } else { min };
                    let max = if value > max { value } else { max };
                    (min, max)
                });

            let count = data.records.len();
            let rect_width = move || (width() / count as f32) as f32;

            data.records.iter()
                .zip(ColorPallet::Material.colors().iter().cycle())
                .enumerate()
                .map(|(index, (record, color))| {
                    let value = record.value;
                    let ratio = value / max;
                    let rect_height = move || (height() * ratio) as f32;
                    let rect_x = move || index as f32 * rect_width();
                    let rect_y = move || height() - rect_height();
                    let rect_fill = move || color.into_color_code();

                    view! { cx,
                        <rect x=rect_x y=rect_y width=rect_width height=rect_height fill=rect_fill></rect>
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    view! { cx,
        <svg width=width height=height viewBox=view_box>
            {bars}
        </svg>
    }
}
