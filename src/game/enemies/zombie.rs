use game::collisions::Rectangle;
use game::map;
use game::sprite;
use game::graphics;

use game::units;

use game::common;
use game::common::Character;

static SPRITE_NUM_FRAMES:  units::Frame  = 2;
static SPRITE_FPS:         units::Fps    = 20;

// Slow Zombie
static SLOW_WALK_FRAME:  units::Tile = units::Tile(4);

static SLOW_WALKING_ACCEL:  units::Acceleration  = units::Acceleration(0.00003007812);
static SLOW_MAX_VELOCITY: units::Velocity      = units::Velocity(0.15859375);

// Crazy Zombie
static CRAZY_WALK_FRAME:  units::Tile = units::Tile(6);

static CRAZY_START_WALKING_ACCEL:  units::Acceleration  = units::Acceleration(0.00063007812);
static CRAZY_CHASING_WALKING_ACCEL: units::Acceleration = units::Acceleration(0.00183007812);
static CRAZY_MAX_VELOCITY: units::Velocity      = units::Velocity(0.15859375);

// Random Zombie
static RANDOM_WALK_FRAME:  units::Tile = units::Tile(7);

static RANDOM_WALKING_ACCEL: units::Acceleration = units::Acceleration(0.00183007812);
static RANDOM_MAX_VELOCITY: units::Velocity      = units::Velocity(0.20859375);

// Cloud Zombie
static CLOUD_WALK_FRAME:  units::Tile = units::Tile(6);

static CLOUD_WALKING_ACCEL: units::Acceleration = units::Acceleration(0.00083007812);
static CLOUD_MAX_VELOCITY: units::Velocity      = units::Velocity(0.05859375);

// y-offsets for different horizontal facings.
static WEST_OFFSET: units::Tile = units::Tile(2);
static EAST_OFFSET: units::Tile = units::Tile(3);

pub trait Zombie {
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map);
	fn set_acceleration(&mut self, player_x: units::Game, player_y: units::Game);
	fn draw(&self, display: &mut graphics::Graphics);
	fn damage_rectangle(&self) -> Rectangle;
	fn zombie_type(&self) -> i32;
	fn get_target(&self) -> (units::Game, units::Game);
	fn get_x(&self) -> units::Game;
	fn get_y(&self) -> units::Game;
	fn get_map_x(&self) -> units::Game;
	fn get_map_y(&self) -> units::Game;
	fn kill_zombie(&mut self);
	fn is_killed(&mut self) -> bool;
}

pub struct SlowZombie {
	character: Character
}

pub struct CrazyZombie {
	character: Character,
	chasing:   bool
}

pub struct RandomZombie {
	character: Character
}

pub struct CloudZombie {
	character: Character,
	chasing:   bool
}

impl SlowZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> SlowZombie {

		let mut new_slow_zombie = SlowZombie { 
			character: common::Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_slow_zombie.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_slow_zombie
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.load_killed_sprite(display);
		self.character.sprites.insert(movement, {
			let asset_path = "assets/base/Npc/NpcWeed.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = SLOW_WALK_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => WEST_OFFSET,
				sprite::Facing::East => EAST_OFFSET
			};
				
			Box::new( sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) ) as Box<dyn sprite::Updatable<_>>
		});
	}
}

impl Zombie for SlowZombie {
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		// calculate current position
		self.character.elapsed_time = elapsed_time;

		// update sprite
		self.character.current_motion(); // update motion once at beginning of frame for consistency
		if self.character.accel_x < 0 {
			self.character.set_facing(sprite::Facing::West);
		}
		else {
		 	self.character.set_facing(sprite::Facing::East);
		} 
		self.character.sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		if self.character.is_killed() {
			self.character.killed_sprite.get_mut(0).unwrap().update(elapsed_time);
		}

		// run physics sim
		self.character.update_x(map, SLOW_WALKING_ACCEL, SLOW_MAX_VELOCITY);
		self.character.update_y(map, SLOW_WALKING_ACCEL, SLOW_MAX_VELOCITY);
	}

	fn set_acceleration(&mut self, player_x: units::Game, player_y: units::Game) {
		self.character.accel_x = match self.character.map_center_x() {
			ref center if center < &player_x => 1,
			ref center if center > &player_x => -1,
			_				            => 0
		};
		self.character.accel_y = match self.character.map_center_y() {
			ref center if center < &player_y => 1, 
			ref center if center > &player_y => -1, 
			_				            => 0
		};
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		self.character.draw(display);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
	}

	fn zombie_type(&self) -> i32 {
		1
	}

	fn get_target(&self) -> (units::Game, units::Game) {
		(self.character.target_x, self.character.target_y)
	}

	fn get_x(&self) -> units::Game {
		self.character.center_x()
	}

	fn get_y(&self) -> units::Game {
		self.character.center_y()
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}

	fn kill_zombie(&mut self) {
		self.character.kill_character();
	}

	fn is_killed(&mut self) -> bool {
		match self.character.killed {
			0 => { true },
			_ => { self.character.killed = self.character.killed - 1; false }
		}
	}
}

