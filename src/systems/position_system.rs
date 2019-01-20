use components::position_component::PositionComponent;
use entity::Entity;

#[derive(Debug)]
pub struct PositionSystem {
    pub content: Vec<Option<PositionComponent>>,
}

impl PositionSystem {
    pub fn new() -> PositionSystem {
        PositionSystem {
            content: Vec::new(),
        }
    }

    pub fn add(&mut self, position: PositionComponent) {
        self.content.push(Some(position))
    }

    pub fn get_component(&self, entity: Entity) -> Option<&PositionComponent> {
        self.content[entity as usize].as_ref()
    }

    pub fn get_component_mut(&mut self, entity: Entity) -> Option<&mut PositionComponent> {
        self.content[entity as usize].as_mut()
    }
    //TODO: Refactor this shit for god's sake.
    pub fn move_entity(&mut self, entity: Entity, position: PositionComponent) -> Option<PositionComponent> {
        let last_position = self.get_component(entity).cloned();
        self.content[entity as usize] = Some(position);
        last_position
    }
}