use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::Game;

use game::units;

use game::common;
use game::common::Character;

type MotionTup = (sprite::Motion, sprite::Facing);

static SPRITE_NUM_FRAMES:  units::Frame  = 2;
static SPRITE_FPS:         units::Fps    = 20;

// Sprite locations
static SPEEDUP_FRAME: units::Tile = units::Tile(1);
static KILLZOMBIE_FRAME: units::Tile = units::Tile(2);
static WIPEOUT_FRAME: units::Tile = units::Tile(3);
static FREEZE_FRAME: units::Tile = units::Tile(7);

static X_BOX: Rectangle = Rectangle {
	x: units::Game(6.0), y: units::Game(10.0), 
	width: units::Game(20.0), height: units::Game(12.0)
};
static Y_BOX: Rectangle = Rectangle {
	x: units::Game(10.0), y: units::Game(6.0), 
	width: units::Game(12.0), height: units::Game(30.0)
};

pub trait Powerup {
	fn draw(&self, display: &graphics::Graphics);
	fn damage_rectangle(&self) -> Rectangle;
	fn get_type(&self) -> int;
}

pub struct CricketBat {
	character: Character
}

pub struct KillZombie {
	character: Character
}

pub struct WipeOut {
	character: Character
}

pub struct Freeze {
	character: Character
}

impl CricketBat {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> CricketBat {

		let mut new_slow_zombie = CricketBat { 
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = SPEEDUP_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});
	}
}

impl Powerup for CricketBat {
	fn draw(&self, display: &graphics::Graphics) {
		self.character.sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
	}

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + X_BOX.left(),
			y: self.character.y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	fn get_type(&self) -> int {
		1
	}
}

impl KillZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> KillZombie {

		let mut new_crazy_zombie = KillZombie { 
			character: common::Character::new(x, y)
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = KILLZOMBIE_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});
	}
}

impl Powerup for KillZombie {
	fn draw(&self, display: &graphics::Graphics) {
		self.character.sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
	}

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + X_BOX.left(),
			y: self.character.y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	fn get_type(&self) -> int {
		2
	}
}

impl WipeOut {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> WipeOut {

		let mut new_random_zombie = WipeOut { 
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = WIPEOUT_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});
	}
}

impl Powerup for WipeOut {
	fn draw(&self, display: &graphics::Graphics) {
		self.character.sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
	}

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + X_BOX.left(),
			y: self.character.y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	fn get_type(&self) -> int {
		3
	}
}

impl Freeze {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Freeze {

		let mut new_cloud_zombie = Freeze { 
			character: common::Character::new(x, y)
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = FREEZE_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::AnimatedSprite::new(
				display, asset_path,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				SPRITE_NUM_FRAMES, SPRITE_FPS
			).unwrap() as Box<sprite::Updatable<_>>
		});
	}
}

impl Powerup for Freeze {
	fn draw(&self, display: &graphics::Graphics) {
		self.character.sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
	}

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + X_BOX.left(),
			y: self.character.y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	fn get_type(&self) -> int {
		4
	}
}

// TODO: Add trap powerup with benefits(?)
