use crate::board::Board;
use crate::cell::Cell;
use crate::params::Params;
use crate::terrain::Terrain;
use bevy::prelude::*;
use std::fs;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ToolType {
    Battery,
    WeldingArm,
    SuctionPump,
    CoolingDevice,
    PneumaticArm,
}

#[derive(Clone, Copy, Component)]
pub struct Tool {
    pub x: usize,
    pub y: usize,
    pub tool_type: Option<ToolType>,
}

impl Tool {
    pub fn new(x: usize, y: usize, tool_type: Option<ToolType>) -> Self {
        Self { x, y, tool_type }
    }
}

fn read_tools(idx: usize) -> Vec<(usize, usize)> {
    let mut data: Vec<(usize, usize)> = Vec::<(usize, usize)>::default();
    let field_path = "inputs/tools_".to_string() + &idx.to_string();
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

pub fn spawn_tools(
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
    let tools_positions = read_tools(params.input_idx);

    for (tool, quantity) in &params.items_quantity {
        let mut cont = 0;
        while cont < *quantity {
            let (x, y) = tools_positions[idx];
            idx += 1;
            let mut cell = query.get_mut(board.cells[x][y]).unwrap();
            match cell.terrain {
                Terrain::Grass => match cell.tool {
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
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: asset_server.load("empty_texture.png"),
                                transform: Transform::from_xyz(cx, cy, 2.0),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(cell_width, cell_height)),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Tool::new(x, y, Some(*tool)));
                        cell.tool = Some(*tool);
                        cont += 1;
                    }
                },
                _ => {}
            }
        }
    }
}

pub fn render_tools(
    asset_server: Res<AssetServer>,
    mut query: Query<(&Tool, &mut Handle<Image>), Changed<Tool>>,
) {
    for (tool, mut image_handle) in query.iter_mut() {
        match tool.tool_type {
            Some(ToolType::Battery) => *image_handle = asset_server.load("battery.png"),
            Some(ToolType::WeldingArm) => *image_handle = asset_server.load("welding.png"),
            Some(ToolType::SuctionPump) => *image_handle = asset_server.load("pump.png"),
            Some(ToolType::CoolingDevice) => *image_handle = asset_server.load("cooling.png"),
            Some(ToolType::PneumaticArm) => *image_handle = asset_server.load("pneumatic.png"),
            None => *image_handle = asset_server.load("empty_texture.png"),
        }
    }
}
