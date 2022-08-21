use crate::math_lib::vector::{Vector, Vector2U};
pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;
pub const HEIGHT_LESS_ONE: usize = HEIGHT - 1;
pub const WIDTH_LESS_ONE: usize = WIDTH - 1;
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
#[derive(Debug)]
pub struct Map {
    pub data: Vector<Tile, { WIDTH * HEIGHT }>,
    pub size: Vector<usize, 2>,
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
