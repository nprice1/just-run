use std::collections::hashmap::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::common;
use game::common::Character;

use game::units;

static SCRAPPED_X_OFFSET: units::Tile = units::Tile(29);
static SCRAPPED_Y_OFFSET: units::Tile = units::Tile(1);
static PARTS_Y_OFFSET:    units::Tile = units::Tile(3);
static PROP_OFFSET:       units::Tile = units::Tile(17);
static WIND_OFFSET:       units::Tile = units::Tile(22);
static BAR_OFFSET:        units::Tile = units::Tile(26);

pub struct Helicopter {
	pub x: units::Game,
	pub y: units::Game,
	pub map_x: units::Game, 
	pub map_y: units::Game,

	sprites:  HashMap<sprite::Facing, Box<sprite::Updatable<units::Game>>>,
}

pub trait Part {
	fn draw(&self, display: &mut graphics::Graphics);
	fn damage_rectangle(&self) -> Rectangle;
	fn part_type(&self) -> int;
	fn get_x(&self) -> units::Game;
	fn get_y(&self) -> units::Game;
	fn get_map_x(&self) -> units::Game;
	fn get_map_y(&self) -> units::Game;
}

pub struct Prop {
	character: Character
}

pub struct Windshield {
	character: Character
}

pub struct Bar {
	character: Character
}

impl Helicopter {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Helicopter {
		
		let sprite_map = HashMap::<sprite::Facing, Box<sprite::Updatable<_>>>::new();

		let mut new_helicopter = Helicopter { 
			x: x, y: y,
			map_x: x, map_y: y,

			sprites: sprite_map,
		};

		new_helicopter.load_sprite(display);

		new_helicopter
	}

	fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics) {

		self.sprites.find_or_insert_with(sprite::West,
			|_| -> Box<sprite::Updatable<_>> {
				let asset_path = "assets/base/heli.bmp".to_string();
				let sprite_x = SCRAPPED_X_OFFSET;
				let sprite_y = SCRAPPED_Y_OFFSET;

				box sprite::Sprite::new(
						display, 
						(sprite_x, sprite_y), 
						(units::Tile(4), units::Tile(4)),
						asset_path
					) as Box<sprite::Updatable<_>>
			}
		);
	}

	pub fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.x + units::HalfTile(1), y: self.y + units::HalfTile(2),
			width: units::Game(5.0), height: units::Game(5.0),
		}
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		let sprite_ref = self.sprites.get_mut(&sprite::West);
		sprite_ref.update(elapsed_time);
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		self.sprites.get(&sprite::West).draw(display, (self.x, self.y));
	}
}

impl Prop {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Prop {

		let mut new_prop = Prop { 
			character: Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_prop.load_sprite(display, (*motion, *facing));
			}
		}

		new_prop
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/heli.bmp".to_string();
			let motion_frame = PROP_OFFSET;

			let facing_frame = PARTS_Y_OFFSET;

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(3), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});
	}
}

impl Part for Prop {
    fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::HalfTile(1), y: self.character.y + units::HalfTile(2),
			width: units::Game(5.0), height: units::Game(5.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> int {
		1
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
}

impl Windshield {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Windshield {

		let mut new_wind = Windshield { 
			character: Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_wind.load_sprite(display, (*motion, *facing));
			}
		}

		new_wind
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/heli.bmp".to_string();
			let motion_frame = WIND_OFFSET;

			let facing_frame = PARTS_Y_OFFSET;

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});
	}
}

impl Part for Windshield {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::HalfTile(1), y: self.character.y + units::HalfTile(2),
			width: units::Game(5.0), height: units::Game(5.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> int {
		2
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
}

impl Bar {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Bar {

		let mut new_bar = Bar { 
			character: Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_bar.load_sprite(display, (*motion, *facing));
			}
		}

		new_bar
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/heli.bmp".to_string();
			let motion_frame = BAR_OFFSET;

			let facing_frame = PARTS_Y_OFFSET;

			box sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(2), units::Tile(1)),
				asset_path
			) as Box<sprite::Updatable<_>>
		});
	}
}

impl Part for Bar {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::HalfTile(1), y: self.character.y + units::HalfTile(2),
			width: units::Game(5.0), height: units::Game(5.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> int {
		3
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
}