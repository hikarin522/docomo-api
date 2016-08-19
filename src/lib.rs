
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod docomo_api;
