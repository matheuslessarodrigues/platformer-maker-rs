#![feature(vec_resize_default)]

// external
pub extern crate sdl2;

pub extern crate serde;
#[macro_use]
pub extern crate serde_derive;
pub extern crate serde_json;

pub extern crate specs;
#[macro_use]
pub extern crate specs_derive;

// local

pub mod core;
pub mod renderer;

pub mod systems;
pub mod entities;
pub mod components;
pub mod assets;

pub mod input;
pub mod math;

pub mod sdl;
