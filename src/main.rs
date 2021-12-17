extern crate sdl2;
extern crate rand;
extern crate time;

pub mod game;

pub fn main() {
	println!("initalizing sdl ...");
	let sdl_context = sdl2::init().unwrap();
	let _audio = sdl_context.audio().unwrap();

	let mut story = ::game::Game::new(&sdl_context);
	story.start();
}
