use std::cmp;
use std::io::Timer;
use std::io::File;
use std::vec::Vec;
use std::rand::{task_rng, Rng};
use collections::string::String;

use sdl2::rect;
use sdl2::sdl;
use sdl2::event;
use sdl2::keycode;
use sdl2_ttf;

pub use game::units::{AsGame};

pub use game::units;
pub use game::enemies;
pub use game::powerups;
pub use game::traps;
pub use game::map;
pub use game::input;
pub use game::goal;
pub use game::player;
pub use game::graphics;
pub use game::collisions::Rectangle;

static TARGET_FRAMERATE: units::Fps  =  60;
static MAX_FRAME_TIME: units::Millis =  units::Millis(5 * (1000 / TARGET_FRAMERATE) as int);

pub static SCREEN_WIDTH:  units::Tile = units::Tile(20);
pub static SCREEN_HEIGHT: units::Tile = units::Tile(20);

pub static POSSIBLE_CHARACTER_TILES: uint = 18;
pub static POSSIBLE_GOAL_TILES:		 uint = 17;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
)

/// An instance of the `just-run` game with its own event loop.
pub struct Game {
	player:    player::Player,
	enemies:   Vec<Box<enemies::Zombie>>,
	powerups:  Vec<Box<powerups::Powerup>>,
	traps:     Vec<Box<traps::Trap>>,
	killed:    Vec<Box<enemies::Zombie>>,
	activated: Vec<Box<powerups::Powerup>>,
	tripped:   Vec<Box<traps::Trap>>,
	goal:      goal::Goal,
	map:       map::Map,

	display:        graphics::Graphics,
	controller:     input::Input,
	paused:         bool,
	updates:        int,
	level:          int, 
	highscore:      int,
	freeze_counter: int, 
	alt_control:    bool
}

impl Game {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new() -> Game {
		println!("initalizing sdl ...");
		
		// initialize all major subsystems
		// hide the mouse cursor in our drawing context
		sdl::init(sdl::InitEverything);
		sdl2_ttf::init();
		let mut display = graphics::Graphics::new();
		let controller  = input::Input::new();
		let mut rng = task_rng();
		let enemies_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let powerups_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let traps_vector: Vec<Box<traps::Trap>> = Vec::new();
		let killed_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let activated_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let tripped_vector: Vec<Box<traps::Trap>> = Vec::new();

		let mut game = Game {
			map: map::Map::create_test_map(&mut display),
			player: player::Player::new(
				&mut display,
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
			),

			enemies: enemies_vector,
			powerups: powerups_vector,
			traps: traps_vector,
			killed: killed_vector,
			activated: activated_vector,
			tripped: tripped_vector,

			goal: goal::Goal::new(
				&mut display, 
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(), 
				(units::Tile(rng.gen_range(1u, POSSIBLE_GOAL_TILES))).to_game()
			),

			display:        display,
			controller:     controller, 
			paused:         true,
			updates:        0,
			level:          0,
			highscore:      Game::get_highscore(),
			freeze_counter: 0, 
			alt_control:    false
		};
		game.spawn_zombie(rng.gen_range(1u, 5u), (units::Game(0.0), units::Game(0.0)));
		game.spawn_powerup(rng.gen_range(1u, 7u));
		game.spawn_trap(1);

		game
	}

