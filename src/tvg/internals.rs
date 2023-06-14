use allen_interval_algebra::interval::Interval;
use log::error;
use crate::tvg::internals::IntervalTvgEdge::{BaseEdge, DataEdge};

/// Enum for the time-varying graph edge types. `BaseEdge` is for simple edges compromised from only
/// an interval, while `DataEdge` should be used in cases where there is also some data associated
/// with an interval. For example data sent over time.
#[derive(Debug, Clone, Copy)]
pub enum IntervalTvgEdge {
    /// Simple interval edge without data
    BaseEdge(Interval<f32>),
    /// Interval edge with float data
    DataEdge(Interval<f32>, f32),
}

impl IntervalTvgEdge {
    /// `eq` function for the IntervalTvgEdge, in case of comparing two different edges it will panic!
    pub fn eq(self, other: &IntervalTvgEdge) -> bool {
        return match (&self, other) {
            (BaseEdge(interval), BaseEdge(other_interval)) => {
                interval.start == other_interval.start && interval.end == other_interval.end
            }

            (DataEdge(interval, _data), DataEdge(other_interval, _other_data)) => {
                interval.start == other_interval.start && interval.end == other_interval.end
            }

            _ => {
                error!("Edge types are not matching! self: {:?} other: {:?}",self,other);
                panic!("Edge types are not matching! self: {:?} other: {:?}", self, other);
            }
        };
    }
}