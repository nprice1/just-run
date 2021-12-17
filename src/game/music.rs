
use std::path::Path;

use sdl2;

use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

/// Acts as a buffer to the underlying display
pub struct Music<'g> {
	music:    		   sdl2::mixer::Music<'g>,
	pub sound_effects: Vec<sdl2::mixer::Chunk>,
}

impl<'g> Music<'g> {
	/// Prepare the display for rendering
	#[allow(unused_must_use)]
	pub fn new(context: &'g sdl2::Sdl) -> Music<'g> {
		let _audio = context.audio().unwrap();

		// setup background music
		sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024);
		let _mixer_context = 
			sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG );

		sdl2::mixer::allocate_channels(4);

		let music = sdl2::mixer::Music::from_file( &Path::new("assets/background.wav") ).unwrap();

		// setup sound effects
		let mut sound_effect_vec: Vec<sdl2::mixer::Chunk> = Vec::new();
		let bullet = sdl2::mixer::Chunk::from_file( &Path::new("assets/bullet.wav") ).unwrap();
		sound_effect_vec.push(bullet);
		let wipeout = sdl2::mixer::Chunk::from_file( &Path::new("assets/wipeout.wav") ).unwrap();
		sound_effect_vec.push(wipeout);
		let nuke = sdl2::mixer::Chunk::from_file( &Path::new("assets/nuke.wav") ).unwrap();
		sound_effect_vec.push(nuke);
		let powerup = sdl2::mixer::Chunk::from_file( &Path::new("assets/powerup.wav") ).unwrap();
		sound_effect_vec.push(powerup);
		let debuff = sdl2::mixer::Chunk::from_file( &Path::new("assets/debuff.wav") ).unwrap();
		sound_effect_vec.push(debuff);
		let trap = sdl2::mixer::Chunk::from_file( &Path::new("assets/trap.wav") ).unwrap();
		sound_effect_vec.push(trap);
		let hit = sdl2::mixer::Chunk::from_file( &Path::new("assets/hit.wav") ).unwrap();
		sound_effect_vec.push(hit);
		let goal = sdl2::mixer::Chunk::from_file( &Path::new("assets/goal.wav") ).unwrap();
		sound_effect_vec.push(goal);

		let music: Music = 
			Music {
				music:           music,
				sound_effects:   sound_effect_vec
			};
		
		return music;
	}

    #[allow(unused_must_use)]
	pub fn play_music(&self) {
		self.music.play(-1);
	}

	pub fn pause_music(&self) {
		sdl2::mixer::Music::pause();
	}

	pub fn resume_music(&self) {
		sdl2::mixer::Music::resume();
	}

	pub fn play_sound_effect(&self, index: u32) {
		let channel = sdl2::mixer::Channel::all();
		let chunk = self.sound_effects.get(index as usize).unwrap() as &sdl2::mixer::Chunk; 
		channel.play(chunk, 0).unwrap();
	}
}
