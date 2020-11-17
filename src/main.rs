extern crate rltk;
extern crate specs; 
extern crate specs_derive;  // Apparently this is not needed for the "new" versions of Rust?
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
mod hexes;
mod data;

struct State {
    ecs: World
}

#[derive(Component)]
struct Gamedata {
    hexes:  array2d::Array2D<u32>,
    level:  u32,
    score:  u32
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        let game = self.ecs.read_storage::<Gamedata>();
        for row in 0..hexes::ROWS {
            for col in 0..hexes::COLS {
                //ctx.print(2 + (col * 10), 2 + row, .hexes[row][col]);
                ctx.print(2 + (col * 10), 2 + row, game.hexes[row][col]);
            }
        }
        /*
        ctx.print(2, 2, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 4, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 6, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 8, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 10, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 12, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 14, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 16, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 18, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 20, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 22, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 24, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 26, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 28, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 30, "0x001f44  0x11a030  0x5020aa  0x002f44");
        ctx.print(2, 32, "0x001f44  0x11a030  0x5020aa  0x11b030");
        ctx.print(2, 36, "Lvl 0001  Tm  0001  Mvs 0000  Scr 0000");
        */
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    //let context = RltkBuilder::simple80x50()
    let context = RltkBuilder::simple(42, 42)
        .unwrap_or_default() // the simple above won't run without this
        .with_title("ethics-gradient")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Gamedata>();
    gs.ecs
        .create_entity()
        .with(Gamedata {
            hexes: hexes::generate_hex(1),
            level: 1,
            score: 0
        })
        .build();
    rltk::main_loop(context, gs)
}