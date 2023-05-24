use bevy::app::App;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}