mod pathfinding;

use colored::{ColoredString, Colorize};

use rand::seq::SliceRandom;
use rand::{self, Rng};

use crate::buildings::FindBuilding;
use crate::coords::Coords;
use crate::terrain::{self, Terrain};
use crate::terrain::{MineralType, TileType};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RaceType {
    HUMAN,
    ANT,
    ALIEN,
}

/// methods mainly for debug purposes
impl RaceType {
    pub fn to_colored_string(self) -> ColoredString {
        match self {
            RaceType::ALIEN => "ALIEN".green(),
            RaceType::ANT => "ANT".red(),
            RaceType::HUMAN => "HUMAN".blue(),
        }
    }
    pub fn to_string(self) -> String {
        match self {
            RaceType::ALIEN => "ALIEN".to_string(),
            RaceType::ANT => "ANT".to_string(),
            RaceType::HUMAN => "HUMAN".to_string(),
        }
    }
    pub fn to_u32(self) -> u32 {
        match self {
            RaceType::ANT => 0xff0000ff,
            RaceType::ALIEN => 0x00ff00ff,
            RaceType::HUMAN => 0x0000ffff,
        }
    }
    /// The lower the value, the higher unit thinks
    /// ANTS   : Slow thinking speed
    /// HUMANS : Quick thinking speed
    /// ALIENS : Medium thinking speed
    pub fn get_thinking_speed(self) -> i32 {
        match self {
            RaceType::HUMAN => 1000,
            RaceType::ANT => 250,
            RaceType::ALIEN => 150,
        }
    }

    /// The lower the value, the higher unit moves
    /// ANTS   : Quick moving speed
    /// HUMANS : Slow moving speed
    /// ALIENS : Medium moving speed
    pub fn get_moving_speed(self) -> i32 {
        match self {
            RaceType::HUMAN => 100,
            RaceType::ANT => 200,
            RaceType::ALIEN => 100,
        }
    }

