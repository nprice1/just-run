use std::collections::hashmap::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::Game;
use game::map;

use game::units;

use game::common;
use game::common::Character;

type MotionTup = (sprite::Motion, sprite::Facing);

static SPRITE_NUM_FRAMES:  units::Frame  = 3;
static SPRITE_FPS:         units::Fps    = 20;

// Sprite locations
static CRICKET_FRAME: units::Tile = units::Tile(1);
static KILLZOMBIE_FRAME: units::Tile = units::Tile(2);
static WIPEOUT_FRAME: units::Tile = units::Tile(3);
static FREEZE_FRAME: units::Tile = units::Tile(7);
static TELEPORT_FRAME: units::Tile = units::Tile(8);
static TELEPORT_ANIMATION_FRAME: units::Tile = units::Tile(0);
static NUKE_FRAME: units::Tile = units::Tile(9);
static BAD_FRAME: units::Tile = units::Tile(10);

// Animation frames 
static WIPEOUT_ANIMATION_FRAME: units::Tile = units::Tile(1);
static NUKE_ANIMATION_FRAME: units::Tile = units::Tile(1);

pub trait Powerup {
	fn draw(&self, display: &graphics::Graphics);
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map);
	fn damage_rectangle(&self) -> Rectangle;
	fn get_type(&self) -> int;
	fn toggle_debuff(&mut self);
	fn is_debuff(&self) -> bool;
	fn is_finished(&mut self) -> bool;
	fn set_timer(&mut self);
	fn get_map_x(&self) -> units::Game;
	fn get_map_y(&self) -> units::Game;
}

pub struct CricketBat {
	character: Character, 
	is_debuff: bool
}

pub struct KillZombie {
	character: Character, 
	animation_timer: int,
	is_debuff: bool
}

pub struct WipeOut {
	character: Character, 
	animation_sprite: Vec<Box<sprite::Updatable<units::Game>>>, 
	animation_timer: int,
	is_debuff: bool
}

pub struct Freeze {
	character: Character, 
	is_debuff: bool
}

pub struct Teleport {
	character: Character, 
	animation_sprite: Vec<Box<sprite::Updatable<units::Game>>>, 
	animation_timer: int,
	is_debuff: bool
}

pub struct Nuke {
	character: Character, 
	alternate_sprites: HashMap<MotionTup, Box<sprite::Updatable<units::Game>>>,
	animation_sprite: Vec<Box<sprite::Updatable<units::Game>>>, 
	animation_timer: int,
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
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		;
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
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

	fn is_finished(&mut self) -> bool {
		true
	}

	fn set_timer(&mut self) {
		;
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}
}

impl KillZombie {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> KillZombie {

		let mut new_powerup = KillZombie { 
			character: common::Character::new(x, y), 
			animation_timer: 0,
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
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		;
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
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

	fn is_finished(&mut self) -> bool {
		match self.animation_timer {
			0 => { true },
			_ => { self.animation_timer = self.animation_timer - 1; false }
		}
	}

	fn set_timer(&mut self) {
		self.animation_timer = 1;
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}
}

impl WipeOut {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> WipeOut {

		let animation: Vec<Box<sprite::Updatable<units::Game>>> = Vec::new();

