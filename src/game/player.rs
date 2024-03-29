use std::collections::HashMap;

use game::graphics;
use game::sprite;

use game::map;

use game::units;

use game::common;

pub type MotionTup = (sprite::Motion, sprite::Facing);

// player sprite animation
const CHAR_OFFSET:          u32           = 12;
const CRICKET_OFFSET:       u32           = 10;
static SPRITE_NUM_FRAMES:   units::Frame  = 3;
static SPRITE_FPS:          units::Fps    = 20;
static STANDING_HIT_FRAMES: units::Frame  = 2;
static WALKING_HIT_FRAMES:  units::Frame  = 4;
static HIT_FPS:             units::Fps    = 10;

static WALKING_ACCEL:  units::Acceleration  = units::Acceleration(0.00083007812);
static MAX_VELOCITY: units::Velocity      = units::Velocity(0.17859375);

// motion
static STAND_FRAME: units::Tile   = units::Tile(0);
static TELEPORT_FRAME: units::Tile = units::Tile(3);
static TELEPORT_ANIMATION_FRAME: units::Tile = units::Tile(1);
static HIT_FRAME: units::Tile = units::Tile(3);

// horizontal facing (Facing)
static FACING_WEST: units::Tile  = units::Tile(0 + CHAR_OFFSET);
static FACING_EAST: units::Tile  = units::Tile(1 + CHAR_OFFSET);
static CRICKET_FACING_WEST: units::Tile  = units::Tile(0 + CRICKET_OFFSET);
static CRICKET_FACING_EAST: units::Tile  = units::Tile(1 + CRICKET_OFFSET);

pub struct Player {
	pub character: common::Character,
	cricket_sprites: HashMap<MotionTup, Box<dyn sprite::Updatable<units::Game>>>,
	teleport_sprites: HashMap<MotionTup, Box<dyn sprite::Updatable<units::Game>>>,
	hit_sprites: HashMap<MotionTup, Box<dyn sprite::Updatable<units::Game>>>,
	cricket_bat: bool,
	teleport_timer: i32, 
	immunity_timer: i32,
	health: u32,
	follow_x: units::Game,
	follow_y: units::Game,
	update_count: i32
}

impl Player {
	/// Loads and initializes a set of sprite-sheets for the various combinatoins of directions.
	/// (These incl: facing west and east for: standing, walking, jumping, falling.)
	///
	/// The player will spawn at `x` and `y`, though it will immediately be subject to gravity.
	/// The player is initailized `standing` facing `east`.
	/// The player will continue to fall until some collision is detected.
	pub fn new(graphics: &mut graphics::Graphics, x: units::Game, y: units::Game) -> Player {
		let cricket = HashMap::<MotionTup, Box<dyn sprite::Updatable<_>>>::new();
		let teleport = HashMap::<MotionTup, Box<dyn sprite::Updatable<_>>>::new();
		let hit = HashMap::<MotionTup, Box<dyn sprite::Updatable<_>>>::new();
		// construct new player
		let mut new_player = Player{
			character: common::Character::new(x, y),
			cricket_sprites: cricket,
			teleport_sprites: teleport,
			hit_sprites: hit,
			cricket_bat: false,
			teleport_timer: 0, 
			immunity_timer: 0,
			health: 3, 
			follow_x: units::Game(0 as f64),
			follow_y: units::Game(0 as f64),
			update_count: 0
		};

		// load sprites for every possible movement tuple.
		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_player.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_player
	}

	/// Updates player-state that relies on time data. (Namely physics calculations.)
	/// Determines which sprite-sheet should be used for this frame.
	/// Forwards the elapsed time to the current sprite.
	pub fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		// calculate current position
		self.character.elapsed_time = elapsed_time;

		// update timers
		if self.teleport_timer > 0 {
			self.teleport_timer = self.teleport_timer - 1;
		}
		if self.immunity_timer > 0 {
			self.immunity_timer = self.immunity_timer - 1;
		}

		// update the last know location of player for enemies that chase
		if self.update_count == 10 {
			self.follow_x = self.character.map_center_x();
			self.follow_y = self.character.map_center_y();
			self.update_count = 0;
		} else {
			self.update_count = self.update_count + 1;
		}

		// update sprite
		self.character.current_motion(); // update motion once at beginning of frame for consistency
		self.character.sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		if self.has_bat() {
			self.cricket_sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		}
		if self.teleport_timer > 0 {
			self.teleport_sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		}
		if self.immunity_timer > 0 {
			self.hit_sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		}
		if self.character.is_killed() {
			self.character.killed_sprite.get_mut(0).unwrap().update(elapsed_time);
		}

