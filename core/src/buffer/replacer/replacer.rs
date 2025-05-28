use crate::types::FrameId;

pub trait Replacer {
    fn record_access(&mut self, frame_id: FrameId);

    fn pin(&mut self, frame_id: FrameId);

    fn unpin(&mut self, frame_id: FrameId);

    fn evict(&mut self) -> Option<FrameId>;
}
