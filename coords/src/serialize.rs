use std::ops::{ Add, AddAssign, Sub, SubAssign };

use serde::{ Serialize, Deserialize, Deserializer };

use crate::coords::Coords;

impl<'de, T> Deserialize<'de>
    for Coords<T>
    where T: Clone + Add + AddAssign + Sub + SubAssign + Serialize + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct CoordsInternal<T> {
            x: T,
            y: T,
        }

        let helper = CoordsInternal::deserialize(deserializer)?;
        Ok(Coords::new(helper.x, helper.y))
    }
}
