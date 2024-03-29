#![feature(core_intrinsics)]

// #[macro_use]
// extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate failure;

extern crate validator;

pub mod actors;
pub mod controllers;
pub mod db;
pub mod models;
pub mod requests;
pub mod schema;
pub mod web;
