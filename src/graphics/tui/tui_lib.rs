use crate::graphics::common::common_graphics::Drawable;
use crate::logic::map_lib::map::{Map, Tile, TileType, WIDTH_LESS_ONE};

impl Drawable for Tile {
    fn draw(&self) {
        match self.tile_type {
            TileType::ROCK => print!("\u{1F378}"),
            TileType::WATER => print!("\u{1F37A}"),
        }
    }
}

impl Drawable for Map {
    fn draw(&self) {
        for tile in self.data {
            let &position_y = tile.position.get(1);
            let &position_x = tile.position.get(0);
            if position_y == 0 && position_x == 0 {
                print!("/|");
            } else if position_y ==  WIDTH_LESS_ONE{
            } else if position_y == 0 {
                print!("/|");
            }
            tile.draw();

            if position_y == 0 && position_x == 0 {
                print!("|");
            } else if position_y == WIDTH_LESS_ONE {
                println!("\\");
            } else {
                print!("|");
            }
        }
    }
}
