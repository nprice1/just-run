use std::rand::{task_rng, Rng};

use std::collections::hashmap::HashMap;

use game::graphics;
use game::sprite;

use game::collisions::{Info,Rectangle};
use game::map;

use game::units;
use game::units::AsGame;

pub type MotionTup = (sprite::Motion, sprite::Facing);

static SPRITE_NUM_FRAMES:  units::Frame  = 3;
static SPRITE_FPS:         units::Fps    = 20;

static KILL_FRAME:  units::Tile = units::Tile(0);

// collision detection boxes
// (expressed as `units::Game`)
static X_BOX: Rectangle = Rectangle {
	x: units::Game(6.0), y: units::Game(10.0), 
	width: units::Game(20.0), height: units::Game(12.0)
};
static Y_BOX: Rectangle = Rectangle {
	x: units::Game(10.0), y: units::Game(6.0), 
	width: units::Game(12.0), height: units::Game(30.0)
};

// Screen correction for changing pages
pub static SCREEN_CORRECTION: units::Tile = units::Tile(20);

pub struct Character {
	// assets
	pub sprites:   HashMap<MotionTup, Box<sprite::Updatable<units::Game>>>,
	pub killed_sprite: Vec<Box<sprite::Updatable<units::Game>>>, 

	// positioning on screen
	pub x: units::Game, 
	pub y: units::Game,
	// positioning on map
	pub map_x: units::Game,
	pub map_y: units::Game,
	pub movement:  MotionTup,

	// physics
	pub elapsed_time:  units::Millis,
	pub velocity_x:    units::Velocity,
	pub velocity_y:    units::Velocity,
	pub accel_x:       int,
	pub accel_y:       int, 
	pub target_x:      units::Game, 
	pub target_y:      units::Game,

	// flags
	pub killed: int
}

impl Character {
	/// Loads and initializes a set of sprite-sheets for the various combinatoins of directions.
	/// (These incl: facing west and east for: standing, walking, jumping, falling.)
	///
	/// The player will spawn at `x` and `y`, though it will immediately be subject to gravity.
	/// The player is initailized `standing` facing `east`.
	pub fn new(x: units::Game, y: units::Game) -> Character {
		// insert sprites into map
		let sprite_map = HashMap::<MotionTup, Box<sprite::Updatable<_>>>::new();
		let killed_vec: Vec<Box<sprite::Updatable<units::Game>>> = Vec::new();

		// construct new player
		let new_character = Character{
			elapsed_time: units::Millis(0),
			sprites:   sprite_map,
			killed_sprite: killed_vec,

			x: x,
			y: y,
			map_x: x,
			map_y: y,
			movement: (sprite::Standing, sprite::East),
			
			velocity_x: units::Velocity(0.0),
			velocity_y: units::Velocity(0.0),
			accel_x: 1,
			accel_y: 0, 
			target_x: x, 
			target_y: y, 

			killed: -1
		};

		new_character
	}

	// Draws player to screen
	pub fn draw(&self, display: &mut graphics::Graphics) {
		if self.killed >= 0 {
			self.killed_sprite.get(0).draw(display, (self.x, self.y));
		} else {
			self.sprites.get(&self.movement).draw(display, (self.x, self.y));
		}
	}

	pub fn load_killed_sprite(&mut self, display: &mut graphics::Graphics)
	{
		let asset_path = "assets/base/killed.bmp".to_string();
		let motion_frame = KILL_FRAME; 
		let facing_frame = units::Tile(0);
		let sprite = box sprite::AnimatedSprite::new(
				display, asset_path, 
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				SPRITE_NUM_FRAMES, SPRITE_FPS
			).unwrap() as Box<sprite::Updatable<_>>;
		self.killed_sprite.push(sprite);
	}

	pub fn current_motion(&mut self) {
		let (_, last_facing) = self.movement;

		self.movement = 
			if self.accel_x == 0 && self.accel_y == 0 {
				(sprite::Standing, last_facing)
			} else {
				(sprite::Walking, last_facing)
			}	
	}

	pub fn kill_character(&mut self) {
		self.killed = 5;
	}

	pub fn is_killed(&self) -> bool {
		self.killed >= 0
	}

	pub fn set_facing(&mut self, direction: sprite::Facing) {
		let (last_action, _) = self.movement;
		self.movement = (last_action, direction);
	}

