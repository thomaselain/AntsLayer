use crate::{
    terrain::{self, MineralType, Terrain},
    units::{display_action_queue, ActionQueue, ActionType, JobType, RaceType, Unit},
};

#[derive(Clone)]
pub struct Team {
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub color: u32,
}

impl Team {
    pub fn new(race_type: RaceType) -> Team {
        // /////////////////////// UNITS /////////////////////////////////////////////
        let mut units: Vec<Unit> = Vec::new();

        for _ in 0..1 {
            let mut unit = Unit::new();
            unit.race = race_type;
            unit.job = JobType::MINER(terrain::MineralType::IRON);
            unit.action_queue.do_now(ActionType::WANDER, unit.coords);
            units.push(unit);
        }
        /////////////////////////////////////////////////////////

        Team {
            units,
            color: 0xff000033,
            buildings: vec![
                Building::new(race_type, BuildingType::Hearth),
                Building::new(race_type, BuildingType::Stockpile(MineralType::MOSS)),
                Building::new(race_type, BuildingType::Stockpile(MineralType::ROCK)),
                Building::new(race_type, BuildingType::Stockpile(MineralType::IRON)),
                Building::new(race_type, BuildingType::Stockpile(MineralType::GOLD)),
            ],
        }
    }
    pub fn units_turn(mut self, terrain: &Terrain, delta_time: i32) -> Terrain {
        for u in self.units.iter_mut() {
            display_action_queue(RaceType::ANT, u.clone());
            let job = u.job.get_action(terrain, u);
            u.action_queue.do_now(job.0, job.1);
            //    u.think(&mut &terrain, delta_time);
        }
        terrain.clone()
    }
}

impl Terrain {
    /// Find matching team in terrain
    /// Creates a new one of race_type if none found (maybe it should not ?)
    pub fn team(self, race_type: RaceType) -> Team {
        /*   for (race, team) in self.teams.iter() {
                    if *race == race_type {
                        return team.clone();
                    }
                }
        */
        return Team::new(race_type);
    }
}
