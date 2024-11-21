//! Generic function for map creation

#![allow(unused_imports)]
use crate::map::buildings::{Buildable, Building, Content, Stockpile};
use crate::map::minerals::MineralType;
use crate::units::jobs::JobType;
use crate::units::{RaceType, Unit};

use super::{buildings::Hearth, Tile};
use super::{Map, HEIGHT, WIDTH};
use coords::Coords;

#[test]
fn generate() -> Result<(), Coords> {
    let mut map = Map::new();

    map.generate()?;
    Ok(())
}
#[test]
fn joe_finds_a_job() -> Result<(), ()> {
    let mut joe = Unit::new(None);
    joe.coords = Coords(10, 10);
    joe.job = JobType::MINER(super::TileType::Mineral(MineralType::MOSS));

    let mut map = Map::new();
    let res_map = map.generate().clone();
    println!("map generation : {:?}", res_map);

    println!(
        "\"Joe finaly found a job\" -> {:?}",
        joe.find_job_action(&map.clone())
            .expect("Nevermind, find_job_action() failed")
    );
    Ok(())
}
#[test]
fn create_hearths() -> Result<(), Coords> {
    let mut map = Map::new();

    map.build_starting_zone(crate::units::RaceType::ANT)?;
    map.build_starting_zone(crate::units::RaceType::HUMAN)?;
    map.build_starting_zone(crate::units::RaceType::ALIEN)?;

    Ok(())
}

#[test]
fn create_stockpiles() -> Result<(), Coords> {
    let stock: Building<Buildable<RaceType>> = Building {
        buildable: Buildable::Stockpile(Stockpile {
            mineral_type: MineralType::MOSS,
            content: Content(0, 0),
        }),

        race_type: RaceType::ANT,
        coords: Coords(10, 10),
    };

    assert_eq!(stock.stockpile().content.stored_amount(), 0);

    stock.stockpile().content.add(1);

    Ok(())
}