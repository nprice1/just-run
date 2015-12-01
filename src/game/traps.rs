use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::Game;
use game::map;

use game::units;

use game::common;
use game::common::Character;

type MotionTup = (sprite::Motion, sprite::Facing);

static SPRITE_NUM_FRAMES:  units::Frame  = 2;
static SPRITE_FPS:         units::Fps    = 20;

// Sprite locations
const BEAR_TRAP_FRAME: units::Tile = units::Tile(0);

// Animation frames 
const BEAR_TRAP_CLOSED_FRAME: units::Tile = units::Tile(1);

pub trait Trap {
	fn draw(&self, display: &mut graphics::Graphics);
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map);
	fn damage_rectangle(&self) -> Rectangle;
	fn get_type(&self) -> i32;
	fn is_finished(&mut self) -> bool;
	fn set_timer(&mut self);
	fn get_map_x(&self) -> units::Game;
	fn get_map_y(&self) -> units::Game;
}

pub struct BearTrap {
	character: Character, 
	animation_sprite: Vec<Box<sprite::Updatable<units::Game>>>, 
	animation_timer: i32
}

impl BearTrap {
	pub fn new(graphics: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> BearTrap {

		let animation: Vec<Box<sprite::Updatable<units::Game>>> = Vec::new();

		let mut new_trap = BearTrap { 
			character: common::Character::new(x, y), 
			animation_sprite: animation,
			animation_timer: 0
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_trap.load_sprite(graphics, (*motion, *facing));
			}
		}

		new_trap
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		let asset_path = "assets/base/traps.bmp".to_string();
		let motion_frame = BEAR_TRAP_FRAME;

		let facing_frame = units::Tile(0);

		let loaded_sprite = Box::new( sprite::Sprite::new(
			display,
			(motion_frame, facing_frame),
			(units::Tile(1), units::Tile(1)),
			asset_path
		) ) as Box<sprite::Updatable<_>>;

		self.character.sprites.insert(movement, loaded_sprite);

		let asset_path = "assets/base/traps.bmp".to_string();
		let motion_frame = BEAR_TRAP_CLOSED_FRAME;
		let facing_frame = units::Tile(0);
		let animation_sprite = Box::new( sprite::Sprite::new(
			display, 
			(motion_frame, facing_frame),
			(units::Tile(1), units::Tile(1)),
			asset_path,
		) ) as Box<sprite::Updatable<_>>;
		self.animation_sprite.push(animation_sprite);
	}
}

impl Trap for BearTrap {
	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.get_map_x() % common::SCREEN_CORRECTION;
		let correction_y = self.character.get_map_y() % common::SCREEN_CORRECTION;
		if self.animation_timer > 0 {
			self.animation_sprite.get(0).unwrap().draw(display, (correction_x, correction_y));
		} else {
			self.character.sprites.get(&self.character.movement).unwrap().draw(display, (correction_x, correction_y));
		}
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		;
	}

	fn damage_rectangle(&self) -> Rectangle {
		self.character.damage_rectangle()
	}

	fn get_type(&self) -> i32 {
		1
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