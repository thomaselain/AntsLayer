pub(crate) mod jobs;
pub(crate) mod actions;
pub(crate) mod inventory;
mod pathfinding;

use std::i32;
use actions::{ display_action_queue, Action, ActionQueue, ActionType };
use colored::{ ColoredString, Colorize };
use inventory::{ Inventory, Item };
use jobs::JobType;
use rand::seq::SliceRandom;
use rand::{ self, Rng };
use coords::Coords;

use crate::game::map::tile::minerals::MineralType;

use super::map::tile::buildings::BuildingType;
use super::map::tile::{ Tile, TileType };
use super::map::{ Map, HEIGHT, WIDTH };

pub const HOME_STARTING_SIZE: u32 = 15;

#[derive(Clone)]
pub struct Unit {
    pub inventory: Inventory,
    pub color: u32,
    pub job: JobType,
    pub race: RaceType,
    pub coords: Coords,
    pub action_queue: Vec<(ActionType, Coords)>,
    pub path: Vec<(usize, usize)>,
    pub thinking_speed: i32,
    pub moving_speed: i32,
    pub last_action_timer: i32,
    pub last_move_timer: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RaceType {
    ANT,
    HUMAN,
    ALIEN,
}

/// Returns a number in [-1,0,1]
/// Used only in WANDER action
fn random_direction() -> i32 {
    let choices = [-1, 0, 1];
    *choices.choose(&mut rand::thread_rng()).unwrap()
}

/// methods mainly for debug purposes
impl RaceType {
    pub fn starting_coords(self) -> Coords {
        match self {
            RaceType::ANT => Coords((WIDTH as i32) / 2, (HEIGHT as i32) / 2),
            RaceType::HUMAN =>
                Coords(
                    (HOME_STARTING_SIZE as usize)
                        .try_into()
                        .expect("HUMAN Hearth is out of bounds !"),
                    (HOME_STARTING_SIZE as usize)
                        .try_into()
                        .expect("HUMAN Hearth is out of bounds !")
                ),
            RaceType::ALIEN =>
                Coords(
                    (WIDTH - (HOME_STARTING_SIZE as usize))
                        .try_into()
                        .expect("ALIEN Hearth is out of bounds !"),
                    (HEIGHT - (HOME_STARTING_SIZE as usize)).try_into().unwrap()
                ),
        }
    }
    pub fn to_colored_string(self) -> ColoredString {
        match self {
            RaceType::ALIEN => "ALIEN".green(),
            RaceType::ANT => "ANT".red(),
            RaceType::HUMAN => "HUMAN".blue(),
        }
    }
    pub fn to_u32(self) -> u32 {
        match self {
            RaceType::ANT => 0xff0000ff,
            RaceType::ALIEN => 0x00ff00ff,
            RaceType::HUMAN => 0x0000ffff,
        }
    }
    pub fn patience(self) -> usize {
        match self {
            RaceType::HUMAN => 10,
            RaceType::ANT => 10,
            RaceType::ALIEN => 10,
        }
    }
    /// Amount of Items a Unit can carry
    pub fn get_carry_capacity(self) -> i32 {
        match self {
            RaceType::HUMAN => 5,
            RaceType::ANT => 5,
            RaceType::ALIEN => 5,
        }
    }

    pub fn get_thinking_speed(self) -> i32 {
        match self {
            RaceType::HUMAN => 1000,
            RaceType::ANT => 250,
            RaceType::ALIEN => 150,
        }
    }

    pub fn get_moving_speed(self) -> i32 {
        match self {
            RaceType::HUMAN => 100,
            RaceType::ANT => 25,
            RaceType::ALIEN => 50,
        }
    }

