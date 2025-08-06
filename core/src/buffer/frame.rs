use crate::types::{FrameId, PageIdRef};

pub struct Frame {
    pub frame_id: FrameId,
    pub page_id: PageIdRef,
    pub is_dirty: bool,
    pub pin_count: usize,
    pub data: Vec<u8>,
}

impl Frame {}
