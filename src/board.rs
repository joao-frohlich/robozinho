use crate::cell::Cell;
use crate::terrain::Terrain;
use bevy::prelude::*;
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

pub fn setup_board(mut commands: Commands, windows: Res<Windows>, mut board: ResMut<Board>) {
    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

    let terrain = read_terrain();

    for xx in 0..board.width {
        for yy in 0..board.height {
            let x = xx as f32;
            let y = yy as f32;
            let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
            let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
            let cell_x = board.height-yy-1;
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
                .insert(Cell::new(None, terrain[cell_x][cell_y]))
                .id();
            board.cells[cell_x][cell_y] = entity;
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