impl CrazyZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> CrazyZombie {

		let mut new_crazy_zombie = CrazyZombie { 
			character: common::Character::new(x, y),
			chasing:   false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_crazy_zombie.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_crazy_zombie
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.load_killed_sprite(display);
		self.character.sprites.insert(movement, {
			let asset_path = "assets/base/Npc/NpcWeed.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = CRAZY_WALK_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => WEST_OFFSET,
				sprite::Facing::East => EAST_OFFSET
			};
					
			Box::new( sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) ) as Box<dyn sprite::Updatable<_>>
		});
	}
}

impl Zombie for CrazyZombie {
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		// calculate current position
		self.character.elapsed_time = elapsed_time;

		// update sprite
		self.character.current_motion(); // update motion once at beginning of frame for consistency
		if self.character.accel_x < 0 {
			self.character.set_facing(sprite::Facing::West);
		}
		else {
		 	self.character.set_facing(sprite::Facing::East);
		} 
		self.character.sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		if self.character.is_killed() {
			self.character.killed_sprite.get_mut(0).unwrap().update(elapsed_time);
		}

		// set proper acceleration
		let acceleration = match self.chasing {
			true => CRAZY_CHASING_WALKING_ACCEL,
			false => CRAZY_START_WALKING_ACCEL
		};

		// run physics sim
		self.character.update_x(map, acceleration, CRAZY_MAX_VELOCITY);
		self.character.update_y(map, acceleration, CRAZY_MAX_VELOCITY);
	}

	fn set_acceleration(&mut self, player_x: units::Game, player_y: units::Game) {
		// if close to player, start chasing
		self.chasing = match self.character.distance( player_x, player_y ) {
			d if d < 100.0 => true,
			_              => false
		};

		// keep going to target unless it has been reached
		if !self.chasing {
			self.character.set_new_target();
		} else {
		    self.character.target_x = player_x;
			self.character.target_y = player_y;
		}

		self.character.accel_x = match self.character.map_center_x() {
			ref center if center < &self.character.target_x => 1,
			ref center if center > &self.character.target_x => -1,
			_				            	 		   => 0
		};
		self.character.accel_y = match self.character.map_center_y() {
			ref center if center < &self.character.target_y => 1, 
			ref center if center > &self.character.target_y => -1, 
			_				            	 		   => 0
		};
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		self.character.draw(display);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
	}

	fn zombie_type(&self) -> i32 {
		2
	}

	fn get_target(&self) -> (units::Game, units::Game) {
		(self.character.target_x, self.character.target_y)
	}

	fn get_x(&self) -> units::Game {
		self.character.center_x()
	}

	fn get_y(&self) -> units::Game {
		self.character.center_y()
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}

	fn kill_zombie(&mut self) {
		self.character.kill_character();
	}

	fn is_killed(&mut self) -> bool {
		match self.character.killed {
			0 => { true },
			_ => { self.character.killed = self.character.killed - 1; false }
		}
	}
}

impl RandomZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> RandomZombie {

		let mut new_random_zombie = RandomZombie { 
			character: common::Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_random_zombie.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_random_zombie
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.load_killed_sprite(display);
		self.character.sprites.insert(movement, {
			let asset_path = "assets/base/Npc/NpcCemet.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = RANDOM_WALK_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => units::Tile(1),
				sprite::Facing::East => units::Tile(2)
			};

			Box::new( sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) ) as Box<dyn sprite::Updatable<_>>
		});
	}
}

impl Zombie for RandomZombie {
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		// calculate current position
		self.character.elapsed_time = elapsed_time;

		// update sprite
		self.character.current_motion(); // update motion once at beginning of frame for consistency
		if self.character.accel_x < 0 {
			self.character.set_facing(sprite::Facing::West);
		}
		else {
		 	self.character.set_facing(sprite::Facing::East);
		} 
		self.character.sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		if self.character.is_killed() {
			self.character.killed_sprite.get_mut(0).unwrap().update(elapsed_time);
		}

