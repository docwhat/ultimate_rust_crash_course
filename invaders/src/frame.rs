use crate::{NUM_COLS, NUM_ROWS};

// Two dimensional array of characters.
pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    vec![vec![" "; NUM_ROWS]; NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
