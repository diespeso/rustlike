use components::board::Board;
use entity::Entity;
use components::render_component::RenderComponent;
use components::position_component::PositionComponent;
use systems::position_system::PositionSystem;

#[derive(Debug)]
pub struct RenderSystem {
    pub board: Board,
    pub content: Vec<Option<RenderComponent>>,
}

impl RenderSystem {
    
    pub fn new(board: Board) -> RenderSystem {
        RenderSystem {
            board: board,
            content: Vec::new(),
        }
    }

    pub fn add(&mut self, render_component: RenderComponent) {
        self.content.push(Some(render_component));
    }

    pub fn get_component(&self, entity: Entity) -> Option<&RenderComponent> {
        self.content[entity as usize].as_ref()
    }

    pub fn get_component_mut(&mut self, entity: Entity) -> Option<&mut RenderComponent> {
        self.content[entity as usize].as_mut()
    }

    pub fn set(&mut self, entity: Entity, render_component: RenderComponent) {
        self.content[entity as usize] = Some(render_component);
    }

    //Iterates over the position_system, if the current entity doesn't have a position component
    //does nothing, if not retrieves the render component of the current entity and sets the 
    //retrieved sprite in the retrieved position, if there's no sprite, it does nothing.
    pub fn update(&mut self, position_system: &PositionSystem) {
        //for every renderable
        for i in 0 .. self.content.len() {
            //match its position component
            match position_system.get_component(i as u32) {
                Some(ref ref_position) => {
                    //match its render component
                    match self.content[i] {
                       Some(ref ref_render) => self.board.set(ref_position, ref_render),
                       None => (),
                   }
                },
                None => (),
            }
        }
    }

    pub fn draw(&self) {
        for vect in self.board.content.iter() {
            for car in vect.iter() {
                print!("{}{}{}", match *car {
                    '-' => "\x1b[38;5;206m",
                    '~' => "\x1b[36;1m",
                    _ => "",
                },
                car,
                "\x1b[0m "
                );
            }
            println!("");
        }
    }
}