use std::rc::Rc;

use std::collections::hash_map::{HashMap, Entry};
use std::path::Path;
use std::string;

use game;
use game::units;
use game::units::{AsPixel};

use sdl2;
use sdl2::rect;
use sdl2::surface;
use sdl2::render;
use sdl2::video;
use sdl2::mouse;
use sdl2::pixels;

use sdl2_mixer;
use sdl2_ttf;

static MAX_HP: u32 = 3;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

/// Acts as a buffer to the underlying display
pub struct Graphics<'g> {
	screen:   Box<render::Renderer<'g>>,
	music:    sdl2_mixer::Music,
	pub sound_effects: Vec<sdl2_mixer::Chunk>,
	pub sprite_cache:  HashMap<string::String, render::Texture>,
}

impl<'g> Graphics<'g> {
	/// Prepare the display for rendering
	#[allow(unused_must_use)]
	pub fn new(context: &sdl2::Sdl) -> Graphics<'g> {
		let (units::Pixel(w), units::Pixel(h)) = 
			(game::game::SCREEN_WIDTH.to_pixel(), game::game::SCREEN_HEIGHT.to_pixel());

	    let video_subsystem = context.video().unwrap();

	    let window = video_subsystem.window("rust-sdl2 demo: Video", w as u32, h as u32)
	        .position_centered()
	        .opengl()
	        .build()
	        .unwrap();

	    let mut render_context = window.renderer().build().unwrap();

		// setup background music
		sdl2_mixer::open_audio(sdl2_mixer::DEFAULT_FREQUENCY, 0x8010u16, 2, 1024);
		sdl2_mixer::allocate_channels(2);
		sdl2_mixer::init(sdl2_mixer::INIT_MP3 | sdl2_mixer::INIT_FLAC | sdl2_mixer::INIT_MOD | sdl2_mixer::INIT_FLUIDSYNTH | sdl2_mixer::INIT_MODPLUG | sdl2_mixer::INIT_OGG);
		let music = sdl2_mixer::Music::from_file( &Path::new("assets/background2.wav") ).unwrap();

		// setup sound effects
		let mut sound_effect_vec: Vec<sdl2_mixer::Chunk> = Vec::new();
		// let bullet = sdl2_mixer::Chunk::from_file( &Path::new("assets/bullet.wav") ).unwrap();
		// sound_effect_vec.push(bullet);
		// let wipeout = sdl2_mixer::Chunk::from_file( &Path::new("assets/wipeout.wav") ).unwrap();
		// sound_effect_vec.push(wipeout);
		// let nuke = sdl2_mixer::Chunk::from_file( &Path::new("assets/nuke.wav") ).unwrap();
		// sound_effect_vec.push(nuke);
		// let powerup = sdl2_mixer::Chunk::from_file( &Path::new("assets/powerup.wav") ).unwrap();
		// sound_effect_vec.push(powerup);
		// let debuff = sdl2_mixer::Chunk::from_file( &Path::new("assets/debuff.wav") ).unwrap();
		// sound_effect_vec.push(debuff);
		// let trap = sdl2_mixer::Chunk::from_file( &Path::new("assets/trap.wav") ).unwrap();
		// sound_effect_vec.push(trap);
		// let hit = sdl2_mixer::Chunk::from_file( &Path::new("assets/hit.wav") ).unwrap();
		// sound_effect_vec.push(hit);
		// let goal = sdl2_mixer::Chunk::from_file( &Path::new("assets/goal.wav") ).unwrap();
		// sound_effect_vec.push(goal);

		let graphics: Graphics = 
			Graphics {
				screen:        Box::new(render_context),
				sprite_cache:  HashMap::<string::String, render::Texture>::new(),
				music:         music, 
				sound_effects: sound_effect_vec
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
				match self.screen.create_texture_from_surface(&sprite_surface) {
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
	pub fn play_music(&self) {
		self.music.play(-1);
	}
	pub fn pause_music(&self) {
		sdl2_mixer::Music::pause();
	}
	pub fn resume_music(&self) {
		sdl2_mixer::Music::resume();
	}
	#[allow(unused_must_use)]
	pub fn play_sound_effect(&self, index: u32) {
		let channel = sdl2_mixer::Channel::all();
		let chunk = self.sound_effects.get(index as usize).unwrap() as &sdl2_mixer::Chunk; 
		channel.play(chunk, 0);
	}

	#[allow(unused_must_use)]
	pub fn draw_text(&mut self, text: &str, dest_rect: rect::Rect) {
		let font = trying!(sdl2_ttf::Font::from_file(&Path::new("assets/font.ttf"), 128));
		// render a surface, and convert it to a texture bound to the renderer
	    let surface = trying!(font.render(text, sdl2_ttf::blended(pixels::Color::RGBA(255, 0, 0, 255))));
	    let texture = trying!(self.screen.create_texture_from_surface(&surface));
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
		self.load_image(String::from(heart_sprites), false);
		let full_source = match rect::Rect::new(0, 0, 22, 22) {
			Ok(rect) => { rect.unwrap() },
			Err(msg) => { panic!(msg) }
		};
		let empty_source = match rect::Rect::new(25, 0, 22, 22) {
			Ok(rect) => { rect.unwrap() },
			Err(msg) => { panic!(msg) }
		};
		for i in 0.. MAX_HP {
			let x = i * 25;
			let dest = match rect::Rect::new(x as i32, 0, 25, 25) {
				Ok(rect) => { rect.unwrap() },
				Err(msg) => { panic!(msg) }
			};
			if i < hp {
				self.blit_surface(&heart_sprites, &full_source, &dest);
			} else {
				self.blit_surface(&heart_sprites, &empty_source, &dest);
			}
		}
	}
}
