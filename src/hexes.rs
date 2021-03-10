extern crate array2d;
extern crate rand;
use self::array2d::Array2D;
use self::rand::Rng;

pub const COLS: usize = 4;
pub const ROWS: usize = 16;
//pub const UP:   usize = 0;
//pub const DOWN: usize = 1;

pub fn generate_hex(_n: u32, lvl: u32) -> Array2D<u32> {
    let mut hexes = Array2D::filled_with(2, 16, 4);
	let mut numbers: Vec<u32> = vec![];
    //let mut hexes = Array2D::from_rows(&rows);
    let mut rng = rand::thread_rng();
    //let mut nbr = 0;
	for i in 0..lvl {
		numbers.push(rng.gen_range(0, 2147483647));
	}

    for row in 0..ROWS {
        for col in 0..COLS {
            hexes.set(row, col, numbers[rng.gen_range(0, lvl) as usize] + col as u32).ok(); // What exactly does ok() do besides get rid of unused warning?
            //println!("Row is {}, Col is {}, value is {}", row, col, hexes[(row, col)]);
        }
    }
    hexes
}