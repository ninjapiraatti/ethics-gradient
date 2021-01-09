extern crate rltk;
extern crate specs; 
extern crate specs_derive;  // Apparently this is not needed for the "new" versions of Rust?
extern crate array2d;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
//use std::cmp::{max, min};
//use specs_derive::Component;
mod hexes;
mod data;

struct State {
    ecs: World
}

pub fn init_gamedata() -> data::Gamedata {
    let gamedata = data::Gamedata {
        hexes:  hexes::generate_hex(1),
        level:  1,
        score:  0,
        time:   2147483647,
    };
    gamedata
}

fn player_input(gs: &mut State, ctx: &mut Rltk)
{
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => rotate_col(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => rotate_col(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => rotate_col(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => rotate_col(0, 1, &mut gs.ecs),
            VirtualKeyCode::Space => finish_level(&mut gs.ecs),
            _ => {}
        },
    }
}

fn finish_level(ecs: &mut World) {
    println!("Finished level!");
}

fn rotate_col(x: i32, y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<data::Position>();
    let mut gamedata = ecs.write_resource::<data::Gamedata>();
    for pos in (&mut positions).join() { // Without the parentheses the whole thing is fucked.
        println!("{:?}", pos);
        if pos.x + x > 0 && pos.x + x < 5 {
            pos.x += x;
        }
        if pos.y + y > 1 && pos.y + y < 18 {
            pos.y += y;
        }
    }
    gamedata.level = 10;
}

fn set_time(ecs: &mut World) {
    let mut gamedata = ecs.write_resource::<data::Gamedata>();
    gamedata.time -= 1;
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
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
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::vga(42, 42)
        .with_font("vga8x16.png", 8, 16)
        .build()?;
    context.with_post_scanlines(true);
    let mut gs = State {
        ecs: World::new()
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