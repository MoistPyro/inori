pub mod library_layout;
pub mod queue_layout;
use crate::model::*;
use crate::view::Rect;

pub trait InoriLayout {
    fn new(frame_rect: Rect, model: &Model) -> Self;
}
