extern crate specs; 
extern crate specs_derive;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Gamedata {
    pub hexes:  array2d::Array2D<u32>,
    pub level:  u32,
    pub score:  u32
}