	pub fn update_x(&mut self, map: &map::Map, acceleration: units::Acceleration, max_velocity: units::Velocity) {
		// compute next velocity
		let accel_x: units::Acceleration = if self.accel_x < 0  {
			-acceleration
		} else if self.accel_x > 0 {
		    acceleration
		} else { units::Acceleration(0.0) };

		self.velocity_x = self.velocity_x + (accel_x * self.elapsed_time);

		if self.accel_x < 0 {
			self.velocity_x = units::max(self.velocity_x, -max_velocity);
		} else if self.accel_x > 0 {
			self.velocity_x = units::min(self.velocity_x, max_velocity);
		} 

		// x-axis collision checking 
		let delta = self.velocity_x * self.elapsed_time;
		if delta > units::Game(0.0) { // moving right
			// collisions right-side
			let mut info = self.get_collision_info(&self.right_collision(delta), map);
			self.x = if info.collided {
				self.velocity_x = units::Velocity(0.0);
				self.map_x = info.col.to_game() - X_BOX.right();
				(info.col.to_game() - X_BOX.right()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_x = self.map_x + delta;
				self.map_x % SCREEN_CORRECTION.to_game()
			};

			// collisions left-side
			info = self.get_collision_info(&self.left_collision(units::Game(0.0)), map);
			self.x = if info.collided {
				self.map_x = info.col.to_game() + X_BOX.right();
				(info.col.to_game() + X_BOX.right()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_x % SCREEN_CORRECTION.to_game()
			};

		} else { // moving left
			// collisions left-side
			let mut info = self.get_collision_info(&self.left_collision(delta), map);
			self.x = if info.collided {
				self.velocity_x = units::Velocity(0.0);
				self.map_x = info.col.to_game() + X_BOX.right();
				(info.col.to_game() + X_BOX.right()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_x = self.map_x + delta;
				self.map_x % SCREEN_CORRECTION.to_game()
			};

			// collisions right-side
			info = self.get_collision_info(&self.right_collision(units::Game(0.0)), map);
			self.x = if info.collided {
				self.map_x = info.col.to_game() - X_BOX.right();
				(info.col.to_game() - X_BOX.right()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_x % SCREEN_CORRECTION.to_game()
			};
		}
	}

	pub fn update_y (&mut self, map: &map::Map, acceleration: units::Acceleration, max_velocity: units::Velocity) {
		// compute next velocity
		let accel_y: units::Acceleration = if self.accel_y < 0  {
			-acceleration 
		} else if self.accel_y > 0 {
		    acceleration
		} else { units::Acceleration(0.0) };

		self.velocity_y = self.velocity_y + (accel_y * self.elapsed_time);

		if self.accel_y < 0 {
			self.velocity_y = units::max(self.velocity_y, -max_velocity);
		} else if self.accel_y > 0 {
			self.velocity_y = units::min(self.velocity_y, max_velocity);
		} 

		// calculate delta
		let delta = self.velocity_y * self.elapsed_time;

		// check collision in direction of delta
		if delta > units::Game(0.0) { // moving up
			// react to collision
			let mut info = self.get_collision_info(&self.bottom_collision(delta), map);
			self.y = if info.collided {
				self.velocity_y = units::Velocity(0.0);
				self.map_y = info.row.to_game() - Y_BOX.bottom();
				(info.row.to_game() - Y_BOX.bottom()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_y = self.map_y + delta;
				self.map_y % SCREEN_CORRECTION.to_game()
			};

			info = self.get_collision_info(&self.top_collision(units::Game(0.0)), map);
			self.y = if info.collided {
				self.map_y = info.row.to_game() + Y_BOX.height();
				(info.row.to_game() + Y_BOX.height()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_y % SCREEN_CORRECTION.to_game()
			};

		} else { // moving down
			// react to collision
			let mut info = self.get_collision_info(&self.top_collision(delta), map);
			self.y = if info.collided {
				self.velocity_y = units::Velocity(0.0);
				self.map_y = info.row.to_game() + Y_BOX.height();
				(info.row.to_game() + Y_BOX.height()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_y = self.map_y + delta;
				self.map_y % SCREEN_CORRECTION.to_game()
			};

			info = self.get_collision_info(&self.bottom_collision(units::Game(0.0)), map);
			self.y = if info.collided {
				self.map_y = info.row.to_game() - Y_BOX.bottom();
				(info.row.to_game() - Y_BOX.bottom()) % SCREEN_CORRECTION.to_game()
			} else {
				self.map_y % SCREEN_CORRECTION.to_game()
			};
		}
	}

	fn get_collision_info(&self, hitbox: &Rectangle, tile_map: &map::Map) -> Info {
		let tiles = 
			tile_map.get_colliding_tiles(hitbox);

		let mut info = Info { collided: false, row: units::Tile(0), col: units::Tile(0) };
		for tile in tiles.iter() {
			if tile.tile_type == map::Wall {
				info = Info {collided: true, row: tile.row, col: tile.col};
				break;
			}
		}

		info
	}

	/// A player's damage rectangle encompasses the whole player.
	pub fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.map_x + X_BOX.left(),
			y: self.map_y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	pub fn center_x(&self) -> units::Game {
		self.x + units::HalfTile(1)
	}

	pub fn center_y(&self) -> units::Game {
		self.y + units::HalfTile(1)
	}

	pub fn map_center_x(&self) -> units::Game {
		self.map_x + units::HalfTile(1)
	}

	pub fn map_center_y(&self) -> units::Game {
		self.map_y + units::HalfTile(1)
	}

	// x-axis collision detection
	fn left_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta <= units::Game(0.0));

		Rectangle {
			x: self.map_x + (X_BOX.left() + delta),
			y: self.map_y + X_BOX.top(),
			width: (X_BOX.width() / units::Game(2.0)) - delta,
			height: X_BOX.height()
		}
	}

	
	fn right_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta >= units::Game(0.0));
		
		Rectangle {
			x: self.map_x + X_BOX.left() + (X_BOX.width() / units::Game(2.0)),
			y: self.map_y + X_BOX.top(),
			width: 	(X_BOX.width() / units::Game(2.0)) + delta,
			height: X_BOX.height()
		}
	}

	// y-axis collision detection
	fn top_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta <= units::Game(0.0));

		Rectangle {
			x: self.map_x + Y_BOX.left(),
			y: self.map_y + (Y_BOX.top() + delta),
			width: Y_BOX.width(),
			height: (Y_BOX.height() / units::Game(2.0)) - delta
		}
	}

	fn bottom_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta >= units::Game(0.0));
		