		let mut new_powerup = WipeOut { 
			character: common::Character::new(x, y), 
			animation_sprite: animation,
			animation_timer: 0,
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

		let asset_path = "assets/base/explosion.bmp".to_string();
		let motion_frame = WIPEOUT_ANIMATION_FRAME;
		let facing_frame = units::Tile(7);
		let animation_sprite = box sprite::AnimatedSprite::new(
			display, asset_path,
			(motion_frame, facing_frame),
			(units::Tile(5), units::Tile(5)),
			SPRITE_NUM_FRAMES, SPRITE_FPS
		).unwrap() as Box<sprite::Updatable<_>>;
		self.animation_sprite.push(animation_sprite);
	}
}

impl Powerup for WipeOut {
	fn draw(&self, display: &graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		if self.animation_timer > 0 {
			self.animation_sprite.get(0).draw(display, (correction_x - units::Game(60.0), correction_y - units::Game(60.0)));
		} else {
			self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
		}
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		self.animation_sprite.get_mut(0).update(elapsed_time);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
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

	fn is_finished(&mut self) -> bool {
		match self.animation_timer {
			0 => { true },
			_ => { self.animation_timer = self.animation_timer - 1; false }
		}
	}

	fn set_timer(&mut self) {
		self.animation_timer = 5;
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
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
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		;
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
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

	fn is_finished(&mut self) -> bool {
		true
	}

	fn set_timer(&mut self) {
		;
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}
}

impl Teleport {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Teleport {

		let animation: Vec<Box<sprite::Updatable<units::Game>>> = Vec::new();

		let mut new_powerup = Teleport { 
			character: common::Character::new(x, y), 
			animation_sprite: animation,
			animation_timer: 0,
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
			let motion_frame = TELEPORT_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});

		let asset_path = "assets/base/teleport.bmp".to_string();
		let motion_frame = TELEPORT_ANIMATION_FRAME;
		let facing_frame = units::Tile(0);
		let animation_sprite = box sprite::Sprite::new(
			display, 
			(motion_frame, facing_frame),
			(units::Tile(3), units::Tile(3)),
			asset_path
		) as Box<sprite::Updatable<_>>;
		self.animation_sprite.push(animation_sprite);
	}
}

impl Powerup for Teleport {
	fn draw(&self, display: &graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		if self.animation_timer > 0 {
			self.animation_sprite.get(0).draw(display, (correction_x - units::Game(60.0), correction_y - units::Game(60.0)));
		} else {
			self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
		}
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		self.animation_sprite.get_mut(0).update(elapsed_time);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
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

	fn is_finished(&mut self) -> bool {
		match self.animation_timer {
			0 => { true },
			_ => { self.animation_timer = self.animation_timer - 1; false }
		}
	}

	fn set_timer(&mut self) {
		self.animation_timer = 4;
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}
}

impl Nuke {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Nuke {

		let animation: Vec<Box<sprite::Updatable<units::Game>>> = Vec::new();

		let mut new_powerup = Nuke { 
			character: common::Character::new(x, y), 
			alternate_sprites: HashMap::<MotionTup, Box<sprite::Updatable<_>>>::new(),
			animation_sprite: animation,
			animation_timer: 0,
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


		let asset_path = "assets/base/nuke.bmp".to_string();
		let motion_frame = NUKE_ANIMATION_FRAME;

		let facing_frame = units::Tile(10);

		let animation_sprite = box sprite::AnimatedSprite::new(
			display, asset_path,
			(motion_frame, facing_frame),
			(units::Tile(20), units::Tile(20)),
			SPRITE_NUM_FRAMES, 60
		).unwrap() as Box<sprite::Updatable<_>>;
		self.animation_sprite.push(animation_sprite);
	}
}

impl Powerup for Nuke {
	fn draw(&self, display: &graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		if self.animation_timer > 0 {
			self.animation_sprite.get(0).draw(display, (correction_x - units::Game(250.0), correction_y - units::Game(250.0)));
		} else if self.is_debuff {
			self.alternate_sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
		} else {
			self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
		}
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		self.animation_sprite.get_mut(0).update(elapsed_time);
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
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

	fn is_finished(&mut self) -> bool {
		match self.animation_timer {
			0 => { true },
			_ => { self.animation_timer = self.animation_timer - 1; false }
		}
	}

	fn set_timer(&mut self) {
		self.animation_timer = 5;
	}

	fn get_map_x(&self) -> units::Game {
		self.character.map_center_x()
	}

	fn get_map_y(&self) -> units::Game {
		self.character.map_center_y()
	}
}
