use pathfinding::prelude::astar;
use rand::seq::SliceRandom;
use rand::{self, Rng};

use crate::buildings::{Building, FindHome};
use crate::coords::{self, Coords};
use crate::terrain::TileType;
use crate::terrain::{self, Terrain};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RaceType {
    HUMAN,
    ANT,
    ALIEN,
}

#[derive(Copy, Clone)]
pub enum JobType {
    MINER,
    FARMER,
    FIGHTER,
    BUILDER,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionType {
    WANDER,
    WAIT,
    MOVE,
    DIG,
    EAT,
    SLEEP,
    FIGHT,
    BUILD,
}

#[doc = "Unit.speed is thinking speed (in milliseconds) not moving speed"]
#[derive(Clone)]
pub struct Unit {
    pub color: u32,
    pub job: JobType,
    pub race: RaceType,
    pub coords: Coords,
    pub action_queue: Vec<(ActionType, Coords)>,
    action_path: Option<(Vec<(usize, usize)>, i32)>,
    speed: i32,
    last_action_timer: i32,
}

pub fn display_action(action: ActionType, coords: Coords) {
    print!("Going to ");
    print!(
        "{}",
        match action {
            ActionType::DIG => "DIG ",
            ActionType::MOVE => "MOVE ",
            ActionType::WANDER => "WANDER ",
            _ => "do something ",
        }
    );
    println!("at ({:?},{:?}) !", coords.x, coords.y);
}
fn random_direction() -> i32 {
    let choices = [-1, 0, 1];
    *choices.choose(&mut rand::thread_rng()).unwrap()
}

impl Unit {
    pub fn new() -> Unit {
        let coords = Coords {
            x: (terrain::WIDTH / 2) as i32,
            y: (terrain::HEIGHT / 2) as i32,
        };
        let mut rng = rand::thread_rng();
        let race = match rng.gen_range(0..=3) {
            1 => RaceType::HUMAN,
            2 => RaceType::ANT,
            3 => RaceType::ALIEN,
            _ => RaceType::ANT,
        };
        let mut rng = rand::thread_rng();
        let job = match rng.gen_range(1..=4) {
            1 => JobType::MINER,
            2 => JobType::BUILDER,
            3 => JobType::FARMER,
            4 => JobType::FIGHTER,
            _ => JobType::MINER,
        };
        let race_type_str = match race {
            RaceType::ALIEN => "ALIEN",
            RaceType::ANT => "ANT",
            RaceType::HUMAN => "HUMAN",
        };

        println!(
            "New Unit (x : {:?} | y : {:?}) --> {:?}",
            coords.x, coords.y, race_type_str
        );

        Unit {
            color: match race {
                RaceType::ANT => 0xff0000ff,
                RaceType::ALIEN => 0x00ff00ff,
                RaceType::HUMAN => 0x0000ffff,
            },
            race,
            job,
            coords,
            action_queue: vec![],
            action_path: None,
            last_action_timer: 0,
            speed: match race {
                RaceType::HUMAN => 75,
                RaceType::ANT => 10,
                RaceType::ALIEN => 35,
            },
        }
    }
    fn get_movement_cost(&self, is_diagonal: bool, action: Option<ActionType>) -> i32 {
        match self.race {
            RaceType::ANT => {
                if let Some(ActionType::DIG) = action {
                    return 5;
                }
                if is_diagonal {
                    10
                } else {
                    10
                }
            }
            RaceType::HUMAN => {
                if let Some(ActionType::DIG) = action {
                    return 10;
                }
                if is_diagonal {
                    25
                } else {
                    0
                }
            }
            RaceType::ALIEN => {
                if let Some(ActionType::DIG) = action {
                    return 3;
                }
                if is_diagonal {
                    0
                } else {
                    25
                }
            }
        }
    }

    fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        terrain: Terrain,
        action: Option<ActionType>,
    ) -> Option<(Vec<(usize, usize)>, i32)> {
        let (path, cost) = astar(
            &start,
            |&(x, y)| {
                // Définir les voisins cardinaux et diagonaux
                let directions = vec![
                    (x + 1, y, false),
                    (x.wrapping_sub(1), y, false),
                    (x, y + 1, false),
                    (x, y.wrapping_sub(1), false),
                    (x + 1, y + 1, true),
                    (x + 1, y.wrapping_sub(1), true),
                    (x.wrapping_sub(1), y + 1, true),
                    (x.wrapping_sub(1), y.wrapping_sub(1), true),
                ];

                directions
                    .into_iter()
                    .filter_map(|(nx, ny, is_diagonal)| match action {
                        Some(ActionType::MOVE) => {
                            if terrain.is_walkable(nx, ny) {
                                Some((
                                    (nx, ny),
                                    self.get_movement_cost(is_diagonal, Some(ActionType::DIG)),
                                ))
                            } else {
                                None
                            }
                        }
                        Some(ActionType::DIG) => {
                            if terrain.is_diggable(nx, ny) || terrain.is_walkable(nx, ny) {
                                Some((
                                    (nx, ny),
                                    self.get_movement_cost(is_diagonal, Some(ActionType::DIG)),
                                ))
                            } else {
                                None
                            }
                        }
                        None => None,
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            },
            |&(x, y)| {
                let dx = (x as isize - goal.0 as isize).abs();
                let dy = (y as isize - goal.1 as isize).abs();
                ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as i32
            }, // Heuristique euclidien
            |&pos| pos == goal,
        )?;

        // Chercher la dernière position valide avant un obstacle
        let mut last_valid_pos = None;
        for (x, y) in path.iter().rev() {
            if terrain.is_walkable(*x, *y) {
                last_valid_pos = Some((*x, *y));
                break; // On arrête dès qu'on trouve la dernière case valide
            }
        }

        // Si une position valide a été trouvée, la retourner avec le chemin et le coût
        if let Some(last_valid) = last_valid_pos {
            Some((path, cost))
        } else {
            // Sinon, renvoyer None si aucune position valide
            None
        }
    }
}

pub trait Actions {
    fn do_action(&mut self, terrain: &mut Terrain);
    fn think(&mut self, terrain: &mut Terrain, delta_time: i32);
    fn r#move(&mut self, terrain: &mut Terrain, m: Coords);
    fn dig(&self);
    fn build(&self);
}

/// ??? Pas sur de ca
impl Actions for Vec<Unit> {
    fn do_action(&mut self, terrain: &mut Terrain) {
        for u in self {
            u.do_action(terrain)
        }
    }
    fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        for u in self {
            u.think(terrain, delta_time)
        }
    }
    fn r#move(&mut self, terrain: &mut Terrain, m: Coords) {
        for u in self {
            u.r#move(terrain, m)
        }
    }
    fn dig(&self) {
        for u in self {
            u.dig()
        }
    }
    fn build(&self) {
        for u in self {
            u.build()
        }
    }
}

