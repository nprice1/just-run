use std::rc::Rc;

use std::collections::hashmap::HashMap;
use collections::string;

use game;
use game::units;
use game::units::{AsPixel};

use sdl2::rect;
use sdl2::pixels;
use sdl2::surface;
use sdl2::render;
use sdl2::video;
use sdl2::mouse;

use sdl2_mixer::Music;
use sdl2_mixer::{init,allocate_channels,open_audio};
use sdl2_mixer::{InitMp3,InitFlac,InitMod,InitFluidSynth,InitModPlug,InitOgg,DEFAULT_FREQUENCY};

/// Acts as a buffer to the underlying display
pub struct Graphics {
	screen:   Box<render::Renderer<video::Window>>,
	music:    Music,
	pub sprite_cache:  HashMap<string::String, Rc<Box<render::Texture>>>,
}

impl Graphics {
	/// Prepare the display for rendering
	#[allow(unused_must_use)]
	pub fn new() -> Graphics {
		let (units::Pixel(w), units::Pixel(h)) = 
			(game::game::SCREEN_WIDTH.to_pixel(), game::game::SCREEN_HEIGHT.to_pixel());
		
		let current_mode = video::Window::new(
			"Just F&#%IN RUN",                       // title
			video::PosCentered, video::PosCentered,  // position (x,y)
			w as int, h as int,
			video::InputFocus
		);

		let window_context = match current_mode {
			Ok(ctx)  => ctx,
			Err(msg) => fail!(msg),
		};

		let render_context = render::Renderer::from_window(
			window_context,
			render::DriverAuto,
			render::Accelerated,
		);

		// setup audio
		open_audio(DEFAULT_FREQUENCY, 0x8010u16, 2, 1024);
		allocate_channels(0);
		init(InitMp3 | InitFlac | InitMod | InitFluidSynth | InitModPlug | InitOgg);
		let music = Music::from_file( &Path::new("assets/background.wav") ).unwrap();

		let graphics: Graphics = match render_context {
			Ok(renderer) => {
				Graphics{
					screen:        box renderer,
					sprite_cache:  HashMap::<string::String, Rc<Box<render::Texture>>>::new(),
					music:         music
				}
			},
			Err(msg) => {fail!(msg)},
		};
		
		mouse::show_cursor(false);
		return graphics;
	}

	/// Loads a bitmap which resides at `file_path` and returns a handle
	/// This handle can safely be used in any of the graphics subsystem's rendering
	/// contexts.
	pub fn load_image(&mut self, 
	                  file_path: string::String, 
	                  transparent_black: bool) -> Rc<Box<render::Texture>> {
		
		// Retrieve a handle or generate a new one if it exists already.
		let borrowed_display = &self.screen;
		let handle = self.sprite_cache.find_or_insert_with(file_path, |key| {
			// Load sprite
			let sprite_path = Path::new((*key).clone());
			let sprite_window = surface::Surface::from_bmp(&sprite_path);

			// Store sprite
			let sprite_surface = match sprite_window {
				Ok(surface) => surface,
				Err(msg) => fail!("sprite could not be loaded to a surface: {}", msg),
			};

			// wrap surface in texture and store it
			if transparent_black {
				match sprite_surface.set_color_key(true, pixels::RGB(0,0,0)) {
					Ok(_) => {},
					Err(msg) => fail!("Failed to key sprite: {}", msg),
				}
			}

			match borrowed_display.create_texture_from_surface(&sprite_surface) {
				Ok(texture) => Rc::new(box texture),
				Err(msg) => fail!("sprite could not be rendered: {}", msg)
			}
		});

		handle.clone()
	}

	pub fn remove_image(&mut self, file_path: string::String) {
		self.sprite_cache.remove(&file_path);
	}
	
	#[allow(unused_must_use)]
	pub fn blit_surface(&self,
	                    src: &render::Texture,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect) {
		
		self.screen.copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) {
		self.screen.present();
	}

	#[allow(unused_must_use)]
	pub fn clear_buffer(&self) {
		self.screen.clear();
	}

	#[allow(unused_must_use)]
	pub fn play_music(&self) {
		self.music.play(10000);
	}
	pub fn pause_music(&self) {
		Music::pause();
	}
	pub fn resume_music(&self) {
		Music::resume();
	}
}
