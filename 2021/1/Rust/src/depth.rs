use std::collections::VecDeque;
use std::default::Default;

#[derive(Debug, Default)]
pub struct SlidingWindowDepthMeasures {
    queue: VecDeque<usize>,
}

impl SlidingWindowDepthMeasures {
    pub fn add(&mut self, depth_measure: usize) {
        self.queue.push_back(depth_measure);
        while self.queue.len() > 3 {
            self.queue.pop_front();
        }
    }

    pub fn measures_sum(&self) -> Option<usize> {
        if !self.enough_len() {
            return None;
        }
        return Some(self.queue.iter().sum());
    }

    pub fn enough_len(&self) -> bool { self.queue.len() == 3 }
}
