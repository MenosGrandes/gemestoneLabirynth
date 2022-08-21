pub mod logic;
pub mod math_lib;
pub mod graphics;
use graphics::tui::tui_lib::Drawable;

use crate::logic::map_lib::map::Map;
use crate::math_lib::vector::Vector;
fn main() {
    let map : Map = Map::create();
    map.draw()
}
