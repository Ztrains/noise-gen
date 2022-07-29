#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

impl Vector2D {
    // constructor
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }

    // calculates dot product of vectors self and other
    pub fn dot(&self, other: Vector2D) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}