    fn diagonal_cost(self, is_diagonal: bool) -> i32 {
        match self {
            RaceType::HUMAN => {
                if is_diagonal { 0 } else { 0 }
            }
            RaceType::ANT => {
                if is_diagonal { 5 } else { 0 }
            }
            RaceType::ALIEN => {
                if is_diagonal { 0 } else { 5 }
            }
        }
    }
}

impl Unit {
    pub fn new(race_type: Option<RaceType>) -> Unit {
        let mut rng = rand::thread_rng();

        let race = if race_type.is_some() {
            race_type.unwrap()
        } else {
            match rng.gen_range(1..=3) {
                1 => RaceType::HUMAN,
                2 => RaceType::ANT,
                _ => RaceType::ALIEN,
            }
        };

        let mut rng = rand::thread_rng();
        let job = match rng.gen_range(0..=5) {
            1 => JobType::MINER(TileType::Mineral(MineralType::ROCK)),
            2 => JobType::MINER(TileType::Mineral(MineralType::IRON)),
            _ => JobType::MINER(TileType::Mineral(MineralType::MOSS)),
            4 => JobType::MINER(TileType::Mineral(MineralType::DIRT)),
            5 => JobType::MINER(TileType::Mineral(MineralType::GOLD)),
            6 => JobType::BUILDER,
            //         5 => JobType::FARMER,
            //         6 => JobType::FIGHTER,
            _ => JobType::JOBLESS,
        };

        println!("A {:?} was born", race);
        if job == JobType::JOBLESS {
            println!("{:#?}", job);
        } else {
            println!("With job : {:#?}", job);
        }

        Unit {
            inventory: Inventory::new(race.get_carry_capacity() as u32),
            color: race.to_u32(),
            race,
            job,
            coords: race.starting_coords(),
            action_queue: vec![],
            path: vec![],
            last_action_timer: race.get_thinking_speed(),
            last_move_timer: race.get_moving_speed(),
            thinking_speed: race.get_thinking_speed(),
            moving_speed: race.get_moving_speed(),
        }
    }

    // Decide what to do next
    pub fn think(
        &mut self,
        map: &mut Map,
        delta_time: i32
    ) -> Result<(ActionType, Coords), Coords> {
        self.last_action_timer += delta_time;
        self.last_move_timer += delta_time;

        // Check what the first action to do is
        match (self.job, self.clone().action_queue.first()) {
            // MOVE Action
            (_, Some((ActionType::MOVE, coords))) => {
                if self.last_move_timer >= self.moving_speed {
                    if self.r#move(map, *coords).is_none() {
                        self.action_queue.clear();
                        self.action_queue.push((ActionType::WANDER, self.coords));
                    }
                    self.last_move_timer = 0;
                }
            }
            //                  ////////////////////////////
            //                  // Default miner behavior //
            //                  ////////////////////////////
            //
            //                     first item in inventory
            //                         is a mineral ?
            //
            //                 /                            \
            //                /                              \
            //               Yes                              No
            //                |                                |
            //          Haul to the closest                Do nothing
            //     Stockpile of your first item          for this match
            //
            //

            // DIG action only Miners can do
            // adds MOVE actions until mineral is reached
            // Starts digging (behavior is kind of random, sometimes they dig deep, sometimes not idk...)
            (JobType::MINER(tile_type), Some((ActionType::DIG, coords))) => {
                if self.last_action_timer >= self.moving_speed {
                    let dig_at = self.find_job_action(map)?;
                    // println!("{:?}", dig_at);
                    self.last_action_timer = 0;
                    self.dig(map, coords)?;
                }
            }
            // Idle around self.race's Hearth or Stockpile, depending on units job
            (_, Some((ActionType::WANDER, coords))) => {
                if self.last_action_timer >= self.thinking_speed {
                    if
                        self.coords.distance_to(&coords) > (HOME_STARTING_SIZE as f64) / 2.0 &&
                        self.action_queue.len() == 1
                    {
                        self.path.clear();
                        //self.action_queue.clear();
                        //self.action_queue.do_now(ActionType::MOVE, *coords);
                    } else if self.action_queue.len() < 2 {
                        self.r#move(
                            map,
                            Coords(
                                self.coords.x() + random_direction(),
                                self.coords.y() + random_direction()
                            )
                        );
                    } else {
                        self.action_queue.do_now(Action(ActionType::MOVE, self.coords));
                    }
                }
            }

            // If stockpile has enough mats, bring them to building spot (coords)
            // Otherwise, if next action is WANDER, do nothing
            (_, Some((ActionType::BUILD, coords))) => {}

            // Do nothing
            (_, Some((ActionType::WAIT, _))) => {
                self.action_queue.pop();
                println!("Waiting ...");
            }

