use std::vec::Vec;
use std::rc::Rc;

use game::backdrop;
use game::graphics;
use game::sprite;
use game::units;

use game::collisions::Rectangle;
use game::units::{AsGame,AsTile};

#[deriving(PartialEq,Eq,Clone)]
pub enum TileType {
	Air,
	Wall
}

struct CollisionTile {
	pub tile_type:  TileType,
	pub row:        units::Tile,
	pub col:        units::Tile
}

impl CollisionTile {
	pub fn new(row: units::Tile, col: units::Tile, 
	           tile_type: TileType) -> CollisionTile {
		CollisionTile { tile_type: tile_type, row: row, col: col }
	}
}

// TODO: Conflicts w/ units::Tile, should probably have a different name.
#[deriving(Clone)]
pub struct Tile {
	tile_type:  TileType,
	sprite:     Option<Rc<Box<sprite::Updatable<units::Game>>>>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile { tile_type: Air, sprite: None }
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(sprite: Rc<Box<sprite::Updatable<units::Game>>>,
	               tile_type: TileType) -> Tile {
		Tile { tile_type: tile_type, sprite: Some(sprite) }
	}
}

pub struct Map {
	background:  backdrop::FixedBackdrop,
	sprites:     Vec<Box<Vec<Box<Tile>>>>,
	tiles:       Vec<Box<Vec<Box<Tile>>>>,
}

impl Map {
	/// Will initialize a map (20 * 20) tiles:
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static rows: uint = 20; // 480
		static cols: uint = 20; // 640

		let map_path =  "assets/base/Stage/PrtCave.bmp".to_string();
		let sprite   =  Rc::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		);

		let blank_tile = Tile::new();
		let wall_tile = Tile::from_sprite(sprite, Wall);
		let mut sprite_vec: Vec<Box<Vec<Box<Tile>>>> = Vec::new();
		let mut tile_vec: Vec<Box<Vec<Box<Tile>>>> = Vec::new();
		for i in range(0, rows) {
			let mut vec = box Vec::new();
			let mut vec2 = box Vec::new();
			for j in range(0, cols) {
				// make the border
				if i == rows - 1 || i == 0 || j == 0 || j == cols - 1 {
					vec.push(box wall_tile.clone());
					vec2.push(box wall_tile.clone());
				}
				else {
					vec.push(box blank_tile.clone());
					vec2.push(box blank_tile.clone());
				}
			}
			sprite_vec.push(vec);
			tile_vec.push(vec2);
		}
		let map = Map {
			background: backdrop::FixedBackdrop::new("assets/base/bkBlue.bmp".to_string(), graphics),
			sprites: sprite_vec,
			tiles: tile_vec
		};
	
		map
	}

	pub fn draw_background(&self, graphics: &graphics::Graphics) {
		self.background.draw(graphics);
	}

	pub fn draw_sprites(&self, graphics: &graphics::Graphics) {
		for a in range(0, self.sprites.len()) {
			for b in range(0, self.sprites.get(a).len()) {
				match self.sprites.get(a).get(b).sprite {
					Some(ref sprite) => {
						sprite.draw(graphics, 
						            (units::Tile(b).to_game(),
						             units::Tile(a).to_game()));
					}
					_ => {}
				};
			}
		}
	}

	/// Draws current state to `display`
	pub fn draw(&self, graphics: &graphics::Graphics) {
		for a in range(0, self.tiles.len()) {
			for b in range(0, self.tiles.get(a).len()) {
				match self.tiles.get(a).get(b).sprite {
					Some(ref sprite) => {
						sprite.draw(graphics,
						            (units::Tile(b).to_game(),
						             units::Tile(a).to_game()));
					}
					_ => {}
				};
			}
		}
	}


	/// no-op for demo map
	#[allow(unused_variable)]
	pub fn update(&mut self, elapsed_time: units::Millis) {
		/* 
		 * This was effectively unused and IMHO does not warrant the
		 * complexity introduced by using dynamic borrow-ck'ing.
		 * 
		 * As most background sprites are shared [in this demo map] any
		 * animations would look really goofy as all tiles would
		 * advance their frames in perfect sync.
		 */
	}

	/// Checks if `Rectangle` is colliding with any tiles in the foreground.
	/// 
	/// NOTE: Checking a Rectangle which would be placed outside the tile-map
	/// results in a runtime failure!
	/// 
	/// NOTE: This is a simple check of the _outside bounds_ of the
	/// rectangle & tile. -- This method may claim that the player is 
	/// colliding w/ the edge of a tile that _appears to be_ empty space.
	#[allow(visible_private_types)]
	pub fn get_colliding_tiles(&self, rectangle: &Rectangle) -> Box<Vec<CollisionTile>> {
		let mut collision_tiles: Box<Vec<CollisionTile>> = box Vec::new();
		
		let units::Tile(first_row) =  rectangle.top().to_tile();
		let units::Tile(last_row)  =  rectangle.bottom().to_tile();
		let units::Tile(first_col) =  rectangle.left().to_tile();
		let units::Tile(last_col)  =  rectangle.right().to_tile();

		for row in range(first_row, last_row + 1) {
			for col in range(first_col, last_col + 1) {
				collision_tiles.push( 
					CollisionTile::new(units::Tile(row), units::Tile(col), self.tiles.get(row).get(col).tile_type)
				);
			}
		}

		collision_tiles
	}
}
