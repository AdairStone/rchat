#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;

mod controller;
mod domain;
mod extension;
mod logic;
mod middleware;
mod model;
mod router;
mod schedule;
mod service;
mod wsserver;
mod utils;
mod dto;
mod app_config;

use zino::prelude::*;
use zino_core::application::ServerTag;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let app = zino::Cluster::boot()
        .register_debug(router::debug_routes())
        .register_with(ServerTag::Main, router::main_routes())
        .spawn(schedule::job_scheduler());    
    app.run_with(schedule::async_job_scheduler());
    
    Ok(())
}
