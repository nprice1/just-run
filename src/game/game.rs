use std;
use std::cmp;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use rand::Rng;
use std::string::String;

use sdl2;
use sdl2::rect;
use sdl2::timer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2_ttf;

use rand;
use time::{Duration, PreciseTime};

pub use game::units::{AsGame};

pub use game::units;
pub use game::enemies;
pub use game::powerups;
pub use game::traps;
pub use game::map;
pub use game::input;
pub use game::vehicle;
pub use game::car;
pub use game::heli;
pub use game::player;
pub use game::graphics;
pub use game::collisions::Rectangle;

const TARGET_FRAMERATE: units::Fps  =  60;
static MAX_FRAME_TIME: units::Millis =  units::Millis(5 * (1000 / TARGET_FRAMERATE) as i64);

pub static LEVEL_WIDTH:   units::Tile =  units::Tile(60);
pub static SCREEN_WIDTH: units::Tile =  units::Tile(20);
pub static LEVEL_HEIGHT: units::Tile =   units::Tile(60);
pub static SCREEN_HEIGHT: units::Tile = units::Tile(20);

pub static POSSIBLE_CHARACTER_TILES: u32 = 58;
pub static MAX_ENEMIES:              u32 = 30;
pub static MAX_POWERUPS:             u32 = 20;
// pub static MAX_TRAPS:                uint = 5;

pub static POSSIBLE_PART_RANGE: (u32, u32) = (20, 55);
pub static LEVEL_PARTS:                 u32 = 3;
pub static LEVEL_1_TIME:                 i32 = 5000;
pub static LEVEL_1_CINEMATIC_FRAMES:     i32 = 300;
pub static LEVEL_2_TIME:                 i32 = 2000;
pub static LEVEL_2_CINEMATIC_FRAMES:     i32 = 300;

pub static PLAYER_STARTING_X: units::Tile = units::Tile(1);
pub static PLAYER_STARTING_Y: units::Tile = units::Tile(4);

pub static VEHICLE_STARTING_X: units::Tile = units::Tile(1);
pub static VEHICLE_STARTING_Y: units::Tile = units::Tile(1);

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        match rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32) {
        	Ok(rect) => { rect.unwrap() },
        	Err(msg) => { panic!(msg) }
        }
    )
);

/// An instance of the `just-run` game with its own event loop.
pub struct Game<'engine> {
	player:     player::Player,
	enemies:    Vec<Box<enemies::Zombie>>,
	powerups:   Vec<Box<powerups::Powerup>>,
	traps:      Vec<Box<traps::Trap>>,
	killed:     Vec<Box<enemies::Zombie>>,
	activated:  Vec<Box<powerups::Powerup>>,
	tripped:    Vec<Box<traps::Trap>>,
	parts:      Vec<Box<vehicle::Part>>,
	coll_parts: Vec<Box<vehicle::Part>>,
	vehicle:    Box<vehicle::Vehicle>,
	map:        map::Map,

	display:        graphics::Graphics<'engine>,
	context:        &'engine sdl2::Sdl,
	controller:     input::Input,
	paused:         bool,
	updates:        i32,
	level:          i32, 
	highscore:      i32,
	score:          i32,
	timer:          i32,
	completed_lvl:  bool,
	freeze_counter: i32, 
	alt_control:    bool
}

