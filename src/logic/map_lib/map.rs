use crate::{
    graphics::tui::tui_lib::Drawable,
    math_lib::vector::{Vector, Vector2U},
};
const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const HEIGHT_LESS_ONE: usize = HEIGHT - 1;
const WIDTH_LESS_ONE: usize = WIDTH - 1;
#[derive(Debug, Clone, Copy)]
pub enum TileType {
    WATER,
    ROCK,
}
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: Vector2U,
}
impl Tile {
    pub fn default() -> Self {
        Self {
            tile_type: TileType::ROCK,
            position: Vector::default(0),
        }
    }
    pub fn new(position: Vector2U, tile_type: TileType) -> Self {
        Self {
            tile_type,
            position: Vector::move_construct(position),
        }
    }
}
impl Drawable for Tile {
    fn draw(&self) {
        match self.tile_type {
            TileType::ROCK => print!("\u{1F378}"),
            TileType::WATER => print!("\u{1F37A}"),
        }
    }
}
#[derive(Debug)]
pub struct Map {
    data: Vector<Tile, { WIDTH * HEIGHT }>,
    size: Vector<usize, 2>,
}
impl Map {
    pub fn create() -> Self {
        Self {
            data: {
                let mut v: Vector<Tile, { WIDTH * HEIGHT }> = Vector::default(Tile::default());
                for x in 0..WIDTH {
                    for y in 0..HEIGHT {
                        let current_pos = Vector2U::create_from_array([x, y]);
                        v.set(Tile::new(current_pos, TileType::WATER), x * WIDTH + y);
                    }
                }
                v
            },
            size: Vector::default(WIDTH),
        }
    }
}
impl Drawable for Map {
    fn draw(&self) {
        for tile in self.data {
            let &position_y = tile.position.get(1);
            let &position_x = tile.position.get(0);
            /*
             *    +-+
             *    |x|
             *    +-+
             *
             * */
            if position_y == 0 && position_x == 0 {
                print!("/|");
            } else if position_y == WIDTH_LESS_ONE {
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
