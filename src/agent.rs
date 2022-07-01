use crate::board::Board;
use crate::cell::Cell;
use crate::factory::*;
use crate::params::Params;
use crate::path::*;
use crate::terrain::Terrain;
use crate::tool::*;
use bevy::prelude::*;
use priority_queue::PriorityQueue;
use rand::distributions::WeightedIndex;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::{thread, time};

#[derive(Default, Component)]
pub struct Agent {
    x: usize,
    y: usize,
    radius: usize,
    cost: usize,
    expansions: usize,
    last_move: (i32, i32),
    ended: bool,
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
                                cost: 0,
                                expansions: 0,
                                last_move: (0, 0),
                                ended: false,
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

pub fn update_agent_factories(query: Query<&Agent>, mut query_factories: Query<&mut Factory>) {
    let agent = query.get_single().unwrap();
    for mut factory in query_factories.iter_mut() {
        let (x, y) = (factory.x, factory.y);
        for fact in &agent.requisitions {
            if x == fact.x && y == fact.y {
                factory.needed_tool = fact.needed_tool;
                factory.quantity = fact.quantity;
            }
        }
    }
}

pub fn check_radius(
    board: &Res<Board>,
    ax: i32,
    ay: i32,
    r: i32,
    destinations: &mut Vec<(usize, usize)>,
    requisitions: &Vec<Factory>,
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
                    Some(tool) => {
                        if search_requisition(tool, &requisitions) {
                            destinations.push((x as usize, y as usize));
                        }
                    }
                    None => {}
                }
            }
        }
    }
}

pub fn check_requisitions(agent: &mut Agent) -> bool {
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

    let mut ret = 0;

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
            None => {
                ret += 1;
            }
        }
    }
    ret == agent.requisitions.len()
}

