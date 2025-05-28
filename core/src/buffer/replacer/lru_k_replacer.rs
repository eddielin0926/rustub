use std::collections::{HashMap, HashSet, VecDeque};

use crate::types::FrameId;

use super::replacer::Replacer;

pub struct LruKReplacer {
    k: usize,
    current_timestamp: usize,
    history: HashMap<FrameId, VecDeque<usize>>,
    pinned: HashSet<FrameId>,
}

impl Replacer for LruKReplacer {
    fn record_access(&mut self, frame_id: FrameId) {
        let history = self.history.entry(frame_id).or_insert_with(VecDeque::new);
        history.push_back(self.current_timestamp);

        self.current_timestamp += 1;

        if history.len() > self.k {
            history.pop_front();
        }
    }

    fn pin(&mut self, frame_id: FrameId) {
        self.pinned.insert(frame_id);
    }

    fn unpin(&mut self, frame_id: FrameId) {
        self.pinned.remove(&frame_id);
    }

    fn evict(&mut self) -> Option<FrameId> {
        let mut victim: Option<FrameId> = None;
        let mut max_distance: usize = 0;
        let mut oldest_history: usize = 0;

        for (&frame_id, history) in &self.history {
            if self.pinned.contains(&frame_id) {
                continue;
            }

            let distance;
            let oldest_ts = *history.front().unwrap();

            if history.len() < self.k {
                distance = usize::MAX;
            } else {
                distance = self.current_timestamp - history.front().unwrap();
            }

            if distance > max_distance || (distance == max_distance && oldest_ts < oldest_history) {
                max_distance = distance;
                oldest_history = oldest_ts;
                victim = Some(frame_id);
            }
        }

        if let Some(fid) = victim {
            self.history.remove(&fid);
            self.pinned.remove(&fid);
        }

        victim
    }
}

impl LruKReplacer {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            current_timestamp: 0,
            history: HashMap::new(),
            pinned: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_victim() {
        let mut replacer = LruKReplacer::new(2);

        assert_eq!(replacer.evict(), None);
    }

    #[test]
    fn pin_all() {
        let mut replacer = LruKReplacer::new(2);
        replacer.record_access(1);
        replacer.pin(1);
        assert_eq!(replacer.evict(), None);
    }
}
