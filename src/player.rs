use game::graphics;
use game::sprite;

use game::map;

use game::units;

use game::common;
use game::common::Character;

// player sprite animation
static CHAR_OFFSET:        uint          = 12;
static SPRITE_NUM_FRAMES:  units::Frame  = 3;
static SPRITE_FPS:         units::Fps    = 20;

static WALKING_ACCEL:  units::Acceleration  = units::Acceleration(0.00083007812);
static MAX_VELOCITY: units::Velocity      = units::Velocity(0.17859375);

// motion
static STAND_FRAME: units::Tile   = units::Tile(0);

// horizontal facing (Facing)
static FACING_WEST: units::Tile  = units::Tile(0 + CHAR_OFFSET);
static FACING_EAST: units::Tile  = units::Tile(1 + CHAR_OFFSET);

/// Encapsulates the pysical motion of a player as it relates to
/// a sprite which can be animated, positioned, and drawn on the screen.
pub struct Player {
	character: common::Character
}


impl Player {
	/// Loads and initializes a set of sprite-sheets for the various combinatoins of directions.
	/// (These incl: facing west and east for: standing, walking, jumping, falling.)
	///
	/// The player will spawn at `x` and `y`, though it will immediately be subject to gravity.
	/// The player is initailized `standing` facing `east`.
	/// The player will continue to fall until some collision is detected.
	pub fn new(graphics: &mut graphics::Graphics, x: units::Game, y: units::Game) -> Player {
		// construct new player
		let mut new_player = Player{
			character: common::Character::new(x, y)
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
		self.character.sprites.find_or_insert_with(movement, |key| -> ~sprite::Updatable<_> {
			let file_path = ~"assets/base/MyChar.bmp";
			let (_, facing) = *key;
			let motion_frame = STAND_FRAME;

			let facing_frame = match facing {
				sprite::West => FACING_WEST,
				sprite::East => FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Standing, _) => {
					~sprite::Sprite::new(
						graphics, 
						(motion_frame, facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) as ~sprite::Updatable<_> 
				}

				// dynamic: 
				(sprite::Walking, _) => {
					~sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() as ~sprite::Updatable<_>
				}
			}
		});
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
	
	/// The player has been hit by a zombie
	pub fn take_damage(&mut self) {
		println!("zombie has collided with me! D:");
	}

}
