extern crate rltk;
extern crate specs; 
extern crate specs_derive;  // Apparently this is not needed for the "new" versions of Rust?
extern crate array2d;
use array2d::Array2D;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
mod hexes;
mod data;

struct State {
    ecs: World
}

#[derive(Component, Debug)]
struct Position {
    x: i32,
    y: i32,
}

pub fn init_gamedata() -> data::Gamedata {
    let mut gamedata = data::Gamedata {
        hexes:  hexes::generate_hex(1),
        level:  1,
        score:  0,
        posx:   0,
        posy:   0
    };
    gamedata
}

fn player_input(gs: &mut State, ctx: &mut Rltk)
{
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => println!("Left"),
            VirtualKeyCode::Right => println!("Right"),
            VirtualKeyCode::Up => rotate_col(0, 2, &mut gs.ecs),
            VirtualKeyCode::Down => println!("Down"),
            _ => {}
        },
    }
}

fn rotate_col(dir: usize, col: usize, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    //println!("{:?}", positions);
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        let mut gamedata = self.ecs.fetch::<data::Gamedata>();
        //player_input(self, ctx);
        let mut hex = String::new();
        for row in 0..hexes::ROWS {
            for col in 0..hexes::COLS {
                hex = format!("{:x}", gamedata.hexes.get(row, col).unwrap());
                ctx.print_color(2 + (col * 10), 2 + row, RGB::from_f32(0.9, 0.9, 0.9), RGB::from_f32(0.1, 0., 0.), hex);
            }
        }
        let mut positions = self.ecs.write_storage::<Position>();
        for pos in positions.join() {
            println!("{:?}", pos);
        }
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
    gs.ecs.register::<Position>();
    gs.ecs.register::<data::Gamedata>();
    gs.ecs.insert(init_gamedata());
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .build();
    rltk::main_loop(context, gs)
}