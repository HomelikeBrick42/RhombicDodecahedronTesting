use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(grid::GamePlugin )
        .run();
}
