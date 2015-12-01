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

static NUM_PARTS:                u32 = 3;
static POSSIBLE_CONFIGS:         u32 = 8;

pub struct Helicopter {
	pub x: units::Game,
	pub y: units::Game,
	pub map_x: units::Game, 
	pub map_y: units::Game,

	sprites:  HashMap<u32, Box<sprite::Updatable<units::Game>>>,
	parts:    HashMap<u32, u32>
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
		
		let sprite_map = HashMap::<u32, Box<sprite::Updatable<_>>>::new();
		let part_map = HashMap::<u32, u32>::new();

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

		for i in 0.. POSSIBLE_CONFIGS {
			match i {
				0 => {
					self.sprites.insert(0u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = SCRAPPED_X_OFFSET;
							let sprite_y = SCRAPPED_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				1 => {
					self.sprites.insert(1u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_1_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				2 => {
					self.sprites.insert(2u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_2_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				3 => {
					self.sprites.insert(3u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_3_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				4 => {
					self.sprites.insert(4u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_4_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				5 => {
					self.sprites.insert(5u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_5_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				6 => {
					self.sprites.insert(6u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = PERM_6_OFFSET;
							let sprite_y = PERM_Y_OFFSET;

							Box::new( sprite::Sprite::new(
								display, 
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								asset_path
							) ) as Box<sprite::Updatable<_>>
						}
					);
				},
				_ => {
					self.sprites.insert(7u32,
						{
							let asset_path = "assets/base/heli.bmp".to_string();
							let sprite_x = COMPLETE_X_OFFSET;
							let sprite_y = SCRAPPED_Y_OFFSET;

							Box::new( sprite::AnimatedSprite::new(
								display, asset_path,
								(sprite_x, sprite_y), 
								(units::Tile(4), units::Tile(3)),
								SPRITE_NUM_FRAMES, SPRITE_FPS
							).unwrap() ) as Box<sprite::Updatable<_>>
						}
					);
				}
			}
		}
	}
}

impl Vehicle for Helicopter {

	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.x + units::Tile(2), y: self.y + units::Tile(1) + units::HalfTile(1),
			width: units::Game(32.0), height: units::Game(16.0),
		}
	}

	fn update(&mut self, elapsed_time: units::Millis) {
		let sprite_ref = self.sprites.get_mut(&7u32);
		sprite_ref.unwrap().update(elapsed_time);
	}

	fn update_for_cinematic(&mut self) {
		self.y = self.y - units::Game(1.0);
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let mut config = 0u32;
		if self.parts.len() == NUM_PARTS as usize {
			config = 7u32;
		} else if self.parts.contains_key(&1u32) && self.parts.contains_key(&3u32) {
			config = 1u32;
		} else if self.parts.contains_key(&2u32) && self.parts.contains_key(&3u32) {
			config = 2u32;
		} else if self.parts.contains_key(&1u32) && self.parts.contains_key(&2u32) {
			config = 6u32;
		} else if self.parts.contains_key(&3u32) {
			config = 3u32;
		} else if self.parts.contains_key(&2u32) {
			config = 4u32;
		} else if self.parts.contains_key(&1u32) {
			config = 5u32;
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
		1
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

		self.character.sprites.insert(movement, {
			let asset_path = "assets/base/heli.bmp".to_string();
			let motion_frame = PROP_OFFSET;

			let facing_frame = PARTS_Y_OFFSET;

			Box::new( sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(3), units::Tile(1)),
				asset_path
			) ) as Box<sprite::Updatable<_>>
		});
	}
}

impl Part for Prop {
    fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::Tile(1) + units::HalfTile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(32.0 * 2.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
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

		self.character.sprites.insert(movement, {
			let asset_path = "assets/base/heli.bmp".to_string();
			let motion_frame = WIND_OFFSET;

			let facing_frame = PARTS_Y_OFFSET;

			Box::new( sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(1), units::Tile(1)),
				asset_path
			) ) as Box<sprite::Updatable<_>>
		});
	}
}

impl Part for Windshield {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::HalfTile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(16.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
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

		self.character.sprites.insert(movement, {
			let asset_path = "assets/base/heli.bmp".to_string();
			let motion_frame = BAR_OFFSET;

			let facing_frame = PARTS_Y_OFFSET;

			Box::new( sprite::Sprite::new(
				display,
				(motion_frame, facing_frame),
				(units::Tile(2), units::Tile(1)),
				asset_path
			) ) as Box<sprite::Updatable<_>>
		});
	}
}

impl Part for Bar {
	fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.character.x + units::Tile(1), y: self.character.y + units::HalfTile(1),
			width: units::Game(32.0), height: units::Game(16.0),
		}
	}

	fn draw(&self, display: &mut graphics::Graphics) {
		let correction_x = self.character.map_x % common::SCREEN_CORRECTION;
		let correction_y = self.character.map_y % common::SCREEN_CORRECTION;
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