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
static BEAR_TRAP_FRAME: units::Tile = units::Tile(0);

// Animation frames 
static BEAR_TRAP_CLOSED_FRAME: units::Tile = units::Tile(1);

static X_BOX: Rectangle = Rectangle {
	x: units::Game(6.0), y: units::Game(10.0), 
	width: units::Game(20.0), height: units::Game(12.0)
};
static Y_BOX: Rectangle = Rectangle {
	x: units::Game(10.0), y: units::Game(6.0), 
	width: units::Game(12.0), height: units::Game(30.0)
};

pub trait Trap {
	fn draw(&self, display: &graphics::Graphics);
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map);
	fn damage_rectangle(&self) -> Rectangle;
	fn get_type(&self) -> int;
	fn is_finished(&mut self) -> bool;
	fn set_timer(&mut self);
}

pub struct BearTrap {
	character: Character, 
	animation_sprite: Vec<Box<sprite::Updatable<units::Game>>>, 
	animation_timer: int
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/traps.bmp".to_string();
			let motion_frame = BEAR_TRAP_FRAME;

			let facing_frame = units::Tile(0);

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});

		let asset_path = "assets/base/traps.bmp".to_string();
		let motion_frame = BEAR_TRAP_CLOSED_FRAME;
		let facing_frame = units::Tile(0);
		let animation_sprite = box sprite::Sprite::new(
			display, 
			(motion_frame, facing_frame),
			(units::Tile(1), units::Tile(1)),
			asset_path,
		) as Box<sprite::Updatable<_>>;
		self.animation_sprite.push(animation_sprite);
	}
}

impl Trap for BearTrap {
	fn draw(&self, display: &graphics::Graphics) {
		if self.animation_timer > 0 {
			self.animation_sprite.get(0).draw(display, (self.character.x, self.character.y));
		} else {
			self.character.sprites.get(&self.character.movement).draw(display, (self.character.x, self.character.y));
		}
	}

	#[allow(unused_variable)]
	fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		;
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

	fn is_finished(&mut self) -> bool {
		match self.animation_timer {
			0 => { true },
			_ => { self.animation_timer = self.animation_timer - 1; false }
		}
	}

	fn set_timer(&mut self) {
		self.animation_timer = 1;
	}
}