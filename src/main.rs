extern crate rltk;
extern crate specs; 
extern crate specs_derive;  // Apparently this is not needed for the "new" versions of Rust?
extern crate array2d;
extern crate rand;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
//use std::time::{SystemTime, UNIX_EPOCH};
//use std::cmp::{max, min};
//use specs_derive::Component;
mod hexes;
mod data;

struct State {
    ecs: World,
    pub runstate : RunState
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Start, Running, Gameover }

pub fn init_gamedata() -> data::Gamedata {
    let gamedata = data::Gamedata {
        hexes:      hexes::generate_hex(1, 5),
        level:      1,
        score:      0,
        time:       2147483647,
    };
    gamedata
}

fn player_input(gs: &mut State, ctx: &mut Rltk) -> i32 {
    match ctx.key {
        None => {0}
        Some(key) => match key {
            VirtualKeyCode::Left => swap_hex(-1, &mut gs.ecs),
            VirtualKeyCode::Right => swap_hex(1, &mut gs.ecs),
            VirtualKeyCode::Up => rotate_col(-1, &mut gs.ecs),
            VirtualKeyCode::Down => rotate_col(1, &mut gs.ecs),
            VirtualKeyCode::Space => 2,
            _ => {0}
        },
    }
}

fn swap_hex(x: i32, ecs: &mut World) -> i32 {
    let mut positions = ecs.write_storage::<data::Position>();
    let mut gamedata = ecs.write_resource::<data::Gamedata>();
    for pos in (&mut positions).join() { // Without the parentheses the whole thing is fucked.
        println!("{:?}", pos);
        pos.x += x;
		if pos.x == 5 {
			pos.x = 1;
		}
		if pos.x == 0 {
			pos.x = 4;
		}
        gamedata.hexes.set(1, pos.x as usize, 11).ok();
    }
    1
}

fn rotate_col(y: i32, ecs: &mut World) -> i32 {
    let mut positions = ecs.write_storage::<data::Position>();
    let mut gamedata = ecs.write_resource::<data::Gamedata>();
    for pos in (&mut positions).join() { // Without the parentheses the whole thing is fucked.
        if y == -1 {
			let value = *gamedata.hexes.get(15, pos.x as usize).unwrap(); // Currently panics?
			gamedata.hexes.set(0, pos.x as usize, value).ok();
			for row in 0..hexes::ROWS {
				gamedata.hexes.set(row, pos.x as usize, 11).ok();
			}
		}
    }
    1
}

fn set_time(ecs: &mut World) {
    let mut gamedata = ecs.write_resource::<data::Gamedata>();
    gamedata.time -= 10;
}

fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(n)
        .collect()
}

fn flash_character() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0, 100) == 6 {
        (rng.gen_range(0, 41), rng.gen_range(0, 42))
    } else {
        (-1, -1)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        if player_input(self, ctx) == 2 && self.runstate == RunState::Start {
            self.runstate = RunState::Running;
        }
        if self.runstate == RunState::Running {
            set_time(&mut self.ecs);
            let gamedata = self.ecs.fetch::<data::Gamedata>();
            let positions = self.ecs.read_storage::<data::Position>();
                for row in 0..hexes::ROWS {
                    for col in 0..hexes::COLS {
                        for pos in positions.join() {
                            let hex = format!("{:x}", gamedata.hexes.get(row, col).unwrap());
                            if pos.y - 1 == row as i32 && pos.x - 1 == col as i32 {
                                ctx.print_color(2 + (col * 10), 2 + row, RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.1, 0., 0.), hex);
                            } else if pos.x - 1 == col as i32 {
                                ctx.print_color(2 + (col * 10), 2 + row, RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.1, 0., 0.), hex);
                            }
                            else {
                                ctx.print_color(2 + (col * 10), 2 + row, RGB::from_f32(0.8, 0.8, 0.8), RGB::from_f32(0.1, 0., 0.), hex);
                            }
                        }
                    }
                }
            ctx.print_color(2, 19, RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), format!("{:x}", gamedata.level));
            ctx.print_color(12, 19, RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), format!("{:x}", gamedata.score));
            ctx.print_color(22, 19, RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), format!("{:08x}", gamedata.time));
        } else if self.runstate == RunState::Start {
            ctx.print_color(2, 2, RGB::from_f32(1.0, 0.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), "Ethics-gradient v0.1");
        } else {
            ctx.print_color(2, 2, RGB::from_f32(1.0, 0.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), "Game over.");
        }
        let flashcoords = flash_character();
        ctx.print_color(flashcoords.0, flashcoords.1, RGB::from_f32(1.0, 0.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), random_string(1));
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::vga(42, 42)
        .with_font("vga8x16.png", 8, 16)
        .build()?;
    context.with_post_scanlines(true);
    let mut gs = State {
        ecs: World::new(),
        runstate : RunState::Start
    };
    gs.ecs.register::<data::Position>();
    gs.ecs.register::<data::Gamedata>();
    gs.ecs.insert(init_gamedata());
    gs.ecs
        .create_entity()
        .with(data::Position { x: 2, y: 2 })
        .build();
    rltk::main_loop(context, gs)
}