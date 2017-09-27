//! # Lycaon Registry
//!
//! The registry is aimed to fix the issues with the current registry
//! options that are currently available
//!
//! There are many features available:
//!
//! - Ability to delete images
//! - replication and masterless
//! - other stuff...

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate ctrlc;
extern crate fern;
#[macro_use(log, info, debug)]
extern crate log;
// use of slog is currently not supported
// https://github.com/SergioBenitez/Rocket/issues/21
extern crate ring;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod errors;
mod routes;
pub mod response;
pub mod config;

// use slog::Drain;
use rocket::fairing;

fn main() {
    let _log = config::main_logger().apply();

    rocket::ignite()
        .attach(fairing::AdHoc::on_attach(config::startup))
        .mount("/", routes::routes())
        .catch(routes::errors())
        .launch();
}