impl Actions for Unit {
    fn do_action(&mut self, terrain: &mut Terrain) {
        let action = self.action_queue.first().unwrap();
        match action {
            (ActionType::WAIT, _) => {
                display_action(action.0, action.1);
                match action {
                    (ActionType::WAIT, _) => {}
                    _ => {
                        self.action_queue.remove(0);
                        println!("Waiting ...");
                    }
                }
            }
            (ActionType::MOVE, coords) => {
                display_action(action.0, action.1);
                let start = self.coords.to_tuple();
                let goal = coords.to_tuple();
                if self.action_path == None {
                    let path = self.find_path(start, goal, terrain.clone(), Some(ActionType::MOVE));
                    let dig_path =
                        self.find_path(start, goal, terrain.clone(), Some(ActionType::DIG));
                    match path {
                        Some(_) => {
                            match dig_path {
                                Some(_) => {
                                    // found a way ! --> go !
                                    self.action_path = dig_path;
                                }
                                None => {
                                    // can only go by walking --> ...  !
                                    self.action_path = path;
                                }
                            }
                        }
                        None => {
                            // Cant find path --> cancel move order
                            self.action_queue.remove(0);
                            return;
                        }
                    }
                }
                match self.action_path.clone() {
                    Some(next_move) => {
                        if next_move.0.len() > 1 {
                            let coords = Coords {
                                x: next_move.0[1].0 as i32,
                                y: next_move.0[1].1 as i32,
                            };
                            self.r#move(terrain, coords);
                            let mut path = <Option<(Vec<(usize, usize)>, i32)> as Clone>::clone(
                                &self.action_path,
                            )
                            .unwrap();
                            path.0.remove(0);
                            self.action_path = Some(path);
                        } else {
                            self.action_queue.remove(0);
                            self.action_path = None;
                            self.action_queue.push((ActionType::WANDER, self.coords));
                        }
                    }
                    None => {
                        self.action_queue.clear();
                        self.action_queue.push((ActionType::MOVE, self.coords));
                        println!("No path found to the goal.");
                        // DIG ?
                    }
                }
            }
            (ActionType::DIG, coords) => {
                display_action(action.0, action.1);

                let start = self.coords.to_tuple();
                let goal = coords.to_tuple();

                // Vérifier si la case est creusable avant d'essayer de trouver un chemin
                if !terrain.is_diggable(coords.x as usize, coords.y as usize)
                //   && !terrain.is_walkable(coords.x as usize, coords.y as usize)
                {
                    // Si la case n'est pas creusable, annuler l'action
                    println!("Impossible de creuser ici, action annulée");
                    self.action_queue.remove(0); // Retirer l'action `DIG` de la queue
                    return; // Sortir sans ajouter d'autres actions
                }
                // Si l'unité est déjà à la position cible, on commence directement à creuser
                if self.coords.distance_to(*coords) < 3.0 {
                     terrain.dig_radius(*coords, 2);
                }

                // Chercher un chemin vers la destination
                let path = self.find_path(start, goal, terrain.clone(), Some(ActionType::MOVE));
                let dig_path = self.find_path(start, goal, terrain.clone(), Some(ActionType::DIG));
                match dig_path {
                    Some(_) => match path {
                        Some(_) => {

                            self.action_path = path.clone();
                            self.action_queue.remove(0);

                            self.action_queue.push((
                                ActionType::MOVE,
                                Coords {
                                    x: path.clone().unwrap().0.select_nth_unstable(1).1 .0 as i32,
                                    y: path.clone().unwrap().0.select_nth_unstable(1).1 .1 as i32,
                                },
                            ));
                            self.action_queue.push((
                                ActionType::DIG,
                                Coords {
                                    x: goal.0 as i32,
                                    y: goal.1 as i32,
                                },
                            ));

                            return;
                        }
                        None => {
                            self.action_path = dig_path.clone();
                            self.action_queue.remove(0);

                            self.action_queue.push((
                                ActionType::MOVE,
                                Coords {
                                    x: dig_path.clone().unwrap().0.select_nth_unstable(1).1 .0 as i32,
                                    y: dig_path.clone().unwrap().0.select_nth_unstable(1).1 .1 as i32,
                                },
                            ));

                            self.action_queue.push((
                                ActionType::DIG,
                                Coords {
                                    x: goal.0 as i32,
                                    y: goal.1 as i32,
                                },
                            ));
                            return;
                        }
                    },
                    None => {
                        // self.action_queue.remove(0);
                        return;
                    }
                }
            }

            (ActionType::WANDER, _) => {
                display_action(action.0, action.1);
                let home = terrain.buildings.find_home(self.race, terrain);

                match home {
                    Some(coords) => {
                        if self.coords.distance_to(coords) > 5.0 && self.action_path == None {
                            self.action_queue.remove(0);
                            self.action_queue.push((ActionType::WANDER, coords));
                        } else {
                            self.r#move(
                                terrain,
                                Coords {
                                    x: self.coords.x + random_direction(),
                                    y: self.coords.y + random_direction(),
                                },
                            );
                        }
                    }
                    None => {
                        panic!("WHERE IS MY HOME ???")
                    }
                }
            }

            (ActionType::BUILD, _) => {}
            (_, _) => {}
        }
    }

    /// Decide what to do next
    fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        self.last_action_timer += delta_time;
        if self.last_action_timer >= self.speed {
            if let Some(_) = self.action_queue.first() {
                self.do_action(terrain);
                self.last_action_timer = 0;
            }
        }
    }
    //move
    fn r#move(&mut self, terrain: &mut Terrain, m: Coords) {
        if terrain.get_data(m.x as usize, m.y as usize) == Some(TileType::AIR) {
            self.coords.x = m.x as i32;
            self.coords.y = m.y as i32;
        }
    }
    fn dig(&self) {
        todo!("unit.dig")
    }
    fn build(&self) {
        todo!("unit.build")
    }
}
