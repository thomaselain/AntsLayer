#[allow(unused_imports)]
use coords::aliases::TilePos;

#[cfg(test)]
use crate::{ Tile, TileType, TileFlags };

#[test]
fn tile_creation() {
    let tile = Tile::new(TilePos::new(0, 0,0), TileType::Sand, TileFlags::TRAVERSABLE);
    assert_eq!(tile.coords, TilePos::new(0, 0,0));
    assert_eq!(tile.tile_type, TileType::Sand);
    assert!(tile.flags.contains(TileFlags::TRAVERSABLE));
    assert_eq!(tile.extra_data, None);
}

#[test]
fn set_extra_data() {
    let mut tile = Tile::new(TilePos::new(1, 1,1), TileType::Wall, TileFlags::BUILDABLE);
    tile.set_extra_data(10);
    assert_eq!(tile.extra_data, Some(10));
}

#[test]
fn flags() {
    let tile = Tile::new(TilePos::new(2, 2,1), TileType::Fluid(crate::FluidType::Magma), TileFlags::TRAVERSABLE | TileFlags::LIQUID);
    assert!(tile.flags.contains(TileFlags::TRAVERSABLE));
    assert!(tile.flags.contains(TileFlags::LIQUID));
    assert!(!tile.flags.contains(TileFlags::DIGGABLE)); // Vérifie que l'état DIGGABLE n'est pas actif
}

#[test]
fn tile_type() {
    let empty_tile = Tile::new(TilePos::new(3, 3,3), TileType::Empty,  TileFlags::empty());
    let wall_tile = Tile::new(TilePos::new(4, 4,4), TileType::Wall,  TileFlags::TRAVERSABLE);

    assert_eq!(empty_tile.tile_type, TileType::Empty);
    assert_eq!(wall_tile.tile_type, TileType::Wall);
}

#[test]
fn tile_flags() {
    let tile = Tile::new(TilePos::new(5, 5,5), TileType::Floor,  TileFlags::DIGGABLE | TileFlags::BUILDABLE);

    assert!(tile.flags.contains(TileFlags::DIGGABLE));
    assert!(tile.flags.contains(TileFlags::BUILDABLE));
}

#[test]
fn custom_tile_type() {
    let custom_tile = Tile::new(TilePos::new(6, 6,6), TileType::Custom(7),  TileFlags::empty());
    if let TileType::Custom(value) = custom_tile.tile_type {
        assert_eq!(value, 7);
    } else {
        panic!("Expected Custom tile type.");
    }
}
