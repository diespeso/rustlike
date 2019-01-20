use std::fmt;
use super::position_component::PositionComponent;
use super::render_component::RenderComponent;

#[derive(Debug)]
pub struct Board {
    pub content: Vec<Vec<char>>,
}

impl Board {

    pub fn new(capacity: u32) -> Board {
        let mut board = Board {
            content: Vec::new(),
        };

        for i in 0 .. capacity {
            board.content.push(Vec::new());
            for j in 0 .. capacity {
                board.content[i as usize].push('?');
            }
        }
        board
    }

    pub fn set(&mut self, position: &PositionComponent, render: &RenderComponent) {
        self.content[position.x as usize][position.y as usize] = render.sprite;
    }
}