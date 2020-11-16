extern crate array2d;
use array2d::Array2D;

pub fn generate_hex(n: u32) -> Array2D {
    //const X: usize = 4;
    //const Y: usize = 16;

    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let from_rows = Array2D::from_rows(&rows);
    from_rows
}