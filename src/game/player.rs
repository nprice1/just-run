use std::collections::hashmap::HashMap;

use game::graphics;
use game::sprite;

use game::map;

use game::units;

use game::common;
use game::common::Character;

pub type MotionTup = (sprite::Motion, sprite::Facing);

// player sprite animation
static CHAR_OFFSET:        uint          = 12;
static CRICKET_OFFSET:     uint          = 10;
static SPRITE_NUM_FRAMES:  units::Frame  = 3;
static SPRITE_FPS:         units::Fps    = 20;

static WALKING_ACCEL:  units::Acceleration  = units::Acceleration(0.00083007812);
static MAX_VELOCITY: units::Velocity      = units::Velocity(0.17859375);

// motion
static STAND_FRAME: units::Tile   = units::Tile(0);

// horizontal facing (Facing)
static FACING_WEST: units::Tile  = units::Tile(0 + CHAR_OFFSET);
static FACING_EAST: units::Tile  = units::Tile(1 + CHAR_OFFSET);
static CRICKET_FACING_WEST: units::Tile  = units::Tile(0 + CRICKET_OFFSET);
static CRICKET_FACING_EAST: units::Tile  = units::Tile(1 + CRICKET_OFFSET);

pub struct Player {
	pub character: common::Character,
	cricket_sprites: HashMap<MotionTup, Box<sprite::Updatable<units::Game>>>,
	cricket_bat: bool, 
	sticky_count: int
}

impl Player {
	/// Loads and initializes a set of sprite-sheets for the various combinatoins of directions.
	/// (These incl: facing west and east for: standing, walking, jumping, falling.)
	///
	/// The player will spawn at `x` and `y`, though it will immediately be subject to gravity.
	/// The player is initailized `standing` facing `east`.
	/// The player will continue to fall until some collision is detected.
	pub fn new(graphics: &mut graphics::Graphics, x: units::Game, y: units::Game) -> Player {
		let cricket = HashMap::<MotionTup, Box<sprite::Updatable<_>>>::new();
		// construct new player
		let mut new_player = Player{
			character: common::Character::new(x, y),
			cricket_sprites: cricket,
			cricket_bat: false,
			sticky_count: 0
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

		// update sprite
		self.character.current_motion(); // update motion once at beginning of frame for consistency
		self.character.sprites.get_mut(&self.character.movement).update(elapsed_time);
		self.cricket_sprites.get_mut(&self.character.movement).update(elapsed_time);
		if self.character.is_killed() {
			self.character.killed_sprite.get_mut(0).update(elapsed_time);
		}

		// run physics sim
		self.character.update_x(map, WALKING_ACCEL, MAX_VELOCITY, self.sticky_count > 0);
		self.character.update_y(map, WALKING_ACCEL, MAX_VELOCITY, self.sticky_count > 0);

		if self.sticky_count > 0 {
		  self.sticky_count = self.sticky_count - 1;
		}
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
		self.cricket_sprites.find_or_insert_with(movement, |key| -> Box<sprite::Updatable<_>> {
			let file_path = "assets/MyChar.bmp".to_string();
			let (_, facing) = *key;
			let motion_frame = STAND_FRAME;

			let facing_frame = match facing {
				sprite::West => CRICKET_FACING_WEST,
				sprite::East => CRICKET_FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Standing, _) => {
					box sprite::Sprite::new(
						graphics, 
						(motion_frame, facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) as Box<sprite::Updatable<_>> 
				}

				// dynamic: 
				(sprite::Walking, _) => {
					box sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() as Box<sprite::Updatable<_>>
				}
			}
		});
		self.character.sprites.find_or_insert_with(movement, |key| -> Box<sprite::Updatable<_>> {
			let file_path = "assets/MyChar.bmp".to_string();
			let (_, facing) = *key;
			let motion_frame = STAND_FRAME;

			let facing_frame = match facing {
				sprite::West => FACING_WEST,
				sprite::East => FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Standing, _) => {
					box sprite::Sprite::new(
						graphics, 
						(motion_frame, facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) as Box<sprite::Updatable<_>> 
				}

				// dynamic: 
				(sprite::Walking, _) => {
					box sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() as Box<sprite::Updatable<_>>
				}
			}
		});
	}

	//draw the player
	pub fn draw(&self, display: &mut graphics::Graphics) {
		if self.cricket_bat {
			self.cricket_sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
		} else {
			self.character.draw(display);
		}
	}

	/// The player will immediately face `West`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_left(&mut self) {
		self.character.set_facing(sprite::West);
		self.character.accel_x = -1;
	}

	/// The player will immediately face `East`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_right(&mut self) {
		self.character.set_facing(sprite::East);
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
	pub fn stop_moving_horizontally(&mut self) {
		self.character.accel_x = 0;
	}

	pub fn stop_moving_vertically(&mut self) {
		self.character.accel_y = 0;
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

	pub fn give_sticky_feet(&mut self) {
		self.sticky_count = 500;
	}
}
