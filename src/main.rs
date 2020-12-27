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
            VirtualKeyCode::Up => println!("Up"),
            VirtualKeyCode::Down => println!("Down"),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        let gamedata = self.ecs.fetch::<data::Gamedata>();
        let mut hex = String::new();
        for row in 0..hexes::ROWS {
            for col in 0..hexes::COLS {
                hex = format!("{:x}", gamedata.hexes.get(row, col).unwrap());
                ctx.print_color(2 + (col * 10), 2 + row, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.1, 0., 0.), hex);
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    //let context = RltkBuilder::simple80x50()
    let context = RltkBuilder::vga(42, 42)
        //.unwrap_or_default() // the simple above won't run without this
        //.with_font("vga8x8.jpg", 8, 16)
        //.with_automatic_console_resize(true)
        .with_font("vga8x16.png", 8, 16)
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.insert(init_gamedata());
    rltk::main_loop(context, gs)
}