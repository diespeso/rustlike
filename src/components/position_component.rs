use std::fmt;

#[derive(Clone, Debug, Copy)]
pub struct PositionComponent {
    pub x: u32,
    pub y: u32,
}

impl PositionComponent {
    pub fn new(x: u32, y: u32) -> PositionComponent {
        PositionComponent {
            x: x,
            y: y,
        }
    }

    pub fn origin() -> PositionComponent {
        PositionComponent{x: 0, y: 0}
    }

    pub fn as_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn into_tuple(self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl fmt::Display for PositionComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Into<PositionComponent> for (u32, u32) {
    fn into(self) -> PositionComponent {
        PositionComponent::new(self.0, self.1)
    }
}