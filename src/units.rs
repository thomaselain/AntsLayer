mod pathfinding;

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

pub fn display_action_queue(unit: Unit) {
    print!("ACTION LIST : ");
    for action in unit.action_queue {
        print!(
            "{}",
            match action.0 {
                ActionType::MOVE => "M",
                ActionType::DIG => "D",
                ActionType::WANDER => "W",
                _ => " ",
            }
        );
        print!(" <-- ");
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
                RaceType::HUMAN => 100,
                RaceType::ANT => 25,
                RaceType::ALIEN => 50,
            },
        }
    }

    /// Decide what to do next
    pub fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        self.last_action_timer += delta_time;
        if self.last_action_timer >= self.speed {
            display_action_queue(self.clone());
            match self.clone().action_queue.first() {
                Some((ActionType::MOVE, coords)) => {
                    display_action(ActionType::MOVE, *coords);
                    if self.r#move(terrain, *coords).is_none() {
                        // Pathfinding échoué : définir l’action alternative
                        self.action_queue.remove(0);
                           self.action_queue.push((ActionType::WANDER, self.coords));
                    }
                }
                Some((ActionType::DIG, coords)) => {
                    display_action(ActionType::DIG, *coords);
                    // Vérifie si l’unité est à portée pour creuser
                    if self.coords.distance_in_tiles(coords) > 1 {
                        // Ajouter une action MOVE jusqu'à la dernière case accessibl
                        self.action_queue.remove(0); // Action DIG complétée
                        self.action_queue.insert(0, (ActionType::MOVE, *coords));
                    } else {
                        // Prêt à creuser
                        let actions = self.dig(terrain, coords);
                        if !actions.is_none() {
                        //    self.action_queue.push( (ActionType::DIG, *coords));
                            self.action_queue.append(&mut actions.unwrap());
                        }
                    }
                }
                Some((ActionType::WANDER, coords)) => {
                    display_action_queue(self.clone());
                    let home = terrain.buildings.find_home(self.race, terrain);

                    match home {
                        Some(coords) => {
                            if self.coords.distance_to(&coords) > 10.0
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
                            }
                        }
                        None => {
                            panic!("WHERE IS MY HOME ???")
                        }
                    }
                }

                Some((ActionType::BUILD, _)) => {}

                Some((ActionType::WAIT, _)) => {
                    //    display_action(action.0, action.1);
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
        let Some((mut dig_path, _cost)) = self.find_path(
            self.coords.to_tuple(),
            goal,
            terrain.clone(),
            None,
        ) else {
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
            println!("Digging at {:?}", coords);

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
