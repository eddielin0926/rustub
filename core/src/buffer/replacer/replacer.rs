use crate::types::FrameId;

pub trait Replacer {
    fn record_access(&mut self, frame_id: FrameId);

    fn set_evictable(&mut self, frame_id: FrameId, evictable: bool);

    fn evict(&mut self) -> Option<FrameId>;
}