    /// The lower the value, the higher the cost
    /// ANTS   : prefer straight lines
    /// HUMANS : don't care
    /// ALIENS : prefer diagonals
    fn diagonal_cost(self, is_diagonal: bool) -> i32 {
        match self {
            RaceType::HUMAN => {
                if is_diagonal {
                    1
                } else {
                    1
                }
            }
            RaceType::ANT => {
                if is_diagonal {
                    2
                } else {
                    1
                }
            }
            RaceType::ALIEN => {
                if is_diagonal {
                    1
                } else {
                    2
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JobType {
    MINER(MineralType),
    JOBLESS,
    FARMER,
    FIGHTER,
    BUILDER,
}

impl JobType {
    pub fn get_action(self, terrain: &Terrain, unit: &Unit) -> (ActionType, Coords) {
        let dest = if let Some(dest) = terrain.buildings.find_building(unit.race, self, terrain) {
            dest
        } else {
            unit.coords
        };
        match self {
            JobType::MINER(mineral_type) => {
                if let Some(closest) = mineral_type.find_closest(
                    TileType::Mineral(mineral_type),
                    terrain,
                    unit.clone(),
                ) {
                    (ActionType::DIG, closest)
                } else {
                    (ActionType::WANDER, dest)
                }
            }
            _ => (ActionType::WANDER, dest),
        }
    }

    pub fn get_miner_target(self) -> Option<MineralType> {
        match self {
            JobType::MINER(mineral_type) => Some(mineral_type),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionType {
    WANDER,
    WAIT,
    MOVE,
    HAUL,
    DIG,
    EAT,
    SLEEP,
    FIGHT,
    BUILD,
}

impl ActionType {
    pub fn to_str(self) -> String {
        match self {
            ActionType::WANDER => "WANDER".to_string(),
            ActionType::WAIT => "WAIT".to_string(),
            ActionType::MOVE => "MOVE".to_string(),
            ActionType::HAUL => "HAUL".to_string(),
            ActionType::DIG => "DIG".to_string(),
            ActionType::EAT => "EAT".to_string(),
            ActionType::SLEEP => "SLEEP".to_string(),
            ActionType::FIGHT => "FIGHT".to_string(),
            ActionType::BUILD => "BUILD".to_string(),
        }
    }
}

pub trait ActionQueue {
    fn do_now(&mut self, action: ActionType, coords: Coords);
    fn do_later(&mut self, action: ActionType, coords: Coords);
    fn remove_only(&mut self, action: Vec<ActionType>);
    fn keep_only(&mut self, action: Vec<ActionType>);
}

impl ActionQueue for Vec<(ActionType, Coords)> {
    /// Add action at the beginning of self.actiono_queue
    fn do_now(&mut self, action: ActionType, coords: Coords) {
        self.insert(0, (action, coords));
    }
    /// Add action at the end of self.actiono_queue
    fn do_later(&mut self, actions: ActionType, coords: Coords) {
        self.push((actions, coords));
    }
    /// Remove actions in self.action_queue that match any action in actions
    fn remove_only(&mut self, actions: Vec<ActionType>) {
        for to_remove in actions {
            self.retain_mut(|(what, coords)| (*what, *coords) == (to_remove, *coords));
        }
    }
    /// Remove actions in self.action_queue that match no action in actions
    fn keep_only(&mut self, actions: Vec<ActionType>) {
        for to_keep in actions {
            self.retain_mut(|(what, coords)| (*what, *coords) != (to_keep, *coords));
        }
    }
}

/// Cute display in terminal :)
impl ActionType {
    pub fn get_ascii(self) -> ColoredString {
        match self {
            ActionType::MOVE => "-M>".bold().strikethrough().blue(),
            ActionType::DIG => " D ".bold().red(),
            ActionType::WANDER => " ? ".italic().green(),
            ActionType::WAIT => "...".italic().bright_green(),
            _ => " ".into(),
        }
    }
}

#[derive(Clone)]
pub struct Unit {
    pub color: u32,
    pub job: JobType,
    pub race: RaceType,
    pub coords: Coords,
    pub action_queue: Vec<(ActionType, Coords)>,
    pub action_path: Option<(Vec<(usize, usize)>, i32)>,
    pub thinking_speed: i32,
    pub moving_speed: i32,
    pub last_action_timer: i32,
    pub last_move_timer: i32,
}

pub fn display_action_queue(current_race: RaceType, unit: Unit) {
    if current_race != unit.race {
        return;
    }
    print!("{} : ", unit.race.to_colored_string());

    for action in unit.action_queue {
        print!("[{}]", action.0.get_ascii());
    }
    println!("");
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
        let race = match rng.gen_range(1..=3) {
            1 => RaceType::HUMAN,
            2 => RaceType::ANT,
            3 => RaceType::ALIEN,
            _ => RaceType::ANT,
        };
        let mut rng = rand::thread_rng();
        let job = match rng.gen_range(0..=3) {
            1 => JobType::MINER(MineralType::IRON),
            2 => JobType::MINER(MineralType::GOLD),
            3 => JobType::MINER(MineralType::ROCK),
            //         4 => JobType::BUILDER,
            //         5 => JobType::FARMER,
            //         6 => JobType::FIGHTER,
            _ => JobType::JOBLESS,
        };

        println!(
            "New Unit (x : {:?} | y : {:?}) --> {:?}",
            coords.x,
            coords.y,
            race.to_string()
        );

        Unit {
            color: race.to_u32(),
            race,
            job,
            coords,
            action_queue: vec![],
            action_path: None,
            last_action_timer: race.get_thinking_speed(),
            last_move_timer: race.get_moving_speed(),
            thinking_speed: race.get_thinking_speed(),
            moving_speed: race.get_moving_speed(),
        }
    }

    /// Decide what to do next
    pub fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        self.last_action_timer += delta_time;
        self.last_move_timer += delta_time;
        match self.clone().action_queue.first() {
            Some((ActionType::MOVE, coords)) => {
                if self.last_move_timer >= self.moving_speed {
                    if self.r#move(terrain, *coords).is_none() {
                        self.action_queue.remove(0);
                        self.action_queue.push((ActionType::WANDER, self.coords));
                    }
                    self.last_move_timer = 0;
                }
            }
            Some((ActionType::DIG, coords)) => {
                if self.last_action_timer >= self.moving_speed {
                    if self.coords.distance_in_tiles(coords) > 1 {
                        // self.action_queue.clear();
                        self.action_queue.do_now(ActionType::MOVE, *coords);
                    } else {
                        let actions = self.dig(terrain, coords);
                        if !actions.is_none() && !actions.clone().unwrap().len() > 1 {
                            //   self.action_queue.remove(0);
                            self.action_queue.append(&mut actions.unwrap());
                        }
                    }
                    self.last_action_timer = 0;
                }
            }
            Some((ActionType::WANDER, _)) => {
                let home = terrain
                    .buildings
                    .find_building(self.race, self.job, terrain);

                match home {
                    Some(coords) => {
                        if self.last_action_timer >= self.thinking_speed {
                            if self.coords.distance_in_tiles(&coords) > 5
                                && self.action_queue.len() == 1
                            {
                                self.action_path = None;
                                self.action_queue.remove(0);
                                self.action_queue.do_now(ActionType::MOVE, coords);
                            } else if self.action_queue.len() < 2 {
                                self.r#move(
                                    terrain,
                                    Coords {
                                        x: self.coords.x + random_direction(),
                                        y: self.coords.y + random_direction(),
                                    },
                                );
                            } else {
                                self.action_queue.remove(0);
                                self.action_queue
                                    .insert(0, self.job.get_action(&terrain, &self));
                            }
                        }
                    }
                    None => {}
                }
            }

            Some((ActionType::BUILD, _)) => {}

            Some((ActionType::WAIT, _)) => {
                self.action_queue.pop();
                println!("Waiting ...");
            }

            Some((_, _)) => {
            }
            None => {}
        }
    }

    //move
    pub fn r#move(&mut self, terrain: &mut Terrain, m: Coords) -> Option<()> {
        let start = self.coords.to_tuple();
        let goal = m.to_tuple();

        // Tentative avec le pathfinding pour MOVE uniquement
        if let Some(mut path) = self.find_path(start, goal, terrain.clone(), Some(ActionType::MOVE))
        {
            if path.0.len() > 1 {
                let next_coords = Coords {
                    x: path.0[1].0 as i32,
                    y: path.0[1].1 as i32,
                };
                if terrain.is_walkable(next_coords.x as usize, next_coords.y as usize) {
                    self.coords = next_coords;
                    self.action_path = Some(path);
                    return Some(());
                }
            }
        }

        // Si échec, tentative avec pathfinding général (None)
        if let Some(mut path) = self.find_path(start, goal, terrain.clone(), None) {
            if path.0.len() > 1 {
                let next_coords = Coords {
                    x: path.0[1].0 as i32,
                    y: path.0[1].1 as i32,
                };
                if terrain.is_walkable(next_coords.x as usize, next_coords.y as usize) {
                    self.coords = next_coords;
                    self.action_path = Some(path);
                    return Some(());
                }
            }
        }

        None // Aucun chemin possible
    }

    pub fn dig(
        &mut self,
        terrain: &mut Terrain,
        coords: &Coords,
    ) -> Option<Vec<(ActionType, Coords)>> {
        let goal = coords;
        let start = if let Some(start) = terrain
            .buildings
            .find_building(self.race, self.job, terrain)
        {
            start
        } else {
            self.coords
        };

        if let Some((mut dig_path, _)) =
            self.find_path(start.to_tuple(), goal.to_tuple(), terrain.clone(), None)
        {
            if !dig_path.is_empty() {
                let next_pos = Coords {
                    x: dig_path.first_mut().unwrap().0 as i32,
                    y: dig_path.first_mut().unwrap().1 as i32,
                };
                if let Some(tile) = terrain.get_data_from_coords(*goal) {
                    if tile.is_diggable() {
                        if start.distance_in_tiles(&next_pos) < 2 {
                            //self.action_queue.do_now(ActionType::DIG, next_pos);
                            // Si assez proche pour creuser
                            terrain.dig_radius(&goal, 0);
                            println!("Digging at {:?}", next_pos);
                        }
                    } else {
                        self.action_queue.remove(0);
                        // Ajouter le chemin pour le creusage à partir de la dernière case walkable
                        return Some(
                            vec![(
                                dig_path
                                    .into_iter()
                                    .last() // On saute la position actuelle
                                    .map(|(x, y)| {
                                        (
                                            ActionType::DIG,
                                            Coords {
                                                x: x as i32,
                                                y: y as i32,
                                            },
                                        )
                                    })
                                    .unwrap()
                                    .0,
                                next_pos,
                            )], //  .collect::<Vec<_>>(),
                        );
                    }
                }
            }
        }
        None
    }

    pub fn build(&self) {
        todo!("unit.build")
    }
}