            // ??? Should not reach yet
            (JobType::MINER(tile_type), Some((ActionType::HAUL, _))) => {
                if self.last_action_timer >= self.thinking_speed {
                    //println!("Hauling ... ");
                    // Look for a stockpile
                    //
                    //
                    //
                    //
                    let mut count: usize = self.race.patience();
                    'patience: loop {
                        // Attempt to find a stockpile to go to
                        if
                            let Ok(stockpile) = map.find_closest_building(
                                self.coords,
                                BuildingType::Stockpile(
                                    self.job.get_miner_target().ok().expect("Should be mineral")
                                )
                            )
                        {
                            self.action_queue.do_next(Action(ActionType::MOVE, stockpile));
                        }

                        if count == 0 {
                            break 'patience;
                        } else {
                            count -= 1;
                        }
                    }
                }
            }
            (_, _) => {
                return Err(self.coords);
            }
        }
        let mut default_action = (ActionType::WANDER, self.coords);
        if let Some(go_home) = map.clone().go_to_hearth(self.coords).ok() {
            default_action = go_home;
        }

        Ok(self.action_queue.first().map_or(default_action, |a| *a))
    }

    // //Moves unit in grid and updates its path if a new one is found
    // Some if it was possible
    // None otherwise
    pub fn r#move(&mut self, terrain: &mut Map, m: Coords) -> Option<()> {
        let start = self.coords.to_tuple();
        let goal = m.to_tuple();

        // First, find a path through air
        if
            let Some((path, _)) = self.find_path(
                start,
                goal,
                terrain.clone(),
                Some(ActionType::MOVE)
            )
        {
            if path.len() > 1 {
                let next_coords = Coords(path[1].0 as i32, path[1].1 as i32);
                if terrain.is_walkable(next_coords) {
                    self.coords = next_coords;
                    self.path = path;
                    return Some(());
                }
            }
        }

        // If we can't reach by walking find closest path and go until we cant anymore
        if let Some(path) = self.find_path(start, goal, terrain.clone(), None) {
            if path.0.len() > 1 {
                let next_coords = Coords(path.0[1].0 as i32, path.0[1].1 as i32);
                if terrain.is_walkable(next_coords) {
                    self.coords = next_coords;
                    self.path = path.0;
                    return Some(());
                }
            }
        }

        None // Aucun chemin possible
    }

    // Find a path to goal and dig the closest tile
    pub fn dig(&mut self, map: &mut Map, coords: &Coords) -> Result<Coords, Coords> {
        //     let goal = map.find_closest(coords, tile_type)?;
        let goal = coords;
        let start = self.coords;

        if
            let Some(dig_path) = self.find_path(
                start.to_tuple(),
                goal.to_tuple(),
                map.clone(),
                Some(ActionType::DIG)
            )
        {
            let goal = Coords(
                dig_path.0.last().expect("!!!").0 as i32,
                dig_path.0.last().expect("!!!").1 as i32
            );
            // self.path = dig_path.0;
            self.r#move(map, goal);

            if self.coords.distance_in_tiles(&goal) <= 1 {
                if let Ok(dig) = map.dig_cell(goal) {
                    match (dig.1, self.inventory.clone().is_full()) {
                        (_, true) => {
                            self.action_queue.do_now(Action(ActionType::HAUL, goal));
                        }
                        (Some(mineral), _) => {
                            if
                                let Ok(_) = self.inventory.add(
                                    Item::Mineral(mineral.0.to_tile_type())
                                )
                            {
                                self.action_queue.do_next(Action(ActionType::DIG, goal));
                            } else {
                                self.action_queue.do_now(Action(ActionType::HAUL, goal));
                            }
                        }
                        (_, _) => {}
                    }
                }
            }
            return Ok(goal);
        } else if
            let Some(move_path) = self.find_path(
                start.to_tuple(),
                goal.to_tuple(),
                map.clone(),
                Some(ActionType::MOVE)
            )
        {
            self.path = move_path.0.clone();
            let goal = Coords(
                move_path.0.first().expect("!!!").0 as i32,
                move_path.0.first().expect("!!!").1 as i32
            );

            self.r#move(map, Coords(goal.0, goal.1));
            return Err(Coords(goal.0, goal.1));
        }
        Err(self.coords)
    }

    pub fn build(&self) {
        todo!("unit.build")
    }
}
