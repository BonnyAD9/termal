/// Rectangle given by x, y, width and height.
#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    /// Creates new rectangle from its position and size.
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}
