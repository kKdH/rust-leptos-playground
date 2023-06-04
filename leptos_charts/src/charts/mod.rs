use leptos::{component, create_memo, create_rw_signal, IntoView, Memo, ReadSignal, RwSignal, Scope, SignalGet, SignalWith, view};
use nalgebra::{Matrix3, Point2, Rotation2, Vector2};

use crate::{ChartOptions, Extent2};

#[derive(Debug, Clone, PartialEq)]
pub struct BarChartData {
    domain: Vec<String>,
    values: Vec<f32>,
    normalized_values: Vec<f32>,
    min_value: f32,
    max_value: f32,
}

impl BarChartData {

    pub fn new(domain: Vec<String>, values: Vec<f32>) -> Self {
        let (min_value, max_value) = Self::min_and_max(&values);
        let normalized_values = Self::normalize(&values, 0.0, max_value); // TODO: adjust normalization range.
        BarChartData {
            domain,
            values,
            normalized_values,
            min_value,
            max_value,
        }
    }

    pub fn set_domain(&mut self, domain: Vec<String>) {
        self.domain = domain
    }

    pub fn set_values(&mut self, values: Vec<f32>) {
        let (min_value, max_value) = Self::min_and_max(&values);
        self.normalized_values = Self::normalize(&values, 0.0, max_value); // TODO: adjust normalization range.
        self.values = values;
        self.min_value = min_value;
        self.max_value = max_value;
    }

    fn min_and_max(values: &Vec<f32>) -> (f32, f32) {
        values.iter()
            .fold((f32::MAX, f32::MIN), |(min, max), value| (min.min(*value), max.max(*value)))
    }

    fn normalize(values: &Vec<f32>, min: f32, max: f32) -> Vec<f32> {
        values.iter().map(|value| {
            (value - min) / (max - min)
        }).collect::<Vec<f32>>()
    }
}

#[component]
pub fn BarChart(
    cx: Scope,
    options: ReadSignal<ChartOptions>,
    data: ReadSignal<BarChartData>,
) -> impl IntoView {

    let chart_width = move || options.with(|options| options.extent.width);
    let chart_height = move || options.with(|options| options.extent.height);


    let view_matrix = create_memo(cx, move |_| {
        let mut view_matrix = Matrix3::<f32>::new(
            1.0, 0.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, 0.0, 1.0
        );

        // view_matrix *= Rotation2::new(-std::f32::consts::FRAC_PI_4).to_homogeneous();
        view_matrix.append_translation_mut(&Vector2::new(0.0, chart_height()));

        view_matrix
    });

    let bars =  move || {
        data.with(|data| {
            let bar_width = (chart_width() - 10.0 * (data.values.len() - 1) as f32) / data.values.len() as f32;
            data.domain.iter()
                .enumerate()
                .map(|(index, domain)| {
                    let value = data.normalized_values.get(index).cloned().unwrap_or_default();
                    let origin = Vector2::new(index as f32 * (bar_width + 10.0), 0.0);
                    let extent = Extent2::new(bar_width, value * chart_height());
                    view! { cx,
                        <Bar
                            view_matrix=view_matrix
                            origin=origin
                            extent=extent
                        />
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    view! { cx,
        <svg
            width={chart_width}
            height={chart_height}
        >
            <rect x="0" y="0" width={chart_width} height={chart_height} stroke="red" fill="none"></rect>
            <g>
                {bars}
            </g>
        </svg>
    }
}

#[component]
fn Bar(
    cx: Scope,
    view_matrix: Memo<Matrix3<f32>>,
    origin: Vector2<f32>,
    extent: Extent2<f32>,
) -> impl IntoView {

    let d = move || {
        view_matrix.with(|view_matrix| {
            let points = vec![
                Point2::new(0.0, 0.0),
                Point2::new(0.0, extent.height),
                Point2::new(extent.width, extent.height),
                Point2::new(extent.width, 0.0),
            ];
            let commands = vec!["M", "L", "L", "L"];
            let path = points.iter()
                .map(|point| {
                    let point = point + origin;
                    view_matrix.transform_point(&point)
                })
                .zip(commands.iter())
                .map(|(point, command)| format!("{} {} {} ", command, point.x, point.y))
                .collect::<String>();

            path
        })
    };

    view! { cx,
        <path
            d=d
            stroke="red"
            fill="red"
        />
    }
}
