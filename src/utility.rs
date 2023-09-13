use std::{path::Path, process::exit, env};
use crate::strings;

// Welcome to the file that holds *even* terrible code!

pub fn check(appendage: &str) -> bool {
    let mut path = match env::current_dir() {
        Ok(a)=>a, Err(e) => {
            eprintln!("{} :: {}", strings::fetch("errors.pathRetrivialFail"), e); exit(-1);
    }}; path.push(Path::new(appendage));
path.exists() }