		Rectangle {
			x: self.map_x + Y_BOX.left(),
			y: self.map_y + Y_BOX.top() + (Y_BOX.height() / units::Game(2.0)),
			width:  Y_BOX.width(),
			height: (Y_BOX.height() / units::Game(2.0)) + delta
		}
	}

	pub fn distance(&self, other_x: units::Game, other_y: units::Game) -> f64 {
		let units::Game(xs) = (other_x - self.map_x) * (other_x - self.map_x);
		let units::Game(ys) = (other_y - self.map_y) * (other_y - self.map_y);

		(xs + ys).sqrt()
	}

	pub fn set_new_target(&mut self) {
		let mut rng = task_rng();
		let distance_to_target = self.distance( self.target_x, self.target_y );

		if distance_to_target < 20.0 {
			let chance_x = rng.gen_range(1u, 3u);
			let chance_y = rng.gen_range(1u, 3u);
			let plus_or_minus = rng.gen_range(1u, 3u);
			self.target_x = match self.map_center_x() {
				center if plus_or_minus == 1 => {
						if center > units::Tile(1).to_game() && center < units::Tile(56).to_game() {
							center + units::Tile(chance_x).to_game()
						} else {
							center - units::Tile(chance_x).to_game()
						}
					}
				center => {
						if center > units::Tile(3).to_game() && center < units::Tile(58).to_game() {
							center - units::Tile(chance_x).to_game()
						} else {
							center + units::Tile(chance_x).to_game()
						}
					}
			};
			self.target_y = match self.map_center_y() {
				center if plus_or_minus == 1 => {
						if center > units::Tile(1).to_game() && center < units::Tile(56).to_game() {
							center + units::Tile(chance_y).to_game()
						} else {
							center - units::Tile(chance_y).to_game()
						}
					}
				center => {
						if center > units::Tile(3).to_game() && center < units::Tile(58).to_game() {
							center - units::Tile(chance_y).to_game()
						} else {
							center + units::Tile(chance_y).to_game()
						}
					}
			};
		}
	}

	pub fn set_new_random_target(&mut self) {
		let mut rng = task_rng();
		let distance_to_target = self.distance( self.target_x, self.target_y );

		if distance_to_target < 20.0 {
			self.target_x = units::Tile(rng.gen_range(1u, 58u)).to_game();
			self.target_y = units::Tile(rng.gen_range(1u, 58u)).to_game();
		}
	}

}