		// run physics sim
		self.character.update_x(map, WALKING_ACCEL, MAX_VELOCITY);
		self.character.update_y(map, WALKING_ACCEL, MAX_VELOCITY);
	}

	/// Loads a sprite for the selected `movement`, stores it in the player's sprite map.
	/// This exhaustively matches all tuples of (Motion,Facing,Looking), though certain
	/// sprites are considered invalid states [for e.g: walking + looking down]
	fn load_sprite(
		&mut self, 
		graphics: &mut graphics::Graphics, 
		movement: (sprite::Motion, sprite::Facing)
	) {
		self.character.load_killed_sprite(graphics);
		self.cricket_sprites.insert(movement, {
			let file_path = "assets/MyChar.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = STAND_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => CRICKET_FACING_WEST,
				sprite::Facing::East => CRICKET_FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Motion::Standing, _) => {
					Box::new( sprite::Sprite::new(
						graphics, 
						(motion_frame, facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) ) as Box<dyn sprite::Updatable<_>> 
				}

				// dynamic: 
				(sprite::Motion::Walking, _) => {
					Box::new( sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() ) as Box<dyn sprite::Updatable<_>>
				}
			}
		});
		self.teleport_sprites.insert(movement, {
			let file_path = "assets/MyChar.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = TELEPORT_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => CRICKET_FACING_WEST,
				sprite::Facing::East => CRICKET_FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Motion::Standing, _) => {
					Box::new( sprite::Sprite::new(
						graphics, 
						(motion_frame, facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) ) as Box<dyn sprite::Updatable<_>> 
				}

				// dynamic: 
				(sprite::Motion::Walking, _) => {
					Box::new( sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() ) as Box<dyn sprite::Updatable<_>>
				}
			}
		});
		self.hit_sprites.insert(movement, {
			let file_path = "assets/MyChar.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = HIT_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => FACING_WEST,
				sprite::Facing::East => FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Motion::Standing, _) => {
					Box::new( sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						STANDING_HIT_FRAMES, HIT_FPS
					).unwrap() ) as Box<dyn sprite::Updatable<_>>
				}

				// dynamic: 
				(sprite::Motion::Walking, _) => {
					Box::new( sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						WALKING_HIT_FRAMES, HIT_FPS
					).unwrap() ) as Box<dyn sprite::Updatable<_>>
				}
			}
		});
		self.character.sprites.insert(movement, {
			let file_path = "assets/MyChar.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = STAND_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => FACING_WEST,
				sprite::Facing::East => FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Motion::Standing, _) => {
					Box::new( sprite::Sprite::new(
						graphics, 
						(motion_frame, facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) ) as Box<dyn sprite::Updatable<_>> 
				}

				// dynamic: 
				(sprite::Motion::Walking, _) => {
					Box::new( sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() ) as Box<dyn sprite::Updatable<_>>
				}
			}
		});
	}

	//draw the player
	pub fn draw(&self, display: &mut graphics::Graphics) {
		if self.cricket_bat {
			self.cricket_sprites.get(&self.character.movement).unwrap().draw(display, (self.character.x, self.character.y));
		} else if self.teleport_timer > 0 {
			match self.teleport_timer {
				x if x > 15 => {
					let asset_path = "assets/base/teleport.bmp".to_string();
					let motion_frame = TELEPORT_ANIMATION_FRAME;
					let facing_frame = units::Tile(0);
					let teleport_sprite = Box::new( sprite::Sprite::new(
						display, 
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						asset_path
					) ) as Box<dyn sprite::Updatable<_>>;
					teleport_sprite.draw(display, (self.character.x, self.character.y));
				},
				_ => { self.teleport_sprites.get(&self.character.movement).unwrap().draw(display, (self.character.x, self.character.y)); }
			}
		} else if self.immunity_timer > 0 {
			self.hit_sprites.get(&self.character.movement).unwrap().draw(display, (self.character.x, self.character.y));
		} else {
			self.character.draw(display);
		}
	}

	/// The player will immediately face `West`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_left(&mut self) {
		self.character.set_facing(sprite::Facing::West);
		self.character.accel_x = -1;
	}

	/// The player will immediately face `East`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_right(&mut self) {
		self.character.set_facing(sprite::Facing::East);
		self.character.accel_x = 1;
	}

	pub fn start_moving_up(&mut self) {
		self.character.accel_y = -1;
	}

	pub fn start_moving_down(&mut self) {
		self.character.accel_y = 1;
	}

	/// The player will immediately cease acceleration.
	/// They will still be facing the same direction as before this call.
	pub fn stop_moving_horizontally(&mut self, hard_stop: bool) {
		self.character.accel_x = 0;
		if hard_stop {
			self.character.velocity_x = units::Velocity(0.0);
		}		
	}

	pub fn stop_moving_vertically(&mut self, hard_stop: bool) {
		self.character.accel_y = 0;
		if hard_stop {
			self.character.velocity_y = units::Velocity(0.0);
		}
	}

	pub fn give_bat(&mut self) {
		self.cricket_bat = true;
	}

	pub fn take_bat(&mut self) {
		self.cricket_bat = false;
	}

	pub fn has_bat(&self) -> bool {
		self.cricket_bat
	}

	pub fn start_teleport_timer(&mut self) {
		self.teleport_timer = 20;
	}

	pub fn is_teleporting(&mut self) -> bool {
		self.teleport_timer > 0
	}

	pub fn start_immunity(&mut self) {
		self.immunity_timer = 60;
	}

	pub fn is_immune(&mut self) -> bool {
		self.immunity_timer > 0
	}

	pub fn hit_player(&mut self) {
		self.health = self.health - 1;
	}

	pub fn get_health(&mut self) -> u32 {
		self.health
	}

	pub fn get_follow_coords(&self) -> (units::Game, units::Game) {
		(self.follow_x, self.follow_y)
	}
}
