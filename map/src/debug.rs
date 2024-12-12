use std::fmt;

use crate::Map;

impl fmt::Debug for Map {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ((x, y), chunk) in self.chunks.iter() {
            print!("({},{}){:?}", x, y, chunk);
        }
        Ok(())
    }
}
