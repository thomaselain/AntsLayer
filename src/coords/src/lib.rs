mod coords;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coords(pub i32, pub i32);

#[test]
fn add() {
    let some = Coords(1,0);
    let other = Coords(0,1);
    assert_eq!(some + other, Coords(1,1));
}

#[test]
fn sub(){
    let some = Coords(1,0);
    assert_eq!(some - some, Coords(0,0));
}

#[test]
fn swap_coords(){
    let some = Coords(1,0);
    let other = Coords(0,1);
    assert_eq!(some.swap_coords(), other);
}

#[test]
fn distance_in_tiles(){
    let some = Coords(-1,1);
    let other = Coords(1,-1);
    assert_eq!(some.distance_in_tiles(&other), 2);
}
