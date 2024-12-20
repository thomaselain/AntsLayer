use std::fmt;

use crate::Map;

impl fmt::Debug for Map {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (pos, chunk) in self.chunks.iter() {
            print!("{:?} : {:?}", pos, chunk);
        }
        Ok(())
    }
}
