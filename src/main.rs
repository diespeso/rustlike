pub mod components;
pub mod entity;
pub mod game;
pub mod systems;

use game::Game;


fn main() {
    let mut game = Game::new();
    game.add_water((3, 2).into());
    game.update();
    game.draw();
    println!("{:?}", game);
    game.move_entity(0, (3, 3).into());
    println!("{:?}", game);
    game.update();
    game.draw();
}