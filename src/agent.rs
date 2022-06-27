use crate::board::Board;
use crate::cell::Cell;
use crate::factory::*;
use crate::params::Params;
use crate::terrain::Terrain;
use crate::tool::*;
use bevy::prelude::*;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use rand::distributions::WeightedIndex;
use rand::distributions::{Distribution, Uniform};
use std::{thread, time};

#[derive(Default, Component)]
pub struct Agent {
    x: usize,
    y: usize,
    radius: usize,
    state: Vec<(ToolType, usize)>,
    requisitions: Vec<Factory>,
    destination_queue: Vec<(usize, usize)>,
}

pub fn setup_agent(
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

pub fn setup_agent_factories(mut query: Query<&mut Agent>, query_factories: Query<&Factory>) {
    let mut agent = query.get_single_mut().unwrap();
    for factory in query_factories.iter() {
        agent.requisitions.push(*factory)
    }
}

pub fn check_radius(
    board: &Res<Board>,
    ax: i32,
    ay: i32,
    r: i32,
    destinations: &mut Vec<(usize, usize)>,
    query_cell: &Query<&mut Cell>,
) {
    let width = board.width as i32;
    let height = board.height as i32;
    for x in ax - r..=ax + r {
        for y in ay - r..=ay + r {
            if x >= 0 && x < width && y >= 0 && y < height {
                if destinations.contains(&(x as usize, y as usize)) {
                    continue;
                }
                let cell = query_cell.get(board.cells[x as usize][y as usize]).unwrap();
                match cell.tool {
                    Some(_) => {
                        destinations.push((x as usize, y as usize));
                    }
                    None => {}
                }
            }
        }
    }
}

pub fn check_requisitions(agent: &mut Agent) {
    let mut batteries: usize = 0;
    let mut welding_arms: usize = 0;
    let mut pumps: usize = 0;
    let mut cooling_devices: usize = 0;
    let mut pneumatic_arms: usize = 0;

    for (tool_type, quantity) in &agent.state {
        match tool_type {
            ToolType::Battery => {
                batteries = *quantity;
            }
            ToolType::WeldingArm => {
                welding_arms = *quantity;
            }
            ToolType::SuctionPump => {
                pumps = *quantity;
            }
            ToolType::CoolingDevice => {
                cooling_devices = *quantity;
            }
            ToolType::PneumaticArm => {
                pneumatic_arms = *quantity;
            }
        }
    }

    for factory in &agent.requisitions {
        let (x, y) = (factory.x, factory.y);
        if agent.destination_queue.contains(&(x as usize, y as usize)) {
            continue;
        }
        match factory.needed_tool {
            Some(ToolType::Battery) => {
                if batteries >= factory.quantity {
                    agent.destination_queue.push((x, y));
                }
            }
            Some(ToolType::WeldingArm) => {
                if welding_arms >= factory.quantity {
                    agent.destination_queue.push((x, y));
                }
            }
            Some(ToolType::SuctionPump) => {
                if pumps >= factory.quantity {
                    agent.destination_queue.push((x, y));
                }
            }
            Some(ToolType::CoolingDevice) => {
                if cooling_devices >= factory.quantity {
                    agent.destination_queue.push((x, y));
                }
            }
            Some(ToolType::PneumaticArm) => {
                if pneumatic_arms >= factory.quantity {
                    agent.destination_queue.push((x, y));
                }
            }
            None => {}
        }
    }
}

fn h((ax, ay) : (i32, i32), (bx, by) : (i32, i32)) -> i32 {
    (ax-bx).abs() + (ay-by).abs()
}

fn check_next_destination(agent: &Agent) -> usize {
    let mut min_distance: usize = 1000000000;
    let mut idx: usize = 0;
    let mut min_idx: usize = 0;
    for (x,y) in &agent.destination_queue {
        let distance = h((agent.x as i32, agent.y as i32),(*x as i32, *y as i32)) as usize;
        if distance < min_distance {
            min_distance = distance;
            min_idx = idx;
        }
        idx += 1;
    }
    min_idx
}

fn search_requisition(tool_type: ToolType, agent: &Agent) -> bool {
    for factory in &agent.requisitions {
        if factory.needed_tool == Some(tool_type) {
            return true;
        }
    }
    false
}

fn valid(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x < 0 {return false;}
    if y < 0 {return false;}
    if x >= width {return false;}
    if y >= height {return false;}
    true
}

pub fn move_agent(
    windows: Res<Windows>,
    board: Res<Board>,
    mut query: Query<(&mut Agent, &mut Transform)>,
    query_cell: Query<&mut Cell>,
    mut _params: ResMut<Params>,
) {
    let time = time::Duration::from_secs_f32(0.1);
    thread::sleep(time);

    let moves: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let window = windows.primary();
    let border_width = 2.0;
    let cell_width =
        (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
    let cell_height =
        (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

    let (mut agent, mut transform) = query.get_single_mut().unwrap();

    check_requisitions(&mut agent);

    check_radius(
        &board,
        agent.x as i32,
        agent.y as i32,
        agent.radius as i32,
        &mut agent.destination_queue,
        &query_cell,
    );

    // Implementar A* aqui

    // Distância heurística: Manhattan

    let width = board.width as i32;
    let height = board.height as i32;

    while agent.destination_queue.len() > 0 {
        let next_idx = check_next_destination(&agent);
        let (dx, dy) = agent.destination_queue.remove(next_idx);
        let cell = query_cell.get(board.cells[dx][dy]).unwrap();
        let mut should_find_path = true;
        match cell.tool {
            Some(ToolType::Battery) => {should_find_path = search_requisition(ToolType::Battery, &agent)}
            Some(ToolType::WeldingArm) => {should_find_path = search_requisition(ToolType::WeldingArm, &agent)}
            Some(ToolType::SuctionPump) => {should_find_path = search_requisition(ToolType::SuctionPump, &agent)}
            Some(ToolType::CoolingDevice) => {should_find_path = search_requisition(ToolType::CoolingDevice, &agent)}
            Some(ToolType::PneumaticArm) => {should_find_path = search_requisition(ToolType::PneumaticArm, &agent)}
            None => {}
        }
        if !should_find_path {
            continue;
        }
        let (ax, ay) = (agent.x as i32, agent.y as i32);
        let mut pq = PriorityQueue::new();
        let mut vis: HashSet<(i32, i32)> = HashSet::new();
        let mut final_cost = 0;
        pq.push((ax, ay), 0);
        while !pq.is_empty() {
            let ((cx, cy), cost) = pq.pop().unwrap();
            if vis.contains(&(dx as i32, dy as i32)) {
                continue;
            }
            if vis.contains(&(cx, cy)) {
                continue;
            }
            vis.insert((cx, cy));
            if cx == dx as i32 && cy == dy as i32 {
                final_cost = cost;
            }
            for (mx, my) in moves {
                let (nx, ny) = (cx+mx, cy+my);
                if !valid(nx, ny, width, height) {continue;}
                let n_cell = query_cell.get(board.cells[nx as usize][ny as usize]).unwrap();
                let g:i32 = match n_cell.terrain {
                    Terrain::Grass => 1,
                    Terrain::Mountain => 5,
                    Terrain::Swamp => 10,
                    Terrain::Desert => 20,
                    Terrain::Obstacle => -1
                };
                let n_cost = cost - g - h((nx,ny), (dx as i32, dy as i32));
                pq.push((nx, ny), n_cost);
            }
        }
        //Agora precisamos pegar o caminho de "menor" custo
        println!("Distancia entre ({}, {}) e ({}, {}): {}", ax, ay, dx, dy, -final_cost);
    }

    // Primeiro passo e meio: Verificar se o destino escolhido é válido (i. e. existe uma
    // requisição para o item). Caso não seja, ir para o passo final.

    // Segundo passo: executar o algoritmo de A* e determinar o caminho até o destino.

    // Terceiro passo: ir até o destino, marcando novos destinos no caminho, mas sem olhar para
    // esses novos destinos.

    // Passo final: Repetir primeiro passo até que a lista de destinos esteja vazia.

    let mut weights: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut has_option = false;

    if agent.x < board.height - 1 {
        let x = agent.x + 1;
        let y = agent.y;
        let cell = query_cell.get(board.cells[x][y]).unwrap();
        match cell.terrain {
            Terrain::Grass => {
                weights[0] = 1.0;
                has_option = true;
            }
            Terrain::Mountain => {
                weights[0] = 1.0 / 5.0;
                has_option = true;
            }
            Terrain::Swamp => {
                weights[0] = 1.0 / 10.0;
                has_option = true;
            }
            Terrain::Desert => {
                weights[0] = 1.0 / 20.0;
                has_option = true;
            }
            _ => {}
        }
    }
    if agent.y < board.width - 1 {
        let x = agent.x;
        let y = agent.y + 1;
        let cell = query_cell.get(board.cells[x][y]).unwrap();
        match cell.terrain {
            Terrain::Grass => {
                weights[1] = 1.0;
                has_option = true;
            }
            Terrain::Mountain => {
                weights[1] = 1.0 / 5.0;
                has_option = true;
            }
            Terrain::Swamp => {
                weights[1] = 1.0 / 10.0;
                has_option = true;
            }
            Terrain::Desert => {
                weights[1] = 1.0 / 20.0;
                has_option = true;
            }
            _ => {}
        }
    }
    if agent.x > 0 {
        let x = agent.x - 1;
        let y = agent.y;
        let cell = query_cell.get(board.cells[x][y]).unwrap();
        match cell.terrain {
            Terrain::Grass => {
                weights[2] = 1.0;
                has_option = true;
            }
            Terrain::Mountain => {
                weights[2] = 1.0 / 5.0;
                has_option = true;
            }
            Terrain::Swamp => {
                weights[2] = 1.0 / 10.0;
                has_option = true;
            }
            Terrain::Desert => {
                weights[2] = 1.0 / 20.0;
                has_option = true;
            }
            _ => {}
        }
    }
    if agent.y > 0 {
        let x = agent.x;
        let y = agent.y - 1;
        let cell = query_cell.get(board.cells[x][y]).unwrap();
        match cell.terrain {
            Terrain::Grass => {
                weights[3] = 1.0;
                has_option = true;
            }
            Terrain::Mountain => {
                weights[3] = 1.0 / 5.0;
                has_option = true;
            }
            Terrain::Swamp => {
                weights[3] = 1.0 / 10.0;
                has_option = true;
            }
            Terrain::Desert => {
                weights[3] = 1.0 / 20.0;
                has_option = true;
            }
            _ => {}
        }
    }

    if has_option {
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();
        let movement = moves[dist.sample(&mut rng)];
        let new_x: usize = (agent.x as i32 + movement.0) as usize;
        let new_y: usize = (agent.y as i32 + movement.1) as usize;
        agent.x = new_x;
        agent.y = new_y;
    }

    let x = agent.x as f32;
    let y = agent.y as f32;
    let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
    let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
    let translation = &mut transform.translation;
    translation.x = cx;
    translation.y = cy;
}
