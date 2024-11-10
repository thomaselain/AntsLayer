mod pathfinding;

use colored::{ColoredString, Colorize};

use rand::seq::SliceRandom;
use rand::{self, Rng};
use sdl2::pixels::Color;

use crate::buildings::{Building, BuildingType, FindHome};
use crate::coords::{self, Coords};
use crate::terrain::{self, Terrain};
use crate::terrain::{Mineral, MineralType, TileType};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RaceType {
    HUMAN,
    ANT,
    ALIEN,
}
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
    pub fn get_thinking_speed(self) -> i32 {
        match self {
            RaceType::HUMAN => 150,
            RaceType::ANT => 25,
            RaceType::ALIEN => 75,
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
        match self {
            JobType::MINER(mineral_type) => {
                if let Some(closest) = mineral_type.find_closest(
                    TileType::Mineral(mineral_type),
                    terrain,
                    unit.clone(),
                ) {
                    (ActionType::DIG, closest)
                } else {
                    (ActionType::WANDER, unit.coords)
                }
            }
            _ => (ActionType::WANDER, unit.coords),
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
    fn do_now(&mut self, action: ActionType, coords: Coords) {
        self.insert(0, (action, coords));
    }
    fn do_later(&mut self, actions: ActionType, coords: Coords) {
        self.push((actions, coords));
    }
    //wip
    fn remove_only(&mut self, actions: Vec<ActionType>) {
        for to_remove in actions {
            self.retain_mut(|(what, coords)| (*what, *coords) == (to_remove, *coords));
        }
    }
    //wip
    fn keep_only(&mut self, actions: Vec<ActionType>) {
        for to_keep in actions {
            self.retain_mut(|(what, coords)| (*what, *coords) != (to_keep, *coords));
        }
    }
}
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

#[doc = "Unit.speed is thinking speed (in milliseconds) not moving speed"]
#[derive(Clone)]
pub struct Unit {
    pub color: u32,
    pub job: JobType,
    pub race: RaceType,
    pub coords: Coords,
    pub action_queue: Vec<(ActionType, Coords)>,
    pub action_path: Option<(Vec<(usize, usize)>, i32)>,
    pub speed: i32,
    pub last_action_timer: i32,
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
        let job = match rng.gen_range(0..=6) {
            1 => JobType::MINER(MineralType::IRON),
            2 => JobType::MINER(MineralType::GOLD),
            3 => JobType::MINER(MineralType::ROCK),
            4 => JobType::BUILDER,
            5 => JobType::FARMER,
            6 => JobType::FIGHTER,
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
            last_action_timer: 0,
            speed: race.get_thinking_speed(),
        }
    }

    /// Decide what to do next
    pub fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        self.last_action_timer += delta_time;
        if self.last_action_timer >= self.speed {
            match self.clone().action_queue.first() {
                Some((ActionType::MOVE, coords)) => {
                    if self.r#move(terrain, *coords).is_none() {
                        self.action_queue.remove(0);
                        self.action_queue.push((ActionType::WANDER, self.coords));
                    }
                }
                Some((ActionType::DIG, coords)) => {
                    if self.coords.distance_in_tiles(coords) > 1 {
                        self.action_queue.insert(0, (ActionType::MOVE, *coords));
                    } else {
                        let actions = self.dig(terrain, coords);
                        if !actions.is_none() {
                            self.action_queue.remove(0); // Action DIG complétée
                            self.action_queue.append(&mut actions.unwrap());
                        }
                    }
                }
                Some((ActionType::WANDER, coords)) => {
                    let home = terrain
                        .buildings
                        .find_building(self.race, self.job, terrain);

                    match home {
                        Some(coords) => {
                            if self.coords.distance_in_tiles(&coords) >= 2
                                && self.action_queue.len() == 1
                            {
                                self.action_path = None;
                                self.action_queue.push((ActionType::MOVE, coords));
                            } else if self.action_queue.len() == 1 {
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
                        None => {}
                    }
                }

                Some((ActionType::BUILD, _)) => {}

                Some((ActionType::WAIT, _)) => {
                    self.action_queue.pop();
                    println!("Waiting ...");
                }
                Some((_, _)) => {}
                None => {}
            }
            self.last_action_timer = 0;
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
        let start = self.coords.to_tuple();
        let goal = coords.to_tuple();

        // Étape 2 : Si la dernière case walkable n'est pas la destination, commencer à creuser
        let Some((mut dig_path, _cost)) =
            self.find_path(self.coords.to_tuple(), goal, terrain.clone(), None)
        else {
            return None;
        };

        if !dig_path.is_empty() {
            if self.coords.distance_in_tiles(&Coords {
                x: dig_path.first_mut().unwrap().0 as i32,
                y: dig_path.first_mut().unwrap().1 as i32,
            }) < 2
            {
                terrain.dig_radius(coords, 0);
                println!("Digging at {:?}", coords);
            }

            // Ajouter le chemin pour le creusage à partir de la dernière case walkable
            let actions = dig_path
                .into_iter()
                .skip(2) // On saute la position actuelle
                .map(|(x, y)| {
                    (
                        ActionType::DIG,
                        Coords {
                            x: x as i32,
                            y: y as i32,
                        },
                    )
                })
                .collect::<Vec<_>>();
            // Si assez proche pour creuser

            return Some(actions);
        }

        None // Aucun chemin trouvé
    }
    pub fn build(&self) {
        todo!("unit.build")
    }
}
