use std::fmt;

pub struct RenderComponent {
    pub sprite: char,
}

impl RenderComponent {
    pub fn new(sprite: char) -> RenderComponent {
        RenderComponent {
            sprite: sprite,
        }
    }
}

impl fmt::Debug for RenderComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RenderComponent['{}']", self.sprite)
    }
}