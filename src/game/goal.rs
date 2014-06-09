use std::collections::hashmap::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;

use game::units;

static X_OFFSET: units::Tile = units::Tile(0);
static Y_OFFSET: units::Tile = units::Tile(0);

pub struct Goal {
	x: units::Game,
	y: units::Game,

	sprites:  HashMap<sprite::Facing, Box<sprite::Updatable<units::Game>>>,
}

impl Goal {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> Goal {
		
		let sprite_map = HashMap::<sprite::Facing, Box<sprite::Updatable<_>>>::new();

		let mut new_goal = Goal { 
			x: x, y: y,

			sprites: sprite_map,
		};

		new_goal.load_sprite(display);

		new_goal
	}

	fn load_sprite(&mut self, 
	               display: &mut graphics::Graphics) {

		self.sprites.find_or_insert_with(sprite::West,
			|_| -> Box<sprite::Updatable<_>> {
				let asset_path = "assets/base/Stage/PrtCent.bmp".to_string();
				let sprite_x = X_OFFSET;
				let sprite_y = Y_OFFSET;

				box sprite::Sprite::new(
						display, 
						(sprite_x, sprite_y), 
						(units::Tile(1), units::Tile(2)),
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
