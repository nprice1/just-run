use std::collections::hashmap::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;
use game::common;
use game::common::Character;

use game::units;

static SPRITE_NUM_FRAMES:   units::Frame  = 4;
static SPRITE_FPS:          units::Fps    = 20;

static COMPLETE_X_OFFSET: units::Tile = units::Tile(0);
static SCRAPPED_X_OFFSET: units::Tile = units::Tile(29);
static SCRAPPED_Y_OFFSET: units::Tile = units::Tile(2);
static PERM_1_OFFSET:     units::Tile = units::Tile(0);
static PERM_2_OFFSET:     units::Tile = units::Tile(4);
static PERM_3_OFFSET:     units::Tile = units::Tile(9);
static PERM_4_OFFSET:     units::Tile = units::Tile(14);
static PERM_5_OFFSET:     units::Tile = units::Tile(19);
static PERM_6_OFFSET:     units::Tile = units::Tile(24);
static PERM_Y_OFFSET:     units::Tile = units::Tile(5);
static PARTS_Y_OFFSET:    units::Tile = units::Tile(3);
static PROP_OFFSET:       units::Tile = units::Tile(19);
static WIND_OFFSET:       units::Tile = units::Tile(24);
static BAR_OFFSET:        units::Tile = units::Tile(26);

static NUM_PARTS:                uint = 3;
static POSSIBLE_CONFIGS:         uint = 8;

pub struct Helicopter {
	pub x: units::Game,
	pub y: units::Game,
	pub map_x: units::Game, 
	pub map_y: units::Game,

	sprites:  HashMap<uint, Box<sprite::Updatable<units::Game>>>,
	parts:    HashMap<uint, uint>
}

pub trait Part {
	fn draw(&self, display: &mut graphics::Graphics);
	fn damage_rectangle(&self) -> Rectangle;
	fn part_type(&self) -> uint;
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
		
		let sprite_map = HashMap::<uint, Box<sprite::Updatable<_>>>::new();
		let part_map = HashMap::<uint, uint>::new();

		let mut new_helicopter = Helicopter { 
			x: x, y: y,
			map_x: x, map_y: y,

			sprites: sprite_map,
			parts: part_map
		};

		new_helicopter.load_sprites(display);

		new_helicopter
	}

	fn load_sprites(&mut self, 
	               display: &mut graphics::Graphics) {

		for i in range(0, POSSIBLE_CONFIGS) {
			match i {
				0 => {
					self.sprites.find_or_insert_with(0u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = SCRAPPED_X_OFFSET;
							let sprite_y = SCRAPPED_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				1 => {
					self.sprites.find_or_insert_with(1u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_1_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				2 => {
					self.sprites.find_or_insert_with(2u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_2_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				3 => {
					self.sprites.find_or_insert_with(3u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_3_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				4 => {
					self.sprites.find_or_insert_with(4u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_4_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				5 => {
					self.sprites.find_or_insert_with(5u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_5_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				6 => {
					self.sprites.find_or_insert_with(6u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_6_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							box sprite::Sprite::new(
									display, 
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									asset_path
								) as Box<sprite::Updatable<_>>
						}
					);
				},
				_ => {
					self.sprites.find_or_insert_with(7u,
						|_| -> Box<sprite::Updatable<_>> {
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = COMPLETE_X_OFFSET;
							let sprite_y = SCRAPPED_Y_OFFSET;

							box sprite::AnimatedSprite::new(
									display, asset_path,
									(sprite_x, sprite_y), 
									(units::Tile(4), units::Tile(3)),
									SPRITE_NUM_FRAMES, SPRITE_FPS
								).unwrap() as Box<sprite::Updatable<_>>
						}
					);
				}
			}
		}
	}

	pub fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.x + units::Tile(2), y: self.y + units::Tile(2),
			width: units::Game(10.0), height: units::Game(10.0),
		}
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		let sprite_ref = self.sprites.get_mut(&7u);
		sprite_ref.update(elapsed_time);
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		let mut config = 0u;
		if self.parts.len() == NUM_PARTS {
			config = 7u;
		} else if self.parts.contains_key(&1u) && self.parts.contains_key(&3u) {
			config = 1u;
		} else if self.parts.contains_key(&2u) && self.parts.contains_key(&3u) {
			config = 2u;
		} else if self.parts.contains_key(&1u) && self.parts.contains_key(&2u) {
			config = 6u;
		} else if self.parts.contains_key(&3u) {
			config = 3u;
		} else if self.parts.contains_key(&2u) {
			config = 4u;
		} else if self.parts.contains_key(&1u) {
			config = 5u;
		}
		self.sprites.get(&config).draw(display, (self.x, self.y));
	}

	pub fn add_part(&mut self, part_num: uint) {
		self.parts.insert(part_num, 1);
	}

	pub fn isBuilt(&self) -> bool {
		if self.parts.len() == NUM_PARTS {
			true
		} else {
			false
		}
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