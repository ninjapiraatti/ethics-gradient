extern crate rltk; // Apparently this is not needed for the "new" versions of Rust?
use rltk::{Rltk, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(3, 3, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    //let context = RltkBuilder::simple80x50()
    let context = RltkBuilder::simple(40, 80)
        .unwrap_or_default()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}