use entity::Entity;
use components::board::Board;
use components::render_component::RenderComponent;
use components::position_component::PositionComponent;
use systems::render_system::RenderSystem;
use systems::position_system::PositionSystem;

#[derive(Debug)]
pub struct Game {
    pub render_system: RenderSystem,
    pub position_system: PositionSystem,
}

impl Game {

    pub fn new() -> Game {
        Game {
            render_system: RenderSystem::new(Board::new(10)),
            position_system: PositionSystem::new(),
        }
    }

    pub fn update(&mut self) {
        self.render_system.update(&self.position_system);
    }

    pub fn draw(&self) {
        self.render_system.draw();
    }

    pub fn add_water(&mut self, position: PositionComponent) -> Entity {
        self.render_system.add(RenderComponent::new('~'));
        self.position_system.add(position);
        0
    }  

    pub fn move_entity(&mut self, entity: Entity, position: PositionComponent) {
        let last_pos = self.position_system.move_entity(entity, position);
        match last_pos {
            Some(ref pos) => self.render_system.board.set(pos, &RenderComponent::new('?')),
            None => (),
        }
        self.render_system.update(&self.position_system);
    }
}