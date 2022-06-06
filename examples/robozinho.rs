use bevy::prelude::*;
use bevy::window::PresentMode;
use robozinho::board::*;
use robozinho::params::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robozinho".to_string(),
            width: 700.,
            height: 700.,
            resizable: false,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<Board>()
        .insert_resource(Board::new(42, 42))
        .insert_resource(Params::default())
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_board)
        .add_startup_system(setup_camera)
        .add_startup_system(color_cells)
        .add_startup_system(spawn_tools)
        .add_system(render_tools)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
