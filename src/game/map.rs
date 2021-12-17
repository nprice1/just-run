use std::vec::Vec;
use std::rc::Rc;
use rand;
use rand::Rng;

use game;
use game::backdrop;
use game::graphics;
use game::sprite;
use game::units;

use game::collisions::Rectangle;
use game::units::{AsGame,AsTile};

#[derive(PartialEq,Eq,Clone)]
pub enum TileType {
	Air,
	Wall
}

pub struct CollisionTile {
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
#[derive(Clone)]
pub struct Tile {
	tile_type:  TileType,
	sprite:     Option<Rc<Box<dyn sprite::Updatable<units::Game>>>>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile { tile_type: TileType::Air, sprite: None }
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(sprite: Rc<Box<dyn sprite::Updatable<units::Game>>>,
	               tile_type: TileType) -> Tile {
		Tile { tile_type: tile_type, sprite: Some(sprite) }
	}
}

#[derive(Clone)]
pub struct Map {
	background:    backdrop::FixedBackdrop,
	tiles:         Vec<Box<Vec<Box<Tile>>>>, 
	page_x:        u32,
	page_y:        u32
}

impl Map {
	/// Will initialize a map (60 * 60) tiles:
	pub fn load_map(graphics: &mut graphics::Graphics, level: i32) -> Map {
		static ROWS: u32 = 60;
		static COLS: u32 = 60; 

		let map_path =  "assets/base/Stage/PrtCave.bmp".to_string();
		let sprite   =  Rc::new(
			Box::new( sprite::Sprite::new(
				graphics,
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) ) as Box<dyn sprite::Updatable<_>>
		);

		let blank_tile = Tile::new();
		let wall_tile = Tile::from_sprite(sprite, TileType::Wall);
		let mut tile_vec: Vec<Box<Vec<Box<Tile>>>> = Vec::new();
		match level { 
			1 => { 
				for i in 0.. ROWS {
					let mut vec = Box::new( Vec::new() );
					for j in 0.. COLS {
						// make the border
						if i == ROWS - 1 || i == 0 || j == 0 || j == COLS - 1 {
							vec.push( Box::new(wall_tile.clone()) );
						}
						else {
							vec.push( Box::new(blank_tile.clone()) );
						}
					}
					tile_vec.push(vec);
				}
			},
			_ => {
				let mut rng = rand::thread_rng();
				let rand_num_cols = rng.gen_range(1..30);
				let rand_num_rows = rng.gen_range(1..30);
				let mut rand_cols: Vec<i32> = Vec::new();
				let mut rand_rows: Vec<i32> = Vec::new();
				for _ in 0.. rand_num_cols {
					let mut rng = rand::thread_rng();
					rand_cols.push( rng.gen_range(1..60) );
				}
				for _ in 0.. rand_num_rows {
					let mut rng = rand::thread_rng();
					rand_rows.push( rng.gen_range(1..60) );
				}
				for i in 0.. ROWS {
					let mut vec = Box::new( Vec::new() );
					for j in 0.. COLS {
						// make the border
						if i == ROWS - 1 || i == 0 || j == 0 || j == COLS - 1 || ( rand_rows.contains(&(i as i32)) && rand_cols.contains(&(j as i32)) ) {
							if (i > 0 && i < 9) && (j > 0 && j < 9) {
								vec.push( Box::new(blank_tile.clone()) );
							} else {
								vec.push( Box::new(wall_tile.clone()) );
							}
						}
						else {
							vec.push( Box::new(blank_tile.clone()) );
						}
					}
					tile_vec.push(vec);
				}
			}
		}
		let background: backdrop::FixedBackdrop = match level {
			1 => { backdrop::FixedBackdrop::new("assets/base/bkBlue.bmp".to_string(), graphics) },
			_ => { backdrop::FixedBackdrop::new("assets/base/bkRed.bmp".to_string(), graphics) },
		};
		let map = Map {
			background: background,
			tiles: tile_vec, 
			page_x: 0,
			page_y: 0
		};
	
		map
	}

	pub fn draw_background(&self, graphics: &mut graphics::Graphics) {
		self.background.draw(graphics);
	}

	/// Draws current state to `display`
	pub fn draw(&mut self, graphics: &mut graphics::Graphics) {
		for a in self.page_y * 20.. (self.page_y * 20) + 20 {
			for b in self.page_x * 20.. (self.page_x * 20) + 20 {
				match self.tiles.get(a as usize).unwrap().get(b as usize).unwrap().sprite {
					Some(ref sprite) => {
						sprite.draw(graphics,
						            (units::Tile(b as u32).to_game() % game::game::SCREEN_WIDTH.to_game(),
						             units::Tile(a as u32).to_game() % game::game::SCREEN_HEIGHT.to_game()));
					}
					_ => {}
				};
			}
		}
	}

	pub fn set_page(&mut self, player_x: units::Game, player_y: units::Game) {
		if player_x < units::Tile(20).to_game() {
			self.page_x = 0;
		} else if player_x > units::Tile(20).to_game() && player_x < units::Tile(40).to_game() {
			self.page_x = 1;
		} else {
			self.page_x = 2;
		}	
		if player_y < units::Tile(20).to_game() {
			self.page_y = 0;
		} else if player_y > units::Tile(20).to_game() && player_y < units::Tile(40).to_game() {
			self.page_y = 1;
		} else {
			self.page_y = 2;
		}	
	}

	pub fn get_page_x(&self) -> u32 {
		self.page_x
	}

	pub fn get_page_y(&self) -> u32 {
		self.page_y
	}

	pub fn get_tiles(self) -> Vec<Box<Vec<Box<Tile>>>> {
		self.tiles
	}

	pub fn on_screen(&self, map_x: units::Game, map_y: units::Game) -> bool {
		let lower_limit_x = units::Tile(self.page_x * 20).to_game();
		let upper_limit_x = lower_limit_x + units::Tile(20).to_game();
		let lower_limit_y = units::Tile(self.page_y * 20).to_game();
		let upper_limit_y = lower_limit_y + units::Tile(20).to_game();
		if map_x <= upper_limit_x && map_x >= lower_limit_x && map_y <= upper_limit_y && map_y >= lower_limit_y {
			true
		} else {
			false
		}
	}

	/// no-op for demo map
	pub fn update(&mut self) {
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
	pub fn get_colliding_tiles(&self, rectangle: &Rectangle) -> Box<Vec<CollisionTile>> {
		let mut collision_tiles: Box<Vec<CollisionTile>> = Box::new( Vec::new() );
		
		let units::Tile(first_row) =  rectangle.top().to_tile();
		let units::Tile(last_row)  =  rectangle.bottom().to_tile();
		let units::Tile(first_col) =  rectangle.left().to_tile();
		let units::Tile(last_col)  =  rectangle.right().to_tile();

		for row in first_row.. last_row + 1 {
			for col in first_col.. last_col + 1 {
				let tile_type = self.tiles.get(row as usize).unwrap().get(col as usize).unwrap().tile_type.clone();
				collision_tiles.push( 
					CollisionTile::new(units::Tile(row), units::Tile(col), tile_type)
				);
			}
		}

		collision_tiles
	}
}
