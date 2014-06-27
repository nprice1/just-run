use std::collections::hashmap::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::Game;

use game::units;

use game::common;
use game::common::Character;

type MotionTup = (sprite::Motion, sprite::Facing);

// Sprite locations
static CRICKET_FRAME: units::Tile = units::Tile(1);
static KILLZOMBIE_FRAME: units::Tile = units::Tile(2);
static WIPEOUT_FRAME: units::Tile = units::Tile(3);
static FREEZE_FRAME: units::Tile = units::Tile(7);
static STICKY_FRAME: units::Tile = units::Tile(8);
static NUKE_FRAME: units::Tile = units::Tile(9);
static BAD_FRAME: units::Tile = units::Tile(10);

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
	fn toggle_debuff(&mut self);
	fn is_debuff(&self) -> bool;
}

pub struct CricketBat {
	character: Character, 
	is_debuff: bool
}

pub struct KillZombie {
	character: Character, 
	is_debuff: bool
}

pub struct WipeOut {
	character: Character, 
	is_debuff: bool
}

pub struct Freeze {
	character: Character, 
	is_debuff: bool
}

pub struct StickyFeet {
	character: Character, 
	is_debuff: bool
}

pub struct Nuke {
	character: Character, 
	alternate_sprites: HashMap<MotionTup, Box<sprite::Updatable<units::Game>>>,
	is_debuff: bool
}

impl CricketBat {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> CricketBat {

		let mut new_powerup = CricketBat { 
			character: common::Character::new(x, y), 
			is_debuff: false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_powerup.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_powerup
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = CRICKET_FRAME;

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

	fn toggle_debuff(&mut self) {
		self.is_debuff = !self.is_debuff;
	}

	fn is_debuff(&self) -> bool {
		self.is_debuff
	}

	fn get_type(&self) -> int {
		1
	}
}

impl KillZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> KillZombie {

		let mut new_powerup = KillZombie { 
			character: common::Character::new(x, y), 
			is_debuff: false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_powerup.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_powerup
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

	fn toggle_debuff(&mut self) {
		self.is_debuff = !self.is_debuff;
	}

	fn is_debuff(&self) -> bool {
		self.is_debuff
	}

	fn get_type(&self) -> int {
		2
	}
}

impl WipeOut {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> WipeOut {

		let mut new_powerup = WipeOut { 
			character: common::Character::new(x, y), 
			is_debuff: false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_powerup.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_powerup
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

	fn toggle_debuff(&mut self) {
		self.is_debuff = !self.is_debuff;
	}

	fn is_debuff(&self) -> bool {
		self.is_debuff
	}

	fn get_type(&self) -> int {
		3
	}
}

impl Freeze {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Freeze {

		let mut new_powerup = Freeze { 
			character: common::Character::new(x, y), 
			is_debuff: false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_powerup.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_powerup
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = FREEZE_FRAME;

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

	fn toggle_debuff(&mut self) {
		self.is_debuff = !self.is_debuff;
	}

	fn is_debuff(&self) -> bool {
		self.is_debuff
	}

	fn get_type(&self) -> int {
		4
	}
}

impl StickyFeet {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> StickyFeet {

		let mut new_powerup = StickyFeet { 
			character: common::Character::new(x, y), 
			is_debuff: false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_powerup.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_powerup
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = STICKY_FRAME;

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

impl Powerup for StickyFeet {
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

	fn toggle_debuff(&mut self) {
		self.is_debuff = !self.is_debuff;
	}

	fn is_debuff(&self) -> bool {
		self.is_debuff
	}

	fn get_type(&self) -> int {
		5
	}
}

impl Nuke {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Nuke {

		let mut new_powerup = Nuke { 
			character: common::Character::new(x, y), 
			alternate_sprites: HashMap::<MotionTup, Box<sprite::Updatable<_>>>::new(),
			is_debuff: false
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_powerup.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_powerup
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = NUKE_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});

		self.alternate_sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/powerups.bmp".to_string();
			let motion_frame = BAD_FRAME;

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

impl Powerup for Nuke {
	fn draw(&self, display: &graphics::Graphics) {
		if self.is_debuff {
			self.alternate_sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
		} else {
			self.character.sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
		}
	}

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + X_BOX.left(),
			y: self.character.y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	fn toggle_debuff(&mut self) {
		self.is_debuff = !self.is_debuff;
	}

	fn is_debuff(&self) -> bool {
		self.is_debuff
	}

	fn get_type(&self) -> int {
		6
	}
}

// TODO: Add trap powerup with benefits(?)
