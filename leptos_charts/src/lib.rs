mod charts;

use std::fmt::Debug;
pub use charts::{BarChart, BarChartProps, BarChartPropsBuilder, BarChartData};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Extent2<A>
where A: PartialEq + PartialOrd + Clone + Debug {
    pub width: A,
    pub height: A,
}

impl <A> Extent2<A>
where A: PartialEq + PartialOrd + Clone + Debug {
    pub fn new(width: A, height: A) -> Self {
        Extent2 { width, height }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartOptions {
    pub extent: Extent2<f32>,
}
