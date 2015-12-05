use std::collections::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::common;
use game::common::Character;
use game::vehicle::Vehicle;
use game::vehicle::Part;

use game::units;

static SPRITE_NUM_FRAMES:   units::Frame  = 4;
static SPRITE_FPS:          units::Fps    = 20;

const COMPLETE_X_OFFSET: units::Tile = units::Tile(0);
const SCRAPPED_X_OFFSET: units::Tile = units::Tile(0);
const SCRAPPED_Y_OFFSET: units::Tile = units::Tile(2);
const PERM_1_OFFSET:     units::Tile = units::Tile(0);
const PERM_2_OFFSET:     units::Tile = units::Tile(6);
const PERM_3_OFFSET:     units::Tile = units::Tile(12);
const PERM_4_OFFSET:     units::Tile = units::Tile(6);
const PERM_5_OFFSET:     units::Tile = units::Tile(0);
const PERM_6_OFFSET:     units::Tile = units::Tile(6);
const PERM_1_2_5_OFFSET: units::Tile = units::Tile(0);
const PERM_3_4_6_OFFSET: units::Tile = units::Tile(2);
const PARTS_Y_OFFSET:    units::Tile = units::Tile(4);
const TIRE_OFFSET:       units::Tile = units::Tile(2);
const DOOR_OFFSET:       units::Tile = units::Tile(0);
const ENGINE_OFFSET:     units::Tile = units::Tile(4);

static NUM_PARTS:                u32 = 3;
static POSSIBLE_CONFIGS:         u32 = 8;

pub struct Car {
	pub x: units::Game,
	pub y: units::Game,
	pub map_x: units::Game, 
	pub map_y: units::Game,

	sprites:  HashMap<u32, Box<sprite::Updatable<units::Game>>>,
	parts:    HashMap<u32, u32>
}

pub struct Tire {
	character: Character
}

pub struct Engine {
	character: Character
}

pub struct Door {
	character: Character
}

impl Car {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Car {
		
		let sprite_map = HashMap::<u32, Box<sprite::Updatable<_>>>::new();
		let part_map = HashMap::<u32, u32>::new();

		let mut new_car = Car { 
			x: x, y: y,
			map_x: x, map_y: y,

			sprites: sprite_map,
			parts: part_map
		};

		new_car.load_sprites(display);

		new_car
	}

