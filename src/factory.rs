use crate::board::Board;
use crate::cell::Cell;
use crate::params::Params;
use crate::terrain::Terrain;
use crate::tool::*;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Clone, Copy, Component)]
pub struct Factory {
    x: usize,
    y: usize,
    needed_tool: Option<ToolType>,
    quantity: usize,
}

impl Factory {
    pub fn new(x: usize, y: usize, needed_tool: Option<ToolType>, quantity: usize) -> Self {
        Self {
            x,
            y,
            needed_tool,
            quantity,
        }
    }
}

pub fn spawn_factories(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    windows: Res<Windows>,
    mut query: Query<&mut Cell>,
    params: Res<Params>,
) {
    asset_server.watch_for_changes().unwrap();

    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

    let between_width = Uniform::from(0..board.width);
    let between_height = Uniform::from(0..board.height);
    let mut rng = rand::thread_rng();

    for (needed_tool, quantity) in &params.factories_needs {
        let mut cont = 0;
        while cont < 1 {
            let x = between_width.sample(&mut rng);
            let y = between_height.sample(&mut rng);
            let mut cell = query.get_mut(board.cells[x][y]).unwrap();
            match cell.terrain {
                Terrain::Obstacle => {}
                _ => match cell.tool {
                    Some(_) => {}
                    None => match cell.factory {
                        Some(_) => {}
                        None => {
                            let xx = x as f32;
                            let yy = y as f32;
                            let cx = -window.height() / 2.
                                + cell_height * xx
                                + border_width * xx
                                + cell_height / 2.;
                            let cy = -window.width() / 2.
                                + cell_width * yy
                                + border_width * yy
                                + cell_width / 2.;
                            let factory = Factory::new(x, y, Some(*needed_tool), *quantity);
                            commands
                                .spawn_bundle(SpriteBundle {
                                    texture: asset_server.load("factory.png"),
                                    transform: Transform::from_xyz(cx, cy, 2.0),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(factory);
                            cell.factory = Some(factory);
                            cont += 1;
                        }
                    },
                },
            }
        }
    }
}

pub fn render_factories(
    asset_server: Res<AssetServer>,
    mut query: Query<(&Factory, &mut Handle<Image>), Changed<Factory>>,
) {
    for (factory, mut image_handle) in query.iter_mut() {
        match factory.needed_tool {
            Some(ToolType::Battery) => *image_handle = asset_server.load("battery_factory.png"),
            Some(ToolType::WeldingArm) => *image_handle = asset_server.load("welding_factory.png"),
            Some(ToolType::SuctionPump) => *image_handle = asset_server.load("pump_factory.png"),
            Some(ToolType::CoolingDevice) => {
                *image_handle = asset_server.load("cooling_factory.png")
            }
            Some(ToolType::PneumaticArm) => {
                *image_handle = asset_server.load("pneumatic_factory.png")
            }
            None => *image_handle = asset_server.load("factory.png"),
        }
    }
}
