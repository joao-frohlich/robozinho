use crate::board::Board;
use crate::cell::Cell;
use crate::factory::*;
use crate::params::Params;
use crate::terrain::Terrain;
use crate::tool::*;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Default, Component)]
pub struct Agent {
    x: usize,
    y: usize,
    radius: usize,
    state: Vec<(ToolType, usize)>,
    requisitions: Vec<Factory>,
    destination_queue: Vec<(usize, usize)>,
}

fn get_factories(query: Query<&Factory>) -> Vec<Factory> {
    let mut ret: Vec<Factory> = vec![];
    for factory in query.iter() {
        ret.push(*factory)
    }
    ret
}

pub fn setup_agent(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    windows: Res<Windows>,
    mut query: Query<&mut Cell>,
    query_factories: Query<&Factory>,
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

    let mut cont = 0;
    while cont < 1 {
        let x = between_width.sample(&mut rng);
        let y = between_height.sample(&mut rng);
        let cell = query.get_mut(board.cells[x][y]).unwrap();
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
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: asset_server.load("robot.png"),
                                transform: Transform::from_xyz(cx, cy, 2.0),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(cell_width, cell_height)),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Agent {
                                x: x,
                                y: y,
                                radius: params.agent_radius,
                                state: vec![
                                    (ToolType::Battery, 0),
                                    (ToolType::WeldingArm, 0),
                                    (ToolType::SuctionPump, 0),
                                    (ToolType::CoolingDevice, 0),
                                    (ToolType::PneumaticArm, 0),
                                ],
                                requisitions: vec![],
                                destination_queue: vec![],
                            });
                        cont += 1;
                    }
                },
            },
        }
    }
}
