use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Gamedata {
    pub hexes:  array2d::Array2D<u32>,
    pub level:  u32,
    pub score:  u32,
    pub time:   u32,
}