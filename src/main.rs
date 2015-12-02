#![crate_id="rust-story#0.0.1"]

extern crate sdl2;
extern crate sdl2_mixer;
extern crate sdl2_ttf;
extern crate rand;
extern crate time;

pub mod game;

pub fn main() {
	println!("initalizing sdl ...");
	let sdl_context = sdl2::init().unwrap();
	let _ttf_context = sdl2_ttf::init();

	let mut story = ::game::Game::new(&sdl_context);
	story.start();
}
