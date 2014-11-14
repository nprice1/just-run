use std::collections::hashmap::HashMap;

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

static COMPLETE_X_OFFSET: units::Tile = units::Tile(12);
static SCRAPPED_X_OFFSET: units::Tile = units::Tile(0);
static SCRAPPED_Y_OFFSET: units::Tile = units::Tile(0);
static PERM_1_OFFSET:     units::Tile = units::Tile(6);
static PERM_2_OFFSET:     units::Tile = units::Tile(12);
static PERM_3_OFFSET:     units::Tile = units::Tile(0);
static PERM_4_OFFSET:     units::Tile = units::Tile(6);
static PERM_5_OFFSET:     units::Tile = units::Tile(12);
static PERM_6_OFFSET:     units::Tile = units::Tile(6);
static PERM_1_2_OFFSET:   units::Tile = units::Tile(0);
static PERM_3_5_OFFSET:   units::Tile = units::Tile(2);
static PERM_6_7_OFFSET:   units::Tile = units::Tile(4);
static PARTS_Y_OFFSET:    units::Tile = units::Tile(4);
static TIRE_OFFSET:       units::Tile = units::Tile(0);
static DOOR_OFFSET:       units::Tile = units::Tile(2);
static ENGINE_OFFSET:     units::Tile = units::Tile(4);

static NUM_PARTS:                uint = 3;
static POSSIBLE_CONFIGS:         uint = 8;

pub struct Car {
	pub x: units::Game,
	pub y: units::Game,
	pub map_x: units::Game, 
	pub map_y: units::Game,

	sprites:  HashMap<uint, Box<sprite::Updatable<units::Game>>>,
	parts:    HashMap<uint, uint>
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
		
		let sprite_map = HashMap::<uint, Box<sprite::Updatable<_>>>::new();
		let part_map = HashMap::<uint, uint>::new();

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

		for i in range(0, POSSIBLE_CONFIGS) {
			match i {
				0 => {
					self.sprites.find_or_insert_with(0u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = SCRAPPED_X_OFFSET;
							let sprite_y = SCRAPPED_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				1 => {
					self.sprites.find_or_insert_with(1u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_1_OFFSET;
							let sprite_y = PERM_1_2_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				2 => {
					self.sprites.find_or_insert_with(2u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_2_OFFSET;
							let sprite_y = PERM_1_2_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				3 => {
					self.sprites.find_or_insert_with(3u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_3_OFFSET;
							let sprite_y = PERM_3_5_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				4 => {
					self.sprites.find_or_insert_with(4u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_4_OFFSET;
							let sprite_y = PERM_3_5_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				5 => {
					self.sprites.find_or_insert_with(5u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_5_OFFSET;
							let sprite_y = PERM_3_5_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				6 => {
					self.sprites.find_or_insert_with(6u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = PERM_6_OFFSET;
							let sprite_y = PERM_6_7_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				_ => {
					self.sprites.find_or_insert_with(7u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/racer.bmp".to_string();
							let sprite_x = COMPLETE_X_OFFSET;
							let sprite_y = PERM_6_7_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(6), units::Tile(2)),
									asset_path
								) as Box<sprite::Updatable<_>>
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
		let sprite_ref = self.sprites.get_mut(&7u);
		sprite_ref.update(elapsed_time);
	}

	fn update_for_cinematic(&mut self) {
		self.x = self.x + units::Game(4.0);
	}

	fn draw(&self, display: &graphics::Graphics) {
		let mut config = 0u;
		if self.parts.len() == NUM_PARTS {
			config = 7u;
		} else if self.parts.contains_key(&1u) && self.parts.contains_key(&3u) {
			config = 2u;
		} else if self.parts.contains_key(&2u) && self.parts.contains_key(&3u) {
			config = 4u;
		} else if self.parts.contains_key(&1u) && self.parts.contains_key(&2u) {
			config = 6u;
		} else if self.parts.contains_key(&3u) {
			config = 5u;
		} else if self.parts.contains_key(&2u) {
			config = 3u;
		} else if self.parts.contains_key(&1u) {
			config = 1u;
		}
		self.sprites.get(&config).draw(display, (self.x, self.y));
	}

	fn add_part(&mut self, part_num: uint) {
		self.parts.insert(part_num, 1);
	}

	fn is_built(&self) -> bool {
		if self.parts.len() == NUM_PARTS {
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

	fn get_type(&self) -> int {
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/racer.bmp".to_string();
			let motion_frame = TIRE_OFFSET;

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

impl Part for Tire {
    fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::Tile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(32.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> uint {
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/racer.bmp".to_string();
			let motion_frame = ENGINE_OFFSET;

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

impl Part for Engine {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::HalfTile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(16.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> uint {
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

		self.character.sprites.find_or_insert_with(movement, |_| -> Box<sprite::Updatable<_>> {
			let asset_path = "assets/base/racer.bmp".to_string();
			let motion_frame = DOOR_OFFSET;

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

impl Part for Door {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::Tile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(32.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
		self.character.sprites.get(&self.character.movement).draw(display, (correction_x, correction_y));
	}

	fn part_type(&self) -> uint {
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