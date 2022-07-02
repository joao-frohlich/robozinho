use crate::board::Board;
use crate::cell::Cell;
use crate::params::Params;
use crate::terrain::Terrain;
use crate::tool::*;
use bevy::prelude::*;
use std::fs;

#[derive(Clone, Copy, Component, Debug)]
pub struct Factory {
    pub x: usize,
    pub y: usize,
    pub needed_tool: Option<ToolType>,
    pub quantity: usize,
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

fn read_factories(idx: usize) -> Vec<(usize, usize)> {
    let mut data: Vec<(usize, usize)> = Vec::<(usize, usize)>::default();
    let field_path = "inputs/factories_".to_string() + &idx.to_string();
    let contents = fs::read_to_string(field_path).expect("Something went wrong");
    for line in contents.split('\n') {
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() != 2 {
            break;
        }
        let x = values[0].parse::<usize>().unwrap();
        let y = values[1].parse::<usize>().unwrap();
        data.push((x, y));
    }
    data
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

    let mut idx: usize = 0;
    let factories_positions = read_factories(params.input_idx);

    for (needed_tool, quantity) in &params.factories_needs {
        let mut cont = 0;
        while cont < 1 {
            let (x, y) = factories_positions[idx];
            idx += 1;
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
    mut query: Query<(&Factory, &mut Handle<Image>)>,
) {
    for (factory, mut image_handle) in query.iter_mut() {
        // println!("Factory: {:?}", factory);
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
