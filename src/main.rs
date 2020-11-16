extern crate rltk; // Apparently this is not needed for the "new" versions of Rust?
mod generate_hex;
use rltk::{Rltk, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        let hexes = generate_hex::generate_hex(1);
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
        println!("{:?}", hexes)
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    //let context = RltkBuilder::simple80x50()
    let context = RltkBuilder::simple(42, 42)
        .unwrap_or_default() // the simple above won't run without this
        .with_title("ethics-gradient")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}