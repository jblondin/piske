//! Definition of the StepRange iterator use in transpiled source code.

use std::ops::Add;

/// Iterates from a starting point to an end point (exclusive or inclusive) by a certain step.
pub struct StepRange<T> {
    start: T,
    end: T,
    end_inclusive: bool,
    step: T
}
impl<T> StepRange<T> {
    /// Create new StepRange iterator
    pub fn new(start: T, end: T, end_inclusive: bool, step: T) -> StepRange<T> {
        StepRange {
            start: start,
            end: end,
            end_inclusive: end_inclusive,
            step: step,
        }
    }
}
impl<T: Copy + Add<Output=T> + PartialOrd> Iterator for StepRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if (self.end_inclusive && self.start <= self.end) ||
                (!self.end_inclusive && self.start < self.end) {
            let value = self.start;
            self.start = self.start + self.step;
            Some(value)
        } else {
            None
        }
    }
}
