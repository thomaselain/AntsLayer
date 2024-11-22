use colored::{ColoredString, Colorize};
use coords::Coords;

use super::{RaceType, Unit};

pub struct Action(pub ActionType, pub Coords);
impl Action {
    fn to_tuple(self) -> (ActionType, Coords) {
        (self.0, self.1)
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
    fn cancel_first(&mut self) -> Result<(), ()>;
    fn do_next(&mut self, action: Action);
    fn do_now(&mut self, action: Action);
    fn do_later(&mut self, action: Action);
    fn do_then(&mut self, action: Action);
    fn remove_only(&mut self, action: Vec<ActionType>);
    fn keep_only(&mut self, action: Vec<ActionType>);
}

impl ActionQueue for Vec<(ActionType, Coords)> {
    fn cancel_first(&mut self) -> Result<(), ()> {
        if self.len() == 0 {
            Err(())
        } else {
            self.remove(0);
            Ok(())
        }
    }
    fn do_next(&mut self, action: Action) {
        self.insert(0, action.to_tuple());
    }
    fn do_then(&mut self, action: Action) {
        self.insert(1, action.to_tuple());
    }

    #[allow(unused_must_use)]
    fn do_now(&mut self, action: Action) {
        self.cancel_first();
        self.insert(0, action.to_tuple());
    }

    fn do_later(&mut self, action: Action) {
        self.push(action.to_tuple());
    }

    fn remove_only(&mut self, actions: Vec<ActionType>) {
        for to_remove in actions {
            self.retain_mut(|(what, coords)| (*what, *coords) == (to_remove, *coords));
        }
    }

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
            ActionType::MOVE => "-->".bold().blue(),
            ActionType::HAUL => "-->".bold().green(),
            ActionType::DIG => " D ".bold().red(),
            ActionType::WANDER => " ? ".italic().green(),
            ActionType::WAIT => "...".italic().bright_green(),
            _ => " ".into(),
        }
    }
}

/// FOR DEBUG
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
    println!("at ({:?},{:?}) !", coords.x(), coords.y());
}
