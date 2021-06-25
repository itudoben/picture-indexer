// Copyright 2021 Developers of the Web Scraping project.
// Copyright 2021 The Web Scraping Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utilities for pictures indexing
//!
//! Web Scraping provides utilities to web scrape URL, to cache the pages locally, to process
//! them into structured data and to store the structured data.
//!
//! # Quick Start
//!

use std::env;
use std::process;

// This lib is described in the Cargo.toml
use picturelib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Analysing picture from {}", config.picture_path);

    if let Err(e) = picturelib::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
