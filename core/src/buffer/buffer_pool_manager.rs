use std::collections::HashMap;

use crate::types::{FrameId, PageId};

use super::frame::Frame;

pub struct BufferBoolManager {
    pub frames: Vec<Frame>,
    pub free_list: Vec<FrameId>,
    pub page_table: HashMap<PageId, FrameId>,
}
