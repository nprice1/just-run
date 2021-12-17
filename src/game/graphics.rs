use std::collections::hash_map::{HashMap, Entry};
use std::path::Path;
use std::string;

use game;
use game::units;
use game::units::{AsPixel};

use sdl2;
use sdl2::rect;
use sdl2::surface;
use sdl2::render::{self, Canvas, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};
use sdl2::pixels;

static MAX_HP: u32 = 3;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

/// Acts as a buffer to the underlying display
pub struct Graphics {
	screen:   		   Box<Canvas<Window>>,
	texture_creator:   TextureCreator<WindowContext>,
 	ttf: 	   		   Sdl2TtfContext,
	pub sprite_cache:  HashMap<string::String, render::Texture>,
}

impl Graphics {
	/// Prepare the display for rendering
	#[allow(unused_must_use)]
	pub fn new(context: &sdl2::Sdl) -> Graphics {
		let (units::Pixel(w), units::Pixel(h)) = 
			(game::game::SCREEN_WIDTH.to_pixel(), game::game::SCREEN_HEIGHT.to_pixel());

	    let _video_subsystem = context.video().unwrap();

	    let window = _video_subsystem.window("Just Run", w as u32, h as u32)
	        .position_centered()
	        .opengl()
	        .build()
			.unwrap();

		let canvas = window
			.into_canvas()
			.accelerated()
			.build()
			.unwrap();

		let ttf_context = sdl2::ttf::init().unwrap();
		let texture_creator = canvas.texture_creator();

		let graphics: Graphics = 
			Graphics {
				screen:        	 Box::new(canvas),
				texture_creator: texture_creator,
				sprite_cache:    HashMap::<string::String, render::Texture>::new(),
				ttf:   	         ttf_context,
			};
		
		return graphics;
	}

	/// Loads a bitmap which resides at `file_path` and returns a handle
	/// This handle can safely be used in any of the graphics subsystem's rendering
	/// contexts.
	pub fn load_image(&mut self, 
	                  file_path: string::String, 
	                  transparent_black: bool) {
		
		// Retrieve a handle or generate a new one if it exists already.
		// Load sprite
		let sprite_path = Path::new(&file_path[..]);
		let sprite_window = surface::Surface::load_bmp(&sprite_path);

		// Store sprite
		let mut sprite_surface = match sprite_window {
			Ok(surface) => surface,
			Err(msg) => panic!("sprite could not be loaded to a surface: {}", msg),
		};

		// wrap surface in texture and store it
		if transparent_black {
			match sprite_surface.set_color_key(true, pixels::Color::RGB(0,0,0)) {
				Ok(_) => {},
				Err(msg) => panic!("Failed to key sprite: {}", msg),
			}
		}

		match self.sprite_cache.entry(file_path.clone()) {
			Entry::Vacant(entry) => {
				match self.texture_creator.create_texture_from_surface(&sprite_surface) {
					Ok(texture) => { entry.insert(texture); },
					Err(msg) => panic!("sprite could not be rendered: {}", msg)
				}
			},

			_ => {},
		};
	}

	pub fn remove_image(&mut self, file_path: string::String) {
		self.sprite_cache.remove(&file_path);
	}
	
	#[allow(unused_must_use)]
	pub fn blit_surface(&mut self,
	                    src_id: &str,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect) {
		
		let src = &mut self.sprite_cache.get_mut(src_id).unwrap();
		let _ = self.screen.copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&mut self) {
		self.screen.present();
	}

	#[allow(unused_must_use)]
	pub fn clear_buffer(&mut self) {
		self.screen.clear();
	}

	#[allow(unused_must_use)]
	pub fn draw_text(&mut self, text: &str, dest_rect: rect::Rect) {
		let font = self.ttf.load_font(&Path::new("assets/font.ttf"), 128).unwrap();
		// render a surface, and convert it to a texture bound to the renderer
	    let surface = trying!(font.render(text).blended(pixels::Color::RGBA(255, 0, 0, 255)));
		let texture_creator = self.screen.texture_creator();
	    let texture = trying!(texture_creator.create_texture_from_surface(&surface));
    	self.screen.copy(&texture, None, Some(dest_rect));
	}

	#[allow(unused_must_use)]
	pub fn draw_line(&mut self, source: (i32, i32), dest: (i32, i32)) {
		let (x1,y1) = source;
		let (x2,y2) = dest;
		let source_point = rect::Point::new(x1, y1);
		let dest_point = rect::Point::new(x2, y2);
		self.screen.set_draw_color(pixels::Color::RGB(255, 255, 255));
		self.screen.draw_line(source_point, dest_point);
	}

	#[allow(unused_must_use)]
	pub fn draw_health(&mut self, hp: u32) {
		let heart_sprites = "assets/base/heart.bmp"; 
		self.load_image(String::from(heart_sprites), true);
		let full_source = rect::Rect::new(0, 0, 18, 18);
		let empty_source = rect::Rect::new(21, 0, 18, 18);
		for i in 0.. MAX_HP {
			let x = i * 25;
			let dest = rect::Rect::new(x as i32, 0, 25, 25);
			if i < hp {
				self.blit_surface(&heart_sprites, &full_source, &dest);
			} else {
				self.blit_surface(&heart_sprites, &empty_source, &dest);
			}
		}
	}
}