	fn load_sprites(&mut self, 
	               display: &mut graphics::Graphics) {

		for i in 0.. POSSIBLE_CONFIGS {
			match i {
				0 => {
					self.sprites.insert(0u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = SCRAPPED_X_OFFSET;
							let sprite_y = SCRAPPED_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				1 => {
					self.sprites.insert(1u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_1_OFFSET;
							let sprite_y = PERM_1_2_5_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				2 => {
					self.sprites.insert(2u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_2_OFFSET;
							let sprite_y = PERM_1_2_5_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				3 => {
					self.sprites.insert(3u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_3_OFFSET;
							let sprite_y = PERM_3_4_6_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				4 => {
					self.sprites.insert(4u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_4_OFFSET;
							let sprite_y = PERM_3_4_6_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				5 => {
					self.sprites.insert(5u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_5_OFFSET;
							let sprite_y = PERM_1_2_5_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				6 => {
					self.sprites.insert(6u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_6_OFFSET;
							let sprite_y = PERM_3_4_6_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				_ => {
					self.sprites.insert(7u32, {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = COMPLETE_X_OFFSET;
							let sprite_y = COMPLETE_X_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(6), units::Tile(2)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				}
			}
		}
	}
}

impl Vehicle for Car {

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.x + units::Tile(3), y: self.y + units::Tile(1),
			width: units::Game(32.0 * 2.0), height: units::Game(16.0),
		}
	}

	fn update(&mut self, elapsed_time: units::Millis) {
		let sprite_ref = self.sprites.get_mut(&7u32);
		sprite_ref.unwrap().update(elapsed_time);
	}

	fn update_for_cinematic(&mut self) {
		self.x = self.x + units::Game(4.0);
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let mut config = 0u32;
		if self.parts.len() == NUM_PARTS as usize {
			config = 7u32;
		} else if self.parts.contains_key(&1u32) && self.parts.contains_key(&3u32) {
			config = 2u32;
		} else if self.parts.contains_key(&2u32) && self.parts.contains_key(&3u32) {
			config = 4u32;
		} else if self.parts.contains_key(&1u32) && self.parts.contains_key(&2u32) {
			config = 6u32;
		} else if self.parts.contains_key(&3u32) {
			config = 5u32;
		} else if self.parts.contains_key(&2u32) {
			config = 3u32;
		} else if self.parts.contains_key(&1u32) {
			config = 1u32;
		}
		self.sprites.get(&config).unwrap().draw(display, (self.x, self.y));
	}

	fn add_part(&mut self, part_num: u32) {
		self.parts.insert(part_num, 1);
	}

	fn is_built(&self) -> bool {
		if self.parts.len() == NUM_PARTS as usize {
			true
		} else {
			false
		}
	}

	fn get_x(&self) -> units::Game {
		self.x
	}

	fn get_y(&self) -> units::Game {
		self.y
	}

	fn get_map_x(&self) -> units::Game {
		self.map_x
	}

	fn get_map_y(&self) -> units::Game {
		self.map_y
	}

	fn get_type(&self) -> i32 {
		2
	}
}

impl Tire {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Tire {

		let mut new_tire = Tire { 
			character: Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_tire.load_sprite(display, (*motion, *facing));
			}
		}

		new_tire
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		let asset_path = "assets/base/racer.bmp".to_string();
		let motion_frame = TIRE_OFFSET;

		let facing_frame = PARTS_Y_OFFSET;

		let loaded_sprite = Box::new( sprite::Sprite::new(
			display,
			(motion_frame, facing_frame),
			(units::Tile(2), units::Tile(1)),
			asset_path
		) ) as Box<sprite::Updatable<_>>;

		self.character.sprites.insert(movement, loaded_sprite);
	}
}

impl Part for Tire {
    fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::Tile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(32.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.get_map_x() % common::SCREEN_CORRECTION;
		let correction_y = self.character.get_map_y() % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).unwrap().draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> u32 {
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

impl Engine {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Engine {

		let mut new_engine = Engine { 
			character: Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_engine.load_sprite(display, (*motion, *facing));
			}
		}

		new_engine
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		let asset_path = "assets/base/racer.bmp".to_string();
		let motion_frame = ENGINE_OFFSET;

		let facing_frame = PARTS_Y_OFFSET;

		let loaded_sprite = Box::new( sprite::Sprite::new(
			display,
			(motion_frame, facing_frame),
			(units::Tile(1), units::Tile(1)),
			asset_path
		) ) as Box<sprite::Updatable<_>>;

		self.character.sprites.insert(movement, loaded_sprite);
	}
}

impl Part for Engine {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::HalfTile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(16.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.get_map_x() % common::SCREEN_CORRECTION;
		let correction_y = self.character.get_map_y() % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).unwrap().draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> u32 {
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

impl Door {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Door {

		let mut new_door = Door { 
			character: Character::new(x, y)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				new_door.load_sprite(display, (*motion, *facing));
			}
		}

		new_door
	}

	pub fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics,
	               movement: (sprite::Motion, sprite::Facing)) {

		let asset_path = "assets/base/racer.bmp".to_string();
		let motion_frame = DOOR_OFFSET;

		let facing_frame = PARTS_Y_OFFSET;

		let loaded_sprite = Box::new( sprite::Sprite::new(
			display,
			(motion_frame, facing_frame),
			(units::Tile(2), units::Tile(2)),
			asset_path
		) ) as Box<sprite::Updatable<_>>;

		self.character.sprites.insert(movement, loaded_sprite);
	}
}

impl Part for Door {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::Tile(1), y: self.character.y + units::Tile(1),
			width: units::Game(32.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.get_map_x() % common::SCREEN_CORRECTION;
		let correction_y = self.character.get_map_y() % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).unwrap().draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> u32 {
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