impl<'e> Game<'e> {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new(context: &'e sdl2::Sdl) -> Game<'e> {
		// initialize all major subsystems
	    let _ttf_context = sdl2_ttf::init();
		// hide the mouse cursor in our drawing context
		let mut display = graphics::Graphics::new(context);
		let controller  = input::Input::new();
		let mut rng = rand::thread_rng();
		let enemies_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let powerups_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let traps_vector: Vec<Box<traps::Trap>> = Vec::new();
		let killed_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let activated_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let tripped_vector: Vec<Box<traps::Trap>> = Vec::new();
		let part_vector: Vec<Box<vehicle::Part>> = Vec::new();
		let coll_part_vector: Vec<Box<vehicle::Part>> = Vec::new();

		let mut game = Game {
			map: map::Map::load_map(&mut display, 1 as i32),
			player: player::Player::new(
				&mut display,
				PLAYER_STARTING_X.to_game(),
				PLAYER_STARTING_Y.to_game()
			),

			enemies: enemies_vector,
			powerups: powerups_vector,
			traps: traps_vector,
			killed: killed_vector,
			activated: activated_vector,
			tripped: tripped_vector,
			parts: part_vector,
			coll_parts: coll_part_vector,

			vehicle: Box::new( heli::Helicopter::new(
				&mut display, 
				VEHICLE_STARTING_X.to_game(), 
				VEHICLE_STARTING_Y.to_game()
			) ) as Box<vehicle::Vehicle>,

			display:        display,
			context:        context,
			controller:     controller, 
			paused:         true,
			updates:        0,
			level:          1,
			highscore:      Game::get_highscore(),
			score:          0,
			timer:          LEVEL_1_TIME,
			completed_lvl:  false,
			freeze_counter: 0, 
			alt_control:    false
		};
		let number_of_zombies = rng.gen_range(20u32, MAX_ENEMIES);
		for _ in 0.. number_of_zombies {
		  	game.spawn_zombie(rng.gen_range(1u32, 5u32), (units::Game(0.0), units::Game(0.0)));
		}
		let number_of_powerups = rng.gen_range(0u32, MAX_POWERUPS);
		for _ in 0.. number_of_powerups {
			game.spawn_powerup(rng.gen_range(1u32, 7u32));
		}
		// let number_of_traps = rng.gen_range(0u, MAX_TRAPS);
		// for _ in range(0, number_of_traps) {
		// 	game.spawn_trap(1);
		// }
		for i in 0.. LEVEL_PARTS {
			game.spawn_part(i);
		}

		game
	}

	pub fn spawn_zombie(&mut self, kind: u32, location: (units::Game, units::Game)) {
		let mut rng = rand::thread_rng();
		let zombie = match kind {
			1 => {
				Box::new( enemies::SlowZombie::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<enemies::Zombie>
			}
			2 => {
				Box::new( enemies::CrazyZombie::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<enemies::Zombie>
			}
			3 => {
				Box::new( enemies::RandomZombie::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<enemies::Zombie>
			}
			_ => {
				match location {
					(units::Game(0.0), units::Game(0.0)) => Box::new( enemies::CloudZombie::new(
																&mut self.display, 
																(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
																(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
															) ) as Box<enemies::Zombie>,
					(x, y) => Box::new( enemies::CloudZombie::new(&mut self.display, x, y) ) as Box<enemies::Zombie>,
				}
			}
		};
		let colliding_tiles = self.map.get_colliding_tiles(&zombie.damage_rectangle());
		let (old_x, old_y) = location;
		let new_location = (old_x + units::Game(1.0), old_y + units::Game(1.0));
		for tile in colliding_tiles.iter() {
			if tile.tile_type == map::TileType::Wall {
				self.spawn_zombie(kind, new_location);
				return;
			}
		}

		self.enemies.push(zombie);
	}

	pub fn spawn_powerup(&mut self, kind: u32) {
		let mut rng = rand::thread_rng();
		let powerup = match kind {
			1 => {
				Box::new( powerups::CricketBat::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<powerups::Powerup>
			}
			2 => {
				Box::new( powerups::KillZombie::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<powerups::Powerup>
			}
			3 => {
				Box::new( powerups::WipeOut::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<powerups::Powerup>
			}
			4 => {
				Box::new( powerups::Freeze::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<powerups::Powerup>
			}
			5 => {
				Box::new( powerups::Teleport::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<powerups::Powerup>
			} 
			_ => {
				Box::new( powerups::Nuke::new(
					&mut self.display, 
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u32, POSSIBLE_CHARACTER_TILES))).to_game()
				) ) as Box<powerups::Powerup>
			}
		};
		let colliding_tiles = self.map.get_colliding_tiles(&powerup.damage_rectangle());
		for tile in colliding_tiles.iter() {
			if tile.tile_type == map::TileType::Wall {
				self.spawn_powerup(kind);
				return;
			}
		}

		self.powerups.push(powerup);
	}

	// pub fn spawn_trap(&mut self, kind: uint) {
	// 	let mut rng = task_rng();
	// 	match kind {
	// 		_ => {
	// 			let trap = box traps::BearTrap::new(
	// 							&mut self.display, 
	// 							(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
	// 							(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
	// 						);
	// 			self.traps.push(trap as Box<traps::Trap>);
	// 		}
	// 	};
	// }

	pub fn spawn_part(&mut self, kind: u32) {
		let mut rng = rand::thread_rng();
		let (min, max) = POSSIBLE_PART_RANGE;
		let mut x = rng.gen_range(0, max);
		let mut y = rng.gen_range(0, max);
		if x < 20 {
			y = rng.gen_range(min, max);
		} else if y < 20 {
			x = rng.gen_range(min, max);
		}
		let part = match self.vehicle.get_type() {
			1 => {
				match kind {
					0 => {
						Box::new( heli::Prop::new(
							&mut self.display, 
							units::Tile(x).to_game(),
							units::Tile(y).to_game()
						) ) as Box<vehicle::Part>
					},
					1 => {
						Box::new( heli::Windshield::new(
							&mut self.display, 
							units::Tile(x).to_game(),
							units::Tile(y).to_game()
						) ) as Box<vehicle::Part>
					},
					_ => {
						Box::new( heli::Bar::new(
							&mut self.display, 
							units::Tile(x).to_game(),
							units::Tile(y).to_game()
						) ) as Box<vehicle::Part>
					}
				}
			},
			_ => {
				match kind {
					0 => {
						Box::new( car::Tire::new(
							&mut self.display, 
							units::Tile(x).to_game(),
							units::Tile(y).to_game()
						) ) as Box<vehicle::Part>
					},
					1 => {
						Box::new( car::Door::new(
							&mut self.display, 
							units::Tile(x).to_game(),
							units::Tile(y).to_game()
						) ) as Box<vehicle::Part>
					},
					_ => {
						Box::new( car::Engine::new(
							&mut self.display, 
							units::Tile(x).to_game(),
							units::Tile(y).to_game()
						) ) as Box<vehicle::Part>
					}
				}
			}
		};
		let colliding_tiles = self.map.get_colliding_tiles(&part.damage_rectangle());
		for tile in colliding_tiles.iter() {
			if tile.tile_type == map::TileType::Wall {
				self.spawn_part(kind);
				return;
			}
		}
		self.parts.push(part);
	}

	pub fn start(&mut self) {
		self.display.play_music();
		self.draw_start_screen();
		self.event_loop();
	}

	pub fn draw_start_screen(&mut self) {
		self.display.clear_buffer();
		self.map.draw_background(&mut self.display);
		self.display.switch_buffers();
		self.display.draw_text("JUST F&#%IN RUN!!!", rect!(45, 50, 550, 200));
		let score_string = String::from("CURRENT HIGHSCORE: ") + &self.highscore.to_string();
		self.display.draw_text(&score_string, rect!(120, 300, 400, 100));
		self.display.draw_text("PRESS ENTER AND START RUNNING...", rect!(160, 500, 300, 50));
		self.display.switch_buffers();
	}

	pub fn draw_status_bar(&mut self) {
		let score_string = String::from("TIMER: ") + &self.timer.to_string();
		self.display.draw_text(&score_string, rect!(500, 0, 100, 30));
		self.display.draw_health(self.player.get_health());
	}

	pub fn draw_game_over_screen(&mut self) {
		self.display.draw_text("GAME OVER MAN!", rect!(45, 100, 550, 200));
		let score_string = String::from("YOUR SCORE: ") + &self.score.to_string();
		self.display.draw_text(&score_string, rect!(120, 300, 400, 100));
		self.display.draw_text("PRESS ENTER TO RUN SOME MORE...", rect!(160, 500, 300, 50));
		self.display.switch_buffers();
	}

	pub fn draw_completion_screen(&mut self) {
		let level_string = String::from("YOU BEAT LEVEL ") + &self.level.to_string() + "!";
		self.display.draw_text(&level_string, rect!(45, 100, 550, 200));
		let score_string = String::from("YOUR SCORE: ") + &self.score.to_string();
		self.display.draw_text(&score_string, rect!(120, 300, 400, 100));
		if self.score > self.highscore {
			self.display.draw_text("NEW HIGHSCORE!!", rect!(120, 400, 400, 60));
			let score = self.score;
			self.store_highscore(score);
		}
		self.display.draw_text("PRESS ENTER TO RUN SOME MORE...", rect!(160, 500, 300, 50));
		self.display.switch_buffers();
	}

	pub fn set_score_and_timer(&mut self) {
		self.score = self.score + (self.level * 1000) + (self.player.get_health() as i32 * 1000) + self.timer;
		self.timer = self.timer + LEVEL_2_TIME;
	}

	pub fn restart(&mut self) {
		println!("Restarting game...");
		self.level = 0;
		self.new_level(true);
		self.paused = true;
		self.score = 0;
		self.timer = LEVEL_1_TIME;
	}

	pub fn new_level(&mut self, restart: bool) {
		println!("Starting new level...");
		self.level = self.level + 1;

		let mut rng = rand::thread_rng();
		let enemies_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let powerup_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let traps_vector: Vec<Box<traps::Trap>> = Vec::new();
		let killed_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let activated_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let tripped_vector: Vec<Box<traps::Trap>> = Vec::new();
		let coll_part_vector: Vec<Box<vehicle::Part>> = Vec::new();
		let part_vector: Vec<Box<vehicle::Part>> = Vec::new();

		self.player = player::Player::new(
				&mut self.display,
				PLAYER_STARTING_X.to_game(),
				PLAYER_STARTING_Y.to_game()
			);

		let vehicle_num = rng.gen_range(0, 2);
		self.vehicle = match vehicle_num {
			0 => {
				Box::new( heli::Helicopter::new(
					&mut self.display, 
					VEHICLE_STARTING_X.to_game(), 
					VEHICLE_STARTING_Y.to_game()
				) ) as Box<vehicle::Vehicle>
			}, 
			_ => {
				Box::new( car::Car::new(
					&mut self.display, 
					VEHICLE_STARTING_X.to_game(), 
					VEHICLE_STARTING_Y.to_game()
				) ) as Box<vehicle::Vehicle>
			}
		};

		self.enemies = enemies_vector;
		self.powerups = powerup_vector;
		self.traps = traps_vector;
		self.killed = killed_vector;
		self.activated = activated_vector;
		self.tripped = tripped_vector;
		self.coll_parts = coll_part_vector;
		self.parts = part_vector;
		let number_of_zombies = rng.gen_range(20u32, MAX_ENEMIES);
		for _ in 0.. number_of_zombies {
		  	self.spawn_zombie(rng.gen_range(1u32, 5u32), (units::Game(0.0), units::Game(0.0)));
		}
		let number_of_powerups = rng.gen_range(0u32, MAX_POWERUPS);
		for _ in 0.. number_of_powerups {
			self.spawn_powerup(rng.gen_range(1u32, 7u32));
		}
		// let number_of_traps = rng.gen_range(0u, MAX_TRAPS);
		// for _ in range(0, number_of_traps) {
		// 	self.spawn_trap(1);
		// }

		for i in 0.. LEVEL_PARTS {
			self.spawn_part(i);
		}

		self.paused = false;
		self.updates = 0;
		self.freeze_counter = 0;

		// create new level map
		let new_map = map::Map::load_map(&mut self.display, self.level);
		self.map = new_map;
	}

	/// Polls current input events & dispatches them to the engine.
	///
	/// Then renders a snapshot of the world-state and then waits
	/// until its next frame deadline.
	fn event_loop(&mut self) {
		// event loop control
		let frame_delay = units::Millis(1000 / TARGET_FRAMERATE as i64);
		let start_time = PreciseTime::now();
		let mut last_update_time = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds());
		
		let mut running = true;

		let mut event_pump =  match self.context.event_pump() {
			Ok(pump) => { pump },
			Err(msg) => { panic!(msg) }
		};

		while running && !self.completed_lvl {
			let start_time_ms = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds());
			self.controller.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task
			for event in event_pump.poll_iter() {
				match event {
					Event::KeyDown { keycode, .. } => {
						self.controller.key_down_event(keycode.unwrap());
					},
					Event::KeyUp { keycode, .. } => {
						self.controller.key_up_event(keycode.unwrap());
					},
					_ => {},
				}
			}

			// Handle exit game
			if self.controller.was_key_released(Keycode::Escape) {
				running = false;
			}

			// Handle paused game
			if self.controller.was_key_released(Keycode::Return) {
				if self.paused {
					self.paused = false;
					self.display.resume_music();
				} else {
					self.paused = true;
					self.display.pause_music();
				}
			}

			// Handle alternate control method
			if self.controller.was_key_released(Keycode::LShift) {
				self.alt_control = !self.alt_control;
			}

			// Handle player movement
			if self.controller.was_key_released(Keycode::Left) || self.controller.was_key_released(Keycode::Right) {
				self.player.stop_moving_horizontally(self.alt_control);
			} 
			if self.controller.is_key_held(Keycode::Left) && self.controller.is_key_held(Keycode::Right) {
				self.player.stop_moving_horizontally(self.alt_control);
			} else if self.controller.is_key_held(Keycode::Left) {
				self.player.start_moving_left();
			} else if self.controller.is_key_held(Keycode::Right) {
				self.player.start_moving_right();
			} else {
				self.player.stop_moving_horizontally(self.alt_control);
			}

			if self.controller.was_key_released(Keycode::Up) || self.controller.was_key_released(Keycode::Down) {
				self.player.stop_moving_vertically(self.alt_control);
			} 
			if self.controller.is_key_held(Keycode::Up) && self.controller.is_key_held(Keycode::Down) {
				self.player.stop_moving_vertically(self.alt_control);
			} else if self.controller.is_key_held(Keycode::Up) {
				self.player.start_moving_up();
			} else if self.controller.is_key_held(Keycode::Down) {
				self.player.start_moving_down();
			}

			// inform actors of how much time has passed since last frame
			let current_time_ms = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds());
			let elapsed_time    = current_time_ms - last_update_time;
		
			// only update if not in paused state
			if !self.paused {
				self.update(cmp::min(elapsed_time, MAX_FRAME_TIME));
				last_update_time = current_time_ms;

				// draw if game has been started
				if self.updates != 0 {
					self.display.clear_buffer(); // clear back-buffer
					self.draw();
					self.draw_status_bar();
					self.display.switch_buffers();
				}

				// decrement timer
				self.timer = self.timer - 1;	
			}

			// throttle event-loop based on iteration time vs frame deadline
			let iter_time = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds()) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time { 
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };
			
			self.updates = self.updates + 1;

			std::thread::sleep_ms(next_frame_time as u32);

			if (self.completed_lvl && running) {
				let mut cinematic_counter = LEVEL_1_CINEMATIC_FRAMES;
				while self.completed_lvl && running {
					let start_time_ms = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds());
					// inform actors of how much time has passed since last frame
					let current_time_ms = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds());
					let elapsed_time    = current_time_ms - last_update_time;
					let mut show_completion_screen = true;

					self.controller.begin_new_frame();

					// drain event queue once per frame
					// ideally should do in separate task
					for event in event_pump.poll_iter() {
						match event {
							Event::KeyDown { keycode, .. } => {
								self.controller.key_down_event(keycode.unwrap());
							},
							Event::KeyUp { keycode, .. } => {
								self.controller.key_up_event(keycode.unwrap());
							},
							_ => {},
						}
					}

					// Handle exit game
					if self.controller.was_key_released(Keycode::Escape) {
						self.completed_lvl = false;
					}

					// Handle paused game
					if self.controller.was_key_released(Keycode::Return) {
						if cinematic_counter < 0 {
							self.completed_lvl = false;
							self.new_level(false);
							break;
						} 
					}
				
					// only update if not in paused state
					self.update_cinematic(cmp::min(elapsed_time, MAX_FRAME_TIME));
					last_update_time = current_time_ms;

					if cinematic_counter > 0 {
						self.display.clear_buffer(); // clear back-buffer
						self.draw_cinematic(cinematic_counter);
						self.draw_zombies();
						self.display.switch_buffers();
					} else if show_completion_screen {
						self.draw_completion_screen();
					}

					// throttle event-loop based on iteration time vs frame deadline
					let iter_time = units::Millis(start_time.to(PreciseTime::now()).num_milliseconds()) - start_time_ms;
					let next_frame_time: u64 = if frame_delay > iter_time { 
						let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
						(fd - it) as u64
					} else { 0 as u64 };
					
					std::thread::sleep_ms(next_frame_time as u32);

					cinematic_counter = cinematic_counter - 1;
				}
			}

			/* Print current FPS to stdout
			let units::Millis(start_time) = start_time_ms;
			let seconds_per_frame =  (sdl::get_ticks() as int - start_time) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
			
		}

	}

	// Instructs our actors to draw their current state to the screen.
	fn draw(&mut self) {
		// background
		self.map.draw_background(&mut self.display);

		// foreground
		if self.map.on_screen(self.vehicle.get_map_x(), self.vehicle.get_map_y()) {
			self.vehicle.draw(&mut self.display);
		}
		for part in self.parts.iter() {
			if self.map.on_screen(part.get_map_x(), part.get_map_y()) { 
				part.draw(&mut self.display); 
			} 
		}
		for powerup in self.powerups.iter() { 
			if self.map.on_screen(powerup.get_map_x(), powerup.get_map_y()) { 
				powerup.draw(&mut self.display); 
			} 
		}
		for trap in self.traps.iter() { 
			if self.map.on_screen(trap.get_map_x(), trap.get_map_y()) { 
				trap.draw(&mut self.display); 
			} 
		}
		self.draw_zombies();
		self.player.draw(&mut self.display);
		let mut kill_list: Vec<Box<enemies::Zombie>> = Vec::new();
		let mut active_list: Vec<Box<powerups::Powerup>> = Vec::new();
		let mut tripped_list: Vec<Box<traps::Trap>> = Vec::new();
		for _ in 0.. self.activated.len() { 
			match self.activated.pop() {
				Some(activated) => {
					let mut mut_activated = activated;
					// draw 'bullet' for kill zombie 
					if mut_activated.get_type() == 2 {
						let killed_enemy = self.killed.get(self.killed.len() - 1).unwrap();
						let (units::Game(player_x), units::Game(player_y)) = (self.player.character.center_x(), self.player.character.center_y());
						let (units::Game(enemy_x), units::Game(enemy_y)) = (killed_enemy.get_map_x(), killed_enemy.get_map_y());
						self.display.draw_line( (player_x as i32, player_y as i32), (enemy_x as i32, enemy_y as i32) );
					} else {
						mut_activated.draw(&mut self.display);
					}
					if !mut_activated.is_finished() {
						active_list.push(mut_activated);
					}
				},
				None => {}
			}; 
		}
		for _ in 0.. self.tripped.len() { 
			match self.tripped.pop() {
				Some(tripped) => {
					let mut mut_tripped = tripped;
					mut_tripped.draw(&mut self.display);
					if !mut_tripped.is_finished() {
						tripped_list.push(mut_tripped);
					}
				},
				None => {}
			}; 
		}
		for _ in 0.. self.killed.len() { 
			match self.killed.pop() {
				Some(killed) => {
					let mut mut_killed = killed;
					if self.map.on_screen(mut_killed.get_map_x(), mut_killed.get_map_y()) {
						mut_killed.draw(&mut self.display);
					}
					if !mut_killed.is_killed() {
						kill_list.push(mut_killed);
					}
				},
				None => {}
			}; 
		}
		self.killed = kill_list;
		self.activated = active_list;
		self.tripped = tripped_list;
		self.map.draw(&mut self.display);
	}

	fn draw_zombies(&mut self) {
		for enemy in self.enemies.iter() { 
			if self.map.on_screen(enemy.get_map_x(), enemy.get_map_y()) {
			 	enemy.draw(&mut self.display); 
			} 
		}
	}

	fn draw_cinematic(&mut self, counter: i32) {
		// background
		self.map.draw_background(&mut self.display);
		self.map.draw(&mut self.display);
		self.vehicle.update_for_cinematic();
		self.vehicle.draw(&mut self.display);
	}

	/// Passes the current time in milliseconds to our underlying actors.
	fn update(&mut self, elapsed_time: units::Millis) {
		self.map.update(elapsed_time);
		if self.freeze_counter == 0 {
			for i in 0u32.. self.enemies.len() as u32 { 
				let enemy = self.enemies.get_mut(i as usize).unwrap();
				let (player_x, player_y) = self.player.get_follow_coords();
				enemy.set_acceleration(player_x, player_y); 
				enemy.update(elapsed_time, &self.map); 
			}
		} else {
			self.freeze_counter = self.freeze_counter - 1;
		}
		self.player.update(elapsed_time, &self.map);
		self.map.set_page(self.player.character.map_center_x(), self.player.character.map_center_y());
		for i in 0.. self.killed.len() { self.killed.get_mut(i).unwrap().update(elapsed_time, &self.map) }
		for i in 0.. self.activated.len() { self.activated.get_mut(i).unwrap().update(elapsed_time, &self.map) }
		for i in 0.. self.tripped.len() { self.tripped.get_mut(i).unwrap().update(elapsed_time, &self.map) }
		for i in 0.. self.powerups.len() {
			let powerup = self.powerups.get_mut(i);
		    // change debuff status every 10 updates
			if self.updates % 20 == 0 {
				powerup.unwrap().toggle_debuff();
			}
		}
		self.vehicle.update(elapsed_time);

		let mut collidedWithZombie = false;
		if !self.player.is_immune() {
			for i in 0.. self.enemies.len() { 
				if self.enemies.get(i).unwrap().damage_rectangle().collides_with_player(&self.player.character.damage_rectangle()) {
					if self.player.has_bat() || self.player.is_teleporting() {
						let enemy = self.enemies.remove(i);
						self.display.play_sound_effect(6);
						let mut mut_enemy = enemy;
						mut_enemy.kill_zombie();
						self.killed.push(mut_enemy);
						self.player.take_bat();
						self.timer = self.timer + 100;
					}
				 	else {
				 		collidedWithZombie = true;
				 	}
				 	break;
				}
			}
		}

		// Collect part
		if self.coll_parts.len() == 0 {
			for i in 0.. self.parts.len() { 
				if self.parts.get(i).unwrap().damage_rectangle().collides_with(&self.player.character.damage_rectangle()) {
					let part = self.parts.remove(i);
					self.display.play_sound_effect(7);
					self.coll_parts.push(part);
					break;
				}
			}
		}

		// Apply parts to vehicle
		if self.vehicle.damage_rectangle().collides_with(&self.player.character.damage_rectangle()) {
			match self.coll_parts.pop() {
				Some(part) => { self.vehicle.add_part(part.part_type()); },
				None       => {}
			};
		}

		// Apply powerup
		let mut counter = 0;
		let mut hit_powerup = false;
		for powerup in self.powerups.iter_mut() { 
			if powerup.damage_rectangle().collides_with(&self.player.character.damage_rectangle()) {
				hit_powerup = true;
			 	break;
			}
			counter = counter + 1;
		}
		if hit_powerup {
			self.apply_powerup(counter);
		}

		// Activate traps
		let mut counter = 0;
		let mut player_hit_trap = false;
		let mut zombie_hit_trap = false;
		for trap in self.traps.iter_mut() { 
			// check if player hit trap
			if trap.damage_rectangle().collides_with_player(&self.player.character.damage_rectangle()) {
				player_hit_trap = true;
				break;
			}
			// check if zombies hit trap
			for i in 0.. self.enemies.len() {
				if self.enemies.get(i).unwrap().damage_rectangle().collides_with(&trap.damage_rectangle()) {
					let enemy = self.enemies.remove(i as usize);
					let mut mut_enemy = enemy;
					mut_enemy.kill_zombie();
					self.killed.push(mut_enemy);
					zombie_hit_trap = true;
					break;
				}
			}
			if zombie_hit_trap {
				break;
			}
			counter = counter + 1;
		}
		if player_hit_trap || zombie_hit_trap {
			self.activate_trap(counter);
		}

		if self.vehicle.is_built() {
			self.set_score_and_timer();
			self.completed_lvl = true;
		}

		// ran out of time
		if self.timer == 0 {
			self.draw_game_over_screen();
			self.restart();
		}

		if collidedWithZombie || player_hit_trap {
			self.display.play_sound_effect(6);
			match self.player.get_health() {
				health if health > 1 => {
					self.player.hit_player();
					self.player.start_immunity();
				},
				_ => {
					self.player.character.kill_character();
					self.draw();
					// draw game over screen store score and start a new game
					self.draw_game_over_screen();
					self.restart();
				}
			}
		}

		// populate cloud zombies
		if self.updates != 0 {
			let mut new_zombies = false;
			let mut zombie_location: (units::Game, units::Game) = (units::Game(0.0), units::Game(0.0));
			for enemy in self.enemies.iter() { 
				if enemy.zombie_type() == 4 && self.updates % 300 == 0 {
				  new_zombies = true;
				  zombie_location = enemy.get_target();
				  break;
				}
			}
			if new_zombies {
				self.spawn_zombie(4, zombie_location);
			}
		}
	}

	fn update_cinematic(&mut self, elapsed_time: units::Millis) {
		self.map.update(elapsed_time);
		for i in 0u32.. self.enemies.len() as u32 { 
			let enemy = self.enemies.get_mut(i as usize).unwrap();
			let (player_x, player_y) = self.player.get_follow_coords();
			enemy.set_acceleration(player_x, player_y); 
			enemy.update(elapsed_time, &self.map); 
		}
		self.vehicle.update(elapsed_time);
	}

	fn apply_powerup(&mut self, index: u32) {
		let powerup = self.powerups.remove(index as usize);
		let kind = powerup.get_type();
		let length = self.enemies.len();
		match kind {
			// kill next zombie you touch without dying
			1 => { 
				println!("CRICKET BAT"); 
				self.display.play_sound_effect(3); 
				self.player.give_bat(); 
			},
			// kill random zombie
			2 => { 
				println!("KILL ZOMBIE"); 
				if self.enemies.len() > 0 {
					self.display.play_sound_effect(0);
					let mut rng = rand::thread_rng(); 
					let killed = self.enemies.remove( rng.gen_range(0u32, length as u32) as usize );
					let mut mut_enemy = killed;
					self.activated.push(powerup);
					mut_enemy.kill_zombie();
					self.killed.push(mut_enemy);
				}
				self.timer = self.timer + 100;
			},
			// wipe out all zombies in given range
			3 => { 
				println!("WIPE OUT");
				self.display.play_sound_effect(1); 
				let mut new_enemies: Vec<Box<enemies::Zombie>> = Vec::new();
				for _ in 0.. self.enemies.len() { 
					let enemy = self.enemies.pop();
					match enemy {
						Some(enemy) => {
							if self.player.character.distance( enemy.get_map_x(), enemy.get_map_y() ) > 200.0 {
						    	new_enemies.push(enemy);
						    } else {
						    	let mut mut_enemy = enemy;
						    	mut_enemy.kill_zombie();
						    	self.killed.push(mut_enemy);
						    }
						},
						None => {}
					}
				}
				self.timer = self.timer + (self.killed.len() as i32 * 500);
				self.enemies = new_enemies;
				let mut mut_powerup = powerup;
				mut_powerup.set_timer();
				self.activated.push(mut_powerup);
			},
			// freeze all zombies
			4 => { 
				println!("FREEZE"); 
				self.display.play_sound_effect(3);
				self.freeze_counter = 300; 
			},
			5 => { 
				println!("TELEPORT"); 
				self.display.play_sound_effect(3); 
				// teleport player to helicopter
				self.player.character.map_x = self.vehicle.get_map_x(); 
				self.player.character.map_y = self.vehicle.get_map_y() + units::Tile(2).to_game(); 
				self.player.start_teleport_timer();
				let mut mut_powerup = powerup;
				mut_powerup.set_timer();
				self.activated.push(mut_powerup);
			},
			_ => { 
				if powerup.is_debuff() {
					println!("SUCKS TO BE YOU"); 
					self.display.play_sound_effect(4);
					let mut rng = rand::thread_rng();
					let mut new_enemies: Vec<Box<enemies::Zombie>> = Vec::new();
					for _ in 0.. self.enemies.len() { 
						let enemy = self.enemies.pop();
						match enemy {
							Some(enemy) => {
								if rng.gen_range(1u32, 11u32) >= 3 {
									let crazy_zombie = Box::new( enemies::CrazyZombie::new(
										&mut self.display, 
										enemy.get_map_x(),
										enemy.get_map_y()
									) );
							    	new_enemies.push(crazy_zombie);
							    } else {
							    	new_enemies.push(enemy);
							    }
							},
							None => {}
						}
					}
					self.enemies = new_enemies;
				} else {
					println!("NUKE"); 
					self.display.play_sound_effect(2);
					let mut new_enemies: Vec<Box<enemies::Zombie>> = Vec::new();
					for _ in 0.. self.enemies.len() {
						match self.enemies.pop() {
							Some(enemy) => {
								if self.map.on_screen(enemy.get_map_x(), enemy.get_map_y()) {
									let mut mut_enemy = enemy;
									mut_enemy.kill_zombie();
									self.killed.push(mut_enemy);
								} else {
									new_enemies.push(enemy);
								}
							},
							None => {}
						};
					}
					let mut mut_powerup = powerup;
					mut_powerup.set_timer();
					self.activated.push(mut_powerup);
					self.timer = self.timer + (self.killed.len() as i32 * 100);
					self.enemies = new_enemies;
				} 
			}
		};
	}

	fn activate_trap(&mut self, index: u32) {
		let trap = self.traps.remove(index as usize);
		let kind = trap.get_type();
		match kind {
			// Activate bear trap
			_ => { 
				println!("BEAR TRAP");
				self.display.play_sound_effect(5);
				let mut mut_trap = trap;
				mut_trap.set_timer();
				self.tripped.push(mut_trap);
			}
		};
	}

	fn get_highscore() -> i32 {
		match File::open(&Path::new("highscore.txt")) {
		    Ok(file) => { 
		    	let mut score = String::new();
		    	let mut f = file;
		    	f.read_to_string(&mut score);
		    	match score.parse::<i32>() {
		    		Ok(s) => { s },
		    		Err(msg) => { panic!(msg) }
		    	}
		    }, // succeeded
		    Err(e) => { println!("failed to get highscore: {}", e); 0 }
		}
	}

	fn store_highscore(&mut self, new_score: i32) {
		if new_score > self.highscore {
			match File::create(&Path::new("highscore.txt")) {
			    Ok(file) => { 
			    	let mut f = file;
			    	f.write_all(new_score.to_string().as_bytes());
			    	self.highscore = new_score;
			    }, // succeeded
			    Err(e) => println!("failed to write highscore: {}", e)
			}
		}
	}
}
