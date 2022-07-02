use bevy::prelude::*;
use bevy::window::PresentMode;
use robozinho::agent::*;
use robozinho::board::*;
use robozinho::factory::*;
use robozinho::params::*;
use robozinho::path::*;
use robozinho::tool::*;

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
        .insert_resource(Path::default())
        .insert_resource(Board::new(42, 42))
        .insert_resource(Params::new(
            vec![
                (ToolType::Battery, 20),
                (ToolType::WeldingArm, 10),
                (ToolType::SuctionPump, 8),
                (ToolType::CoolingDevice, 6),
                (ToolType::PneumaticArm, 4),
            ],
            vec![
                (ToolType::Battery, 8),
                (ToolType::WeldingArm, 5),
                (ToolType::SuctionPump, 2),
                (ToolType::CoolingDevice, 5),
                (ToolType::PneumaticArm, 2),
            ],
            4,
            1,
            0,
            1,
        ))
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_board)
        .add_startup_system(setup_camera)
        .add_startup_system(color_cells)
        .add_startup_system(spawn_tools)
        .add_startup_system(spawn_factories)
        .add_startup_system(setup_agent)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_agent_factories)
        .add_system(render_tools)
        .add_system(render_factories)
        .add_system(move_agent)
        .add_system(follow_path)
        .add_system(update_agent_factories)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
