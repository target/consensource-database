//allow compile warning that is caused by diesel and can only be fixed on diesel
#![allow(unknown_lints)]
#![allow(proc_macro_derive_resolution_fallback)]

pub mod connection_pool;
pub mod custom_types;
pub mod data_manager;
pub mod errors;
pub mod models;
pub mod tables_schema;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate diesel_full_text_search;