		// run physics sim
		self.character.update_x(map, RANDOM_WALKING_ACCEL, RANDOM_MAX_VELOCITY);
		self.character.update_y(map, RANDOM_WALKING_ACCEL, RANDOM_MAX_VELOCITY);
	}

	#[allow(unused_variables)]
	fn set_acceleration(&mut self, player_x: units::Game, player_y: units::Game) {
		self.character.set_new_random_target();

		self.character.accel_x = match self.character.map_center_x() {
			ref center if center < &self.character.target_x => 1,
			ref center if center > &self.character.target_x => -1,
			_				            	 		   => 0
		};
		self.character.accel_y = match self.character.map_center_y() {
			ref center if center < &self.character.target_y => 1, 
			ref center if center > &self.character.target_y => -1, 
			_				            	 		   => 0
		};
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		self.character.draw(display);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
	}

	fn zombie_type(&self) -> i32 {
		3
	}

	fn get_target(&self) -> (units::Game, units::Game) {
		(self.character.target_x, self.character.target_y)
	}

	fn get_x(&self) -> units::Game {
		self.character.center_x()
	}

	fn get_y(&self) -> units::Game {
		self.character.center_y()
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}

	fn kill_zombie(&mut self) {
		self.character.kill_character();
	}

	fn is_killed(&mut self) -> bool {
		match self.character.killed {
			0 => { true },
			_ => { self.character.killed = self.character.killed - 1; false }
		}
	}
}

impl CloudZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> CloudZombie {

		let mut new_cloud_zombie = CloudZombie { 
			character: common::Character::new(x, y), 
			chasing:   false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_cloud_zombie.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_cloud_zombie
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.load_killed_sprite(display);
		self.character.sprites.insert(movement,  {
			let asset_path = "assets/base/Npc/NpcMaze.bmp".to_string();
			let (_, facing) = movement;
			let motion_frame = CLOUD_WALK_FRAME;

			let facing_frame = match facing {
				sprite::Facing::West => WEST_OFFSET,
				sprite::Facing::East => EAST_OFFSET
			};

			Box::new( sprite::AnimatedSprite::new(
				display, asset_path,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				SPRITE_NUM_FRAMES, SPRITE_FPS
			).unwrap() ) as Box<dyn sprite::Updatable<_>>
		});
	}
}

impl Zombie for CloudZombie {
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		// calculate current position
		self.character.elapsed_time = elapsed_time;

		// update sprite
		self.character.current_motion(); // update motion once at beginning of frame for consistency
		if self.character.accel_x < 0 {
			self.character.set_facing(sprite::Facing::West);
		}
		else {
		 	self.character.set_facing(sprite::Facing::East);
		} 
		self.character.sprites.get_mut(&self.character.movement).unwrap().update(elapsed_time);
		if self.character.is_killed() {
			self.character.killed_sprite.get_mut(0).unwrap().update(elapsed_time);
		}

		// run physics sim
		self.character.update_x(map, CLOUD_WALKING_ACCEL, CLOUD_MAX_VELOCITY);
		self.character.update_y(map, CLOUD_WALKING_ACCEL, CLOUD_MAX_VELOCITY);
	}

	fn set_acceleration(&mut self, player_x: units::Game, player_y: units::Game) {
		// if close to player, start chasing
		self.chasing = match self.character.distance( player_x, player_y ) {
			d if d < 50.0 => true,
			_             => false
		};

		// keep going to target unless it has been reached
		if !self.chasing {
			self.character.set_new_target();
		} else {
			self.character.target_x = player_x;
			self.character.target_y = player_y;
		}

		self.character.accel_x = match self.character.map_center_x() {
			ref center if center < &self.character.target_x => 1,
			ref center if center > &self.character.target_x => -1,
			_				            	 		   => 0
		};
		self.character.accel_y = match self.character.map_center_y() {
			ref center if center < &self.character.target_y => 1, 
			ref center if center > &self.character.target_y => -1, 
			_				            	 		   => 0
		};
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		self.character.draw(display);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
	}

	fn zombie_type(&self) -> i32 {
		4
	}

	fn get_target(&self) -> (units::Game, units::Game) {
		(self.character.target_x, self.character.target_y)
	}

	fn get_x(&self) -> units::Game {
		self.character.center_x()
	}

	fn get_y(&self) -> units::Game {
		self.character.center_y()
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}

	fn kill_zombie(&mut self) {
		self.character.kill_character();
	}

	fn is_killed(&mut self) -> bool {
		match self.character.killed {
			0 => { true },
			_ => { self.character.killed = self.character.killed - 1; false }
		}
	}
}
