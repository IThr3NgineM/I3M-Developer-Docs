//! Executor with your game connected to it as a plugin.
use i3m::engine::executor::Executor;
use rpg::Game;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin_constructor(Game::default());
    executor.run()
}