fn h((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> i32 {
    (ax - bx).abs() + (ay - by).abs()
}

fn check_next_destination(agent: &Agent) -> usize {
    println!("\nChecking next destination");
    println!("Current destination queue: {:?}", agent.destination_queue);
    let mut min_distance: usize = 1000000000;
    let mut idx: usize = 0;
    let mut min_idx: usize = 0;
    for (x, y) in &agent.destination_queue {
        let distance = h((agent.x as i32, agent.y as i32), (*x as i32, *y as i32)) as usize;
        if distance < min_distance {
            min_distance = distance;
            min_idx = idx;
        }
        idx += 1;
    }
    println!(
        "My next destination is: {:?}",
        agent.destination_queue[min_idx]
    );
    min_idx
}

fn search_requisition(tool_type: ToolType, requisitions: &Vec<Factory>) -> bool {
    for factory in requisitions {
        if factory.needed_tool == Some(tool_type) {
            return true;
        }
    }
    false
}

fn valid(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x < 0 {
        return false;
    }
    if y < 0 {
        return false;
    }
    if x >= width {
        return false;
    }
    if y >= height {
        return false;
    }
    true
}

pub fn move_agent(
    windows: Res<Windows>,
    board: Res<Board>,
    mut follow_path: ResMut<Path>,
    mut query: Query<(&mut Agent, &mut Transform)>,
    query_cell: Query<&mut Cell>,
    mut _params: ResMut<Params>,
) {
    if follow_path.moves.is_empty() {
        let time = time::Duration::from_secs_f32(0.1);
        // thread::sleep(time);

        let moves: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let window = windows.primary();
        let border_width = 2.0;
        let cell_width =
            (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
        let cell_height =
            (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

        let (mut agent, mut transform) = query.get_single_mut().unwrap();

        if check_requisitions(&mut agent) {
            if !agent.ended {
                println!("End of execution");
                println!("Final cost: {}", agent.cost);
                println!("Number of expansions: {}", agent.expansions);
                agent.ended = true;
            }
            return;
        }

        let mut requisitions: Vec<Factory> = vec![];
        for requisition in &agent.requisitions {
            requisitions.push(*requisition);
        }

        check_radius(
            &board,
            agent.x as i32,
            agent.y as i32,
            agent.radius as i32,
            &mut agent.destination_queue,
            &requisitions,
            &query_cell,
        );

        // Implementar A* aqui

        // Distância heurística: Manhattan

        let width = board.width as i32;
        let height = board.height as i32;

        if agent.destination_queue.len() > 0 {
            let next_idx = check_next_destination(&agent);
            let (dx, dy) = agent.destination_queue.remove(next_idx);
            println!("Finding path to ({}, {})", dx, dy);
            println!(
                "Current destination queue is: {:?}",
                agent.destination_queue
            );
            let cell = query_cell.get(board.cells[dx][dy]).unwrap();
            let mut should_find_path = true;
            match cell.tool {
                Some(ToolType::Battery) => {
                    should_find_path = search_requisition(ToolType::Battery, &agent.requisitions)
                }
                Some(ToolType::WeldingArm) => {
                    should_find_path = search_requisition(ToolType::WeldingArm, &agent.requisitions)
                }
                Some(ToolType::SuctionPump) => {
                    should_find_path =
                        search_requisition(ToolType::SuctionPump, &agent.requisitions)
                }
                Some(ToolType::CoolingDevice) => {
                    should_find_path =
                        search_requisition(ToolType::CoolingDevice, &agent.requisitions)
                }
                Some(ToolType::PneumaticArm) => {
                    should_find_path =
                        search_requisition(ToolType::PneumaticArm, &agent.requisitions)
                }
                None => match cell.factory {
                    Some(factory) => match factory.needed_tool {
                        Some(_) => {}
                        None => should_find_path = false,
                    },
                    None => should_find_path = false,
                },
            }

            println!(
                "Should I find a path to ({}, {}): {}",
                dx, dy, should_find_path
            );

            if should_find_path {
                let (ax, ay) = (agent.x as i32, agent.y as i32);
                let mut pq = PriorityQueue::new();
                let mut partial_cost: HashMap<(i32, i32), i32> = HashMap::new();
                let mut final_cost = -1000000;
                let mut path: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
                pq.push((ax, ay, 0, 0), 0);
                while !pq.is_empty() {
                    let ((cx, cy, mvx, mvy), cost) = pq.pop().unwrap();
                    if -cost >= -final_cost {
                        continue;
                    }
                    if partial_cost.contains_key(&(cx, cy)) && -cost >= -partial_cost[&(cx, cy)] {
                        continue;
                    }
                    println!("\nGoing from ({}, {}) to ({}, {})", cx, cy, dx, dy);
                    if path.contains_key(&(cx, cy)) {
                        println!("Current path: {:?}", path[(&(cx, cy))]);
                    }
                    println!("Last move: ({}, {})", mvx, mvy);
                    println!("Current cost: {}", -cost);
                    if partial_cost.contains_key(&(cx, cy)) {
                        println!("Partial cost: {}", -partial_cost[&(cx, cy)]);
                    }
                    partial_cost.remove(&(cx, cy));
                    partial_cost.insert((cx, cy), cost);
                    // match path.remove(&(cx, cy)) {
                    //     Some(mut v) => {
                    //         v.push((mvx, mvy));
                    //         path.insert((cx, cy), v);
                    //     }usize
                    //     None => {
                    //         let v = vec![(mvx, mvy)];
                    //         path.insert((cx, cy), v);
                    //     }
                    // }
                    if cx == dx as i32 && cy == dy as i32 {
                        final_cost = cost;
                        while !pq.is_empty() {
                            pq.pop();
                        }
                        break;
                    }
                    agent.expansions += 1;
                    for (mx, my) in moves {
                        let mxx = mx;
                        let myy = my;
                        let (nx, ny) = (cx + mxx, cy + myy);
                        if !valid(nx, ny, width, height) {
                            continue;
                        }
                        let n_cell = query_cell
                            .get(board.cells[nx as usize][ny as usize])
                            .unwrap();
                        let g: i32 = match n_cell.terrain {
                            Terrain::Grass => 1,
                            Terrain::Mountain => 5,
                            Terrain::Swamp => 10,
                            Terrain::Desert => 20,
                            Terrain::Obstacle => -1,
                        };
                        let n_cost = cost - g - h((nx, ny), (dx as i32, dy as i32));
                        if partial_cost.contains_key(&(nx, ny))
                            && -n_cost >= -partial_cost[&(nx, ny)]
                        {
                            continue;
                        }
                        if path.contains_key(&(nx, ny)) {
                            path.remove(&(nx, ny));
                        }
                        let mut partial_path: Vec<(i32, i32)> = vec![];
                        if path.contains_key(&(cx, cy)) {
                            for (xx, yy) in &path[&(cx, cy)] {
                                partial_path.push((*xx, *yy));
                            }
                        }
                        partial_path.push((mxx, myy));
                        path.insert((nx, ny), partial_path);
                        pq.push((nx, ny, mxx, myy), n_cost);
                    }
                }
                //Agora precisamos pegar o caminho de "menor" custo
                println!(
                    "\nDistance between ({}, {}) and ({}, {}): {}",
                    ax, ay, dx, dy, -final_cost
                );
                if path.contains_key(&(dx as i32, dy as i32)) {
                    for (mx, my) in &path[&(dx as i32, dy as i32)] {
                        follow_path.moves.push((*mx, *my));
                    }
                    println!("Path: {:?}", path[&(dx as i32, dy as i32)]);
                } else {
                    follow_path.moves.push((0, 0));
                }

                // let time = time::Duration::from_secs_f32(15.0);
                // thread::sleep(time);
            }
        } else {
            println!("Last move was: {:?}", agent.last_move);
            let mut weights: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            let mut has_option = false;
            let mountain_cost = 5.0;
            let swamp_cost = 10.0;
            let desert_cost = 20.0;

            if agent.x < board.height - 1 && agent.last_move != moves[0] {
                let x = agent.x + 1;
                let y = agent.y;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                match cell.terrain {
                    Terrain::Grass => {
                        weights[0] = 1.0;
                        has_option = true;
                    }
                    Terrain::Mountain => {
                        weights[0] = 1.0 / mountain_cost;
                        has_option = true;
                    }
                    Terrain::Swamp => {
                        weights[0] = 1.0 / swamp_cost;
                        has_option = true;
                    }
                    Terrain::Desert => {
                        weights[0] = 1.0 / desert_cost;
                        has_option = true;
                    }
                    _ => {}
                }
            }
            if agent.y < board.width - 1 && agent.last_move != moves[1] {
                let x = agent.x;
                let y = agent.y + 1;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                match cell.terrain {
                    Terrain::Grass => {
                        weights[1] = 1.0;
                        has_option = true;
                    }
                    Terrain::Mountain => {
                        weights[1] = 1.0 / mountain_cost;
                        has_option = true;
                    }
                    Terrain::Swamp => {
                        weights[1] = 1.0 / swamp_cost;
                        has_option = true;
                    }
                    Terrain::Desert => {
                        weights[1] = 1.0 / desert_cost;
                        has_option = true;
                    }
                    _ => {}
                }
            }
            if agent.x > 0 && agent.last_move != moves[2] {
                let x = agent.x - 1;
                let y = agent.y;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                match cell.terrain {
                    Terrain::Grass => {
                        weights[2] = 1.0;
                        has_option = true;
                    }
                    Terrain::Mountain => {
                        weights[2] = 1.0 / mountain_cost;
                        has_option = true;
                    }
                    Terrain::Swamp => {
                        weights[2] = 1.0 / swamp_cost;
                        has_option = true;
                    }
                    Terrain::Desert => {
                        weights[2] = 1.0 / desert_cost;
                        has_option = true;
                    }
                    _ => {}
                }
            }
            if agent.y > 0 && agent.last_move != moves[3] {
                let x = agent.x;
                let y = agent.y - 1;
                let cell = query_cell.get(board.cells[x][y]).unwrap();
                match cell.terrain {
                    Terrain::Grass => {
                        weights[3] = 1.0;
                        has_option = true;
                    }
                    Terrain::Mountain => {
                        weights[3] = 1.0 / mountain_cost;
                        has_option = true;
                    }
                    Terrain::Swamp => {
                        weights[3] = 1.0 / swamp_cost;
                        has_option = true;
                    }
                    Terrain::Desert => {
                        weights[3] = 1.0 / desert_cost;
                        has_option = true;
                    }
                    _ => {}
                }
            }

            if has_option {
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();
                let movement = moves[dist.sample(&mut rng)];
                agent.last_move = movement;
                agent.last_move.0 *= -1;
                agent.last_move.1 *= -1;
                let new_x: usize = (agent.x as i32 + movement.0) as usize;
                let new_y: usize = (agent.y as i32 + movement.1) as usize;
                agent.x = new_x;
                agent.y = new_y;
                let cell = query_cell.get(board.cells[new_x][new_y]).unwrap();
                agent.cost += match cell.terrain {
                    Terrain::Grass => 1,
                    Terrain::Mountain => 5,
                    Terrain::Swamp => 10,
                    Terrain::Desert => 20,
                    Terrain::Obstacle => 1e9 as usize,
                };
            }

            let x = agent.x as f32;
            let y = agent.y as f32;
            let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
            let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
            let translation = &mut transform.translation;
            translation.x = cx;
            translation.y = cy;
        }
    }
}

pub fn follow_path(
    windows: Res<Windows>,
    board: Res<Board>,
    mut follow_path: ResMut<Path>,
    mut query: Query<(&mut Agent, &mut Transform)>,
    mut query_cell: Query<&mut Cell>,
    mut _params: ResMut<Params>,
    mut query_tool: Query<&mut Tool>,
) {
    if !follow_path.moves.is_empty() {
        println!("Current follow path size: {}", follow_path.moves.len());
        let time = time::Duration::from_secs_f32(0.1);
        // thread::sleep(time);
        let window = windows.primary();
        let border_width = 2.0;
        let cell_width =
            (window.width() - border_width * (board.width - 1) as f32) / (board.width as f32);
        let cell_height =
            (window.height() - border_width * (board.height - 1) as f32) / (board.height as f32);

        let (mut agent, mut transform) = query.get_single_mut().unwrap();
        let (mx, my) = follow_path.moves.remove(0);
        if !(mx == 0 && my == 0) {
            agent.x = (agent.x as i32 + mx) as usize;
            agent.y = (agent.y as i32 + my) as usize;

            let mut requisitions: Vec<Factory> = vec![];
            for requisition in &agent.requisitions {
                requisitions.push(*requisition);
            }

            check_radius(
                &board,
                agent.x as i32,
                agent.y as i32,
                agent.radius as i32,
                &mut agent.destination_queue,
                &requisitions,
                &query_cell,
            );

            let x = agent.x as f32;
            let y = agent.y as f32;
            let cx = -window.width() / 2. + cell_width * x + border_width * x + cell_width / 2.;
            let cy = -window.height() / 2. + cell_height * y + border_width * y + cell_height / 2.;
            let translation = &mut transform.translation;
            translation.x = cx;
            translation.y = cy;

            let x = agent.x;
            let y = agent.y;
            let mut cell = query_cell.get_mut(board.cells[x][y]).unwrap();

            agent.cost += match cell.terrain {
                Terrain::Grass => 1,
                Terrain::Mountain => 5,
                Terrain::Swamp => 10,
                Terrain::Desert => 20,
                Terrain::Obstacle => 1e9 as usize,
            };

            match cell.tool {
                Some(ToolType::Battery) => {
                    agent.state[0].1 += 1;
                    println!("Got 1 Battery at {} {}", x, y);
                    cell.tool = None;
                }
                Some(ToolType::WeldingArm) => {
                    agent.state[1].1 += 1;
                    println!("Got 1 Welding Arm at {} {}", x, y);
                    cell.tool = None;
                }
                Some(ToolType::SuctionPump) => {
                    agent.state[2].1 += 1;
                    println!("Got 1 Suction Pump at {} {}", x, y);
                    cell.tool = None;
                }
                Some(ToolType::CoolingDevice) => {
                    agent.state[3].1 += 1;
                    println!("Got 1 Cooling Device at {} {}", x, y);
                    cell.tool = None;
                }
                Some(ToolType::PneumaticArm) => {
                    agent.state[4].1 += 1;
                    println!("Got 1 Pneumatic Arm at {} {}", x, y);
                    cell.tool = None;
                }
                None => {}
            }
            for mut tool in query_tool.iter_mut() {
                if tool.x == x && tool.y == y {
                    tool.tool_type = None;
                }
            }
            match cell.factory {
                Some(mut factory) => {
                    let mut batteries = 0;
                    let mut welding_arms = 0;
                    let mut pumps = 0;
                    let mut cooling_devices = 0;
                    let mut pneumatic_arms = 0;
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
                    let fx = factory.x;
                    let fy = factory.y;
                    match factory.needed_tool {
                        Some(ToolType::Battery) => {
                            if batteries >= factory.quantity {
                                println!("Drop {} batteries at ({} {})", factory.quantity, fx, fy);
                                agent.state[0].1 -= factory.quantity;
                                factory.quantity = 0;
                                factory.needed_tool = None;
                            }
                        }
                        Some(ToolType::WeldingArm) => {
                            if welding_arms >= factory.quantity {
                                println!(
                                    "Drop {} welding arms at ({} {})",
                                    factory.quantity, fx, fy
                                );
                                agent.state[1].1 -= factory.quantity;
                                factory.quantity = 0;
                                factory.needed_tool = None;
                            }
                        }
                        Some(ToolType::SuctionPump) => {
                            if pumps >= factory.quantity {
                                println!(
                                    "Drop {} suction pumps at ({} {})",
                                    factory.quantity, fx, fy
                                );
                                agent.state[2].1 -= factory.quantity;
                                factory.quantity = 0;
                                factory.needed_tool = None;
                            }
                        }
                        Some(ToolType::CoolingDevice) => {
                            if cooling_devices >= factory.quantity {
                                println!(
                                    "Drop {} cooling devices at ({} {})",
                                    factory.quantity, fx, fy
                                );
                                agent.state[3].1 -= factory.quantity;
                                factory.quantity = 0;
                                factory.needed_tool = None;
                            }
                        }
                        Some(ToolType::PneumaticArm) => {
                            if pneumatic_arms >= factory.quantity {
                                println!(
                                    "Drop {} pneumatic arms at ({} {})",
                                    factory.quantity, fx, fy
                                );
                                agent.state[4].1 -= factory.quantity;
                                factory.quantity = 0;
                                factory.needed_tool = None;
                            }
                        }
                        None => {}
                    }
                    if agent.destination_queue.contains(&(fx, fy)) {
                        let mut idx = 0;
                        for (dx, dy) in &agent.destination_queue {
                            if *dx == fx && *dy == fy {
                                break;
                            }
                            idx += 1;
                        }
                        agent.destination_queue.remove(idx);
                    }
                    for fact in &mut agent.requisitions {
                        let fxx = fact.x;
                        let fyy = fact.y;
                        if fx == fxx && fy == fyy {
                            fact.needed_tool = None;
                            fact.quantity = 0;
                        }
                    }
                }
                None => {}
            }
        }
    }
}
