use crate::cell::Cell;
use crate::params::Params;
use crate::terrain::Terrain;
use crate::tool::Tool;
use crate::tool::ToolType;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::fs;

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Entity>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Entity::from_raw(0); width]; height];
        Self {
            width,
            height,
            cells,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(42, 42)
    }
}

fn read_terrain() -> Vec<Vec<Terrain>> {
    let mut data: Vec<Vec<Terrain>> = Vec::<Vec<Terrain>>::default();
    let field_path = "fields/field.txt".to_string();
    let contents = fs::read_to_string(field_path).expect("Something went wrong");
    for line in contents.split('\n') {
        let line = line.trim();
        let values: Vec<&str> = line.split_whitespace().collect();
        let mut line_terrains: Vec<Terrain> = Vec::<Terrain>::default();
        for value in values {
            let terrain_value = value.parse::<usize>().unwrap();
            match terrain_value {
                0 => line_terrains.push(Terrain::Grass),
                1 => line_terrains.push(Terrain::Mountain),
                2 => line_terrains.push(Terrain::Swamp),
                3 => line_terrains.push(Terrain::Desert),
                4 => line_terrains.push(Terrain::Obstacle),
                _ => line_terrains.push(Terrain::Grass),
            };
        }
        data.push(line_terrains);
    }
    data
}

pub fn setup_board(
    mut commands: Commands,
    windows: Res<Windows>,
    mut board: ResMut<Board>,
) {
    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

    let terrain = read_terrain();

    for xx in 0..board.height {
        for yy in 0..board.width {
            let x = xx as f32;
            let y = yy as f32;
            let cx = -window.height() / 2. + cell_height * x + border_width * x + cell_height / 2.;
            let cy = -window.width() / 2. + cell_width * y + border_width * y + cell_width / 2.;
            let cell_x = board.height - yy - 1;
            let cell_y = xx;
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(cx, cy, 1.0),
                    sprite: Sprite {
                        color: Color::rgb(1., 1., 1.),
                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Cell::new(terrain[cell_x][cell_y], None, None))
                .id();
            board.cells[xx][yy] = entity;
        }
    }
}

pub fn spawn_tools(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>, windows: Res<Windows>, mut query: Query<&mut Cell>, params: Res<Params>) {
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
    

    for (tool, quantity) in &params.items_quantity {
        let mut cont = 0;
        while cont < *quantity {
            let x = between_width.sample(&mut rng);
            let y = between_height.sample(&mut rng);
            let mut cell = query.get_mut(board.cells[x][y]).unwrap();
            match cell.terrain {
                Terrain::Grass => match cell.tool {
                    Some(_) => {}
                    None => {
                        let xx = x as f32;
                        let yy = y as f32;
                        let cx = -window.height() / 2. + cell_height * xx + border_width * xx + cell_height / 2.;
                        let cy = -window.width() / 2. + cell_width * yy + border_width * yy + cell_width / 2.;
                        commands.spawn_bundle(SpriteBundle {
                            texture: asset_server.load("empty_texture.png"),
                            transform: Transform::from_xyz(cx, cy, 2.0),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(cell_width, cell_height)),
                                ..default()
                            },
                            ..default()
                        }).insert(Tool::new(x, y, Some(*tool)));
                        cell.tool = Some(*tool);
                        cont += 1;
                    }
                },
                _ => {}
            }
        }
    }
}

pub fn color_cells(mut query_cell: Query<(&Cell, &mut Sprite), Changed<Cell>>) {
    for (cell, mut sprite) in query_cell.iter_mut() {
        let colors: (f32, f32, f32) = match cell.terrain {
            Terrain::Grass => (0.569, 0.816, 0.306),
            Terrain::Mountain => (0.576, 0.541, 0.325),
            Terrain::Swamp => (0.325, 0.55, 0.827),
            Terrain::Desert => (0.89, 0.42, 0.04),
            Terrain::Obstacle => (0.0, 0.0, 0.0),
        };
        let (red, green, blue) = colors;
        sprite.color = Color::rgb(red, green, blue);
    }
}

pub fn render_tools(asset_server: Res<AssetServer>, mut query: Query<(&Tool, &mut Handle<Image>), Changed<Tool>>) {
    for (tool, mut image_handle) in query.iter_mut() {
        match tool.tool_type {
            Some(ToolType::Battery) => {*image_handle = asset_server.load("electric_battery.png")}
            Some(ToolType::WeldingArm) => {*image_handle = asset_server.load("welding_arm.png")}
            Some(ToolType::SuctionPump) => {*image_handle = asset_server.load("suction_pump.png")}
            Some(ToolType::CoolingDevice) => {*image_handle = asset_server.load("cooling_device.png")}
            Some(ToolType::PneumaticArm) => {*image_handle = asset_server.load("pneumatic_arm.png")}
            None => {*image_handle = asset_server.load("empty_texture.png")}
        }
    }
}