	pub fn spawn_zombie(&mut self, kind: uint, location: (units::Game, units::Game)) {
		let mut rng = task_rng();
		match kind {
			1 => {
				let zombie = box enemies::SlowZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as Box<enemies::Zombie>);
			}
			2 => {
				let zombie = box enemies::CrazyZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as Box<enemies::Zombie>);
			}
			3 => {
				let zombie = box enemies::RandomZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as Box<enemies::Zombie>);
			}
			_ => {
				let zombie = match location {
					(units::Game(0.0), units::Game(0.0)) => box enemies::CloudZombie::new(
																&mut self.display, 
																(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
																(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
															),
					(x, y) => box enemies::CloudZombie::new(&mut self.display, x, y),
				};
				self.enemies.push(zombie as Box<enemies::Zombie>);
			}
		};
	}

	pub fn spawn_powerup(&mut self, kind: uint) {
		let mut rng = task_rng();
		// 20% chance of generating a powerup
		if rng.gen_range(1u, 11u) > 8 {
			match kind {
				1 => {
					let powerup = box powerups::CricketBat::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.powerups.push(powerup as Box<powerups::Powerup>);
				}
				2 => {
					let powerup = box powerups::KillZombie::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.powerups.push(powerup as Box<powerups::Powerup>);
				}
				3 => {
					let powerup = box powerups::WipeOut::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.powerups.push(powerup as Box<powerups::Powerup>);
				}
				4 => {
					let powerup = box powerups::Freeze::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.powerups.push(powerup as Box<powerups::Powerup>);
				}
				5 => {
					let powerup = box powerups::Teleport::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.powerups.push(powerup as Box<powerups::Powerup>);
				}
				_ => {
					let powerup = box powerups::Nuke::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.powerups.push(powerup as Box<powerups::Powerup>);
				}
			};
		}
	}

	pub fn spawn_trap(&mut self, kind: uint) {
		let mut rng = task_rng();
		// 30% chance of spawning a trap
		if rng.gen_range(1u, 11u) > 7 {
			match kind {
				_ => {
					let trap = box traps::BearTrap::new(
									&mut self.display, 
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
									(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
								);
					self.traps.push(trap as Box<traps::Trap>);
				}
			};
		}
	}

	pub fn start(&mut self) {
		self.display.play_music();
		self.draw_start_screen();
		self.event_loop();
		sdl::quit();
	}

	pub fn draw_start_screen(&mut self) {
		self.display.clear_buffer();
		self.map.draw_background(&self.display);
		self.display.switch_buffers();
		self.display.draw_text("JUST F&#%IN RUN!!!", rect!(45, 50, 550, 200));
		let score_string = String::from_str("CURRENT HIGHSCORE: ").append(self.highscore.to_str().as_slice());
		self.display.draw_text(score_string.as_slice(), rect!(120, 300, 400, 100));
		self.display.draw_text("PRESS ENTER AND START RUNNING...", rect!(160, 500, 300, 50));
	}

	pub fn draw_status_bar(&mut self) {
		let score_string = String::from_str("SCORE: ").append(self.level.to_str().as_slice());
		self.display.draw_text(score_string.as_slice(), rect!(500, 0, 100, 30));
	}

	pub fn draw_game_over_screen(&mut self) {
		self.display.draw_text("GAME OVER MAN!", rect!(45, 50, 550, 200));
		let score_string = String::from_str("YOUR SCORE: ").append(self.level.to_str().as_slice());
		self.display.draw_text(score_string.as_slice(), rect!(120, 300, 400, 100));
		if self.level > self.highscore {
			self.display.draw_text("NEW HIGHSCORE!!", rect!(120, 400, 400, 60));
		}
		self.display.draw_text("PRESS ENTER TO RUN SOME MORE...", rect!(160, 500, 300, 50));
	}

	pub fn restart(&mut self) {
		println!("Restarting game...");

		let mut rng = task_rng();
		let enemies_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let powerup_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let traps_vector: Vec<Box<traps::Trap>> = Vec::new();
		let killed_vector: Vec<Box<enemies::Zombie>> = Vec::new();
		let activated_vector: Vec<Box<powerups::Powerup>> = Vec::new();
		let tripped_vector: Vec<Box<traps::Trap>> = Vec::new();

		self.player = player::Player::new(
				&mut self.display,
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
			);

		self.enemies = enemies_vector;
		self.powerups = powerup_vector;
		self.traps = traps_vector;
		self.killed = killed_vector;
		self.activated = activated_vector;
		self.tripped = tripped_vector;
		self.spawn_zombie(rng.gen_range(1u, 5u), (units::Game(0.0), units::Game(0.0)));
		self.spawn_powerup(rng.gen_range(1u, 7u));
		self.spawn_trap(1);

		self.goal = goal::Goal::new(
				&mut self.display, 
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(), 
				(units::Tile(rng.gen_range(1u, POSSIBLE_GOAL_TILES))).to_game()
			);

		self.paused = true;
		self.updates = 0;
		self.level = 0;
		self.freeze_counter = 0;
	}

	/// Polls current input events & dispatches them to the engine.
	///
	/// Then renders a snapshot of the world-state and then waits
	/// until its next frame deadline.
	fn event_loop(&mut self) {
		// event loop control
		let frame_delay          = units::Millis(1000 / TARGET_FRAMERATE as int);
		let mut last_update_time = units::Millis(sdl::get_ticks() as int);
		
		let mut running = true;
		let mut timer   = Timer::new().unwrap();
		
		while running {
			let start_time_ms = units::Millis(sdl::get_ticks() as int);
			self.controller.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task
			match event::poll_event() {
				event::KeyDownEvent(_,_,key_cap,_,_) => {
					self.controller.key_down_event(key_cap);
				},
				event::KeyUpEvent(_,_,key_cap,_,_) => {
					self.controller.key_up_event(key_cap);
				},
				_ => {},
			}

			// Handle exit game
			if self.controller.was_key_released(keycode::EscapeKey) {
				running = false;
			}

			// Handle paused game
			if self.controller.was_key_released(keycode::ReturnKey) {
				if self.paused {
					self.paused = false;
					self.display.resume_music();
				} else {
					self.paused = true;
					self.display.pause_music();
				}
			}

			// Handle alternate control method
			if self.controller.was_key_released(keycode::LShiftKey) {
				self.alt_control = !self.alt_control;
			}

			// Handle player movement
			if self.controller.was_key_released(keycode::LeftKey) || self.controller.was_key_released(keycode::RightKey) {
				self.player.stop_moving_horizontally(self.alt_control);
			} 
			if self.controller.is_key_held(keycode::LeftKey) && self.controller.is_key_held(keycode::RightKey) {
				self.player.stop_moving_horizontally(self.alt_control);
			} else if self.controller.is_key_held(keycode::LeftKey) {
				self.player.start_moving_left();
			} else if self.controller.is_key_held(keycode::RightKey) {
				self.player.start_moving_right();
			} else {
				self.player.stop_moving_horizontally(self.alt_control);
			}

			if self.controller.was_key_released(keycode::UpKey) || self.controller.was_key_released(keycode::DownKey) {
				self.player.stop_moving_vertically(self.alt_control);
			} 
			if self.controller.is_key_held(keycode::UpKey) && self.controller.is_key_held(keycode::DownKey) {
				self.player.stop_moving_vertically(self.alt_control);
			} else if self.controller.is_key_held(keycode::UpKey) {
				self.player.start_moving_up();
			} else if self.controller.is_key_held(keycode::DownKey) {
				self.player.start_moving_down();
			}

			// inform actors of how much time has passed since last frame
			let current_time_ms = units::Millis(sdl::get_ticks() as int);
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
			}

			// throttle event-loop based on iteration time vs frame deadline
			let iter_time = units::Millis(sdl::get_ticks() as int) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time { 
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };
			
			self.updates = self.updates + 1;

			timer.sleep(next_frame_time);

			/* Print current FPS to stdout
			let units::Millis(start_time) = start_time_ms;
			let seconds_per_frame =  (sdl::get_ticks() as int - start_time) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
			
		}

	}

	/// Instructs our actors to draw their current state to the screen.
	fn draw(&mut self) {
		// background
		self.map.draw_background(&self.display);

		// foreground
		self.goal.draw(&mut self.display);
		for powerup in self.powerups.iter() { powerup.draw(&mut self.display); }
		for trap in self.traps.iter() { trap.draw(&mut self.display); }
		for enemy in self.enemies.iter() { enemy.draw(&mut self.display); }
		self.player.draw(&mut self.display);
		let mut kill_list: Vec<Box<enemies::Zombie>> = Vec::new();
		let mut active_list: Vec<Box<powerups::Powerup>> = Vec::new();
		let mut tripped_list: Vec<Box<traps::Trap>> = Vec::new();
		for _ in range(0, self.activated.len()) { 
			match self.activated.pop() {
				Some(activated) => {
					let mut mut_activated = activated;
					// draw 'bullet' for kill zombie 
					if mut_activated.get_type() == 2 {
						let killed_enemy = self.killed.get(self.killed.len() - 1);
						let (units::Game(player_x), units::Game(player_y)) = (self.player.character.center_x(), self.player.character.center_y());
						let (units::Game(enemy_x), units::Game(enemy_y)) = (killed_enemy.get_x(), killed_enemy.get_y()); 
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
		for _ in range(0, self.tripped.len()) { 
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
		for _ in range(0, self.killed.len()) { 
			match self.killed.pop() {
				Some(killed) => {
					let mut mut_killed = killed;
					mut_killed.draw(&mut self.display);
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
		self.map.draw(&self.display);
	}

	/// Passes the current time in milliseconds to our underlying actors.
	fn update(&mut self, elapsed_time: units::Millis) {
		self.map.update(elapsed_time);
		if self.freeze_counter == 0 {
			for i in range(0u, self.enemies.len()) { 
				let enemy = self.enemies.get_mut(i);
				enemy.set_acceleration(self.player.character.center_x(), self.player.character.center_y()); 
				enemy.update(elapsed_time, &self.map); 
			}
		}
		else {
			self.freeze_counter = self.freeze_counter - 1;
		}
		self.player.update(elapsed_time, &self.map);
		for i in range(0, self.killed.len()) { self.killed.get_mut(i).update(elapsed_time, &self.map) }
		for i in range(0, self.activated.len()) { self.activated.get_mut(i).update(elapsed_time, &self.map) }
		for i in range(0, self.tripped.len()) { self.tripped.get_mut(i).update(elapsed_time, &self.map) }
		for i in range(0, self.powerups.len()) {
			let powerup = self.powerups.get_mut(i);
		    // change debuff status every 10 updates
			if self.updates % 20 == 0 {
				powerup.toggle_debuff();
			}
		}
		self.goal.update(elapsed_time);

		let mut collidedWithZombie = false;
		for i in range(0, self.enemies.len()) { 
			if self.enemies.get(i).damage_rectangle().collides_with_player(&self.player.character.damage_rectangle()) {
				if self.player.has_bat() || self.player.is_teleporting() {
					match self.enemies.remove(i) {
						Some(enemy) => {
							self.display.play_sound_effect(6);
							let mut mut_enemy = enemy;
							mut_enemy.kill_zombie();
							self.killed.push(mut_enemy);
							self.player.take_bat();
						}, 
						None => {}
					};
				}
			 	else {
			 		collidedWithZombie = true;
			 	}
			 	break;
			}
		}
		let enteredGoal = self.goal.damage_rectangle().collides_with(&self.player.character.damage_rectangle());

		// Apply powerup
		let mut counter = 0;
		let mut hit_powerup = false;
		for powerup in self.powerups.mut_iter() { 
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
		for trap in self.traps.mut_iter() { 
			// check if player hit trap
			if trap.damage_rectangle().collides_with_player(&self.player.character.damage_rectangle()) {
				player_hit_trap = true;
				break;
			}
			// check if zombies hit trap
			for i in range(0, self.enemies.len()) {
				if self.enemies.get(i).damage_rectangle().collides_with(&trap.damage_rectangle()) {
					match self.enemies.remove(i) {
						Some(enemy) => {
							let mut mut_enemy = enemy;
							mut_enemy.kill_zombie();
							self.killed.push(mut_enemy);
							zombie_hit_trap = true;
							break;
						}, 
						None => {}
					};
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

		if enteredGoal {
			let mut rng = task_rng();
			self.display.play_sound_effect(7);
			self.goal = goal::Goal::new(
				&mut self.display, 
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(), 
				(units::Tile(rng.gen_range(1u, POSSIBLE_GOAL_TILES))).to_game()
			);
			self.spawn_zombie(rng.gen_range(1u, 5u), (units::Game(0.0), units::Game(0.0)));
			self.spawn_powerup(rng.gen_range(1u, 7u));
			self.spawn_trap(1);
			self.level = self.level + 1;
		}
		if collidedWithZombie || player_hit_trap {
			self.player.character.kill_character();
			self.draw();
			// draw game over screen store score and start a new game
			self.draw_game_over_screen();
			let level = self.level;
			self.store_highscore(level);
			self.restart();
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

	fn apply_powerup(&mut self, index: uint) {
		match self.powerups.remove(index) {
			Some(powerup) => { 
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
						self.display.play_sound_effect(0);
						let mut rng = task_rng(); 
						match self.enemies.remove( rng.gen_range(0u, length) ) {
							Some(killed) => {
								let mut mut_enemy = killed;
								self.activated.push(powerup);
								mut_enemy.kill_zombie();
								self.killed.push(mut_enemy);
							},
							None => {}
						};
					},
					// wipe out all zombies in given range
					3 => { 
						println!("WIPE OUT");
						self.display.play_sound_effect(1); 
						let mut new_enemies: Vec<Box<enemies::Zombie>> = Vec::new();
						for _ in range(0, self.enemies.len()) { 
							let enemy = self.enemies.pop();
							match enemy {
								Some(enemy) => {
									if self.player.character.distance( enemy.get_x(), enemy.get_y() ) > 200.0 {
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
						// teleport player to goal
						self.player.character.x = self.goal.x; 
						self.player.character.y = self.goal.y; 
						self.player.start_teleport_timer();
						let mut mut_powerup = powerup;
						mut_powerup.set_timer();
						self.activated.push(mut_powerup);
					},
					_ => { 
						if powerup.is_debuff() {
							println!("SUCKS TO BE YOU"); 
							self.display.play_sound_effect(4);
							let mut rng = task_rng();
							let mut new_enemies: Vec<Box<enemies::Zombie>> = Vec::new();
							for _ in range(0, self.enemies.len()) { 
								let enemy = self.enemies.pop();
								match enemy {
									Some(enemy) => {
										if rng.gen_range(1u, 11u) >= 3 {
											let crazy_zombie = box enemies::CrazyZombie::new(
												&mut self.display, 
												enemy.get_x(),
												enemy.get_y()
											);
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
							for _ in range(0, self.enemies.len()) {
								match self.enemies.pop() {
									Some(enemy) => {
										let mut mut_enemy = enemy;
										mut_enemy.kill_zombie();
										self.killed.push(mut_enemy);
									},
									None => {}
								};
							}
							let mut mut_powerup = powerup;
							mut_powerup.set_timer();
							self.activated.push(mut_powerup);
						} 
					}
				};
			},
			_  => { () }
	    };
	}

	fn activate_trap(&mut self, index: uint) {
		match self.traps.remove(index) {
			Some(trap) => { 
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
			},
			_  => { () }
	    };
	}

	fn get_highscore() -> int {
		match File::open(&Path::new("highscore.txt")).read_to_str() {
		    Ok(score) => { from_str(score.as_slice()).unwrap() }, // succeeded
		    Err(e) => { println!("failed to get highscore: {}", e); 0 }
		}
	}

	fn store_highscore(&mut self, new_score: int) {
		if new_score > self.highscore {
			match File::create(&Path::new("highscore.txt")).write_int(new_score) {
			    Ok(()) => { 
			    	self.highscore = new_score;
			    }, // succeeded
			    Err(e) => println!("failed to write highscore: {}", e)
			}
		}
	}
}
