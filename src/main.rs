pub mod graphics;
pub mod logic;
pub mod math_lib;

use crate::logic::map_lib::map::Map;
use crate::graphics::common::common_graphics::Drawable;
fn main() {
    let map: Map = Map::create();
    map.draw()
}
