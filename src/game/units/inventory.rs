use super::TileType;



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Mineral(TileType),
}
impl Item {
    fn mineral(self) -> Result<Item, ()> {
        match self {
            Item::Mineral(tile_type) => Ok(self),
            _ => Err(()),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Inventory(pub Vec<Item>);
impl Inventory {
    pub fn new(capacity: u32) -> Inventory {
        Inventory(Vec::with_capacity(0 as usize))
    }
    pub fn add(&mut self, item: Item) -> Result<(), ()> {
        if self.0.len() < 10 {
            self.0.push(item);
            return Ok(());
        } else {
            return Err(());
        }
    }
    pub fn consume(&mut self, &mut item: &mut Item) -> Result<&mut Inventory, ()> {
        let mut index = 0;
        for i in self.0.as_mut_slice() {
            if *i == item {
                self.0.remove(index);
                return Ok(self);
            }
            index += 1;
        }
        Err(())
    }
    pub fn is_empty(self) -> bool {
        self.0.len() == 0
    }
    pub fn is_full(self) -> bool {
        self.0.len() == 10
    }
    pub fn any_mineral(self) -> Result<Item, Self> {
        for item in self.0.clone() {
            //println!("{:?}", item);
            item.mineral().ok();
        }
        Err(self)
    }
}
