use std::rc::Rc;

use game;
use game::graphics;
use game::units;
use game::units::{AsPixel};

use sdl2::rect::Rect;
use sdl2::render::Texture;

use std::string;

static BACKGROUND_SIZE: units::Tile = units::Tile(4);

#[derive(Clone)]
pub struct FixedBackdrop {
	surface_id: String 
}

impl FixedBackdrop {
	pub fn new(path: string::String, graphics: &mut graphics::Graphics) -> FixedBackdrop {
		let asset = graphics.load_image(path.clone(), false);
		FixedBackdrop { surface_id: path }
	}

	/// Repeatedly paints the asset across the entire screen.
	/// Moving the destination rectangle `BACKGROUND_SIZE` pixels
	/// in either direction as it progresses.
	pub fn draw(&self, graphics: &mut graphics::Graphics) {
		let (mut x, mut y) = (0i32,0i32);
		let units::Pixel(tile_size) = BACKGROUND_SIZE.to_pixel();

		while units::Pixel(x) < game::LEVEL_WIDTH.to_pixel() {
			while units::Pixel(y) < game::LEVEL_HEIGHT.to_pixel() {
				let src  = match Rect::new(0, 0, tile_size as u32, tile_size as u32) {
					Ok(rect) => { rect },
					Err(msg) => { panic!(msg) }
				};
				let dest = match Rect::new(x, y, tile_size as u32, tile_size as u32) {
					Ok(rect) => { rect },
					Err(msg) => { panic!(msg) }
				};

				graphics.blit_surface(&self.surface_id, &src.unwrap(), &dest.unwrap());
				y+= tile_size;
			}

			x += tile_size;
			y = 0;
		}